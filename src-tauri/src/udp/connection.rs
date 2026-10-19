use std::sync::atomic::{AtomicBool, Ordering};
use std::net::UdpSocket;
use std::thread;
use tauri::{AppHandle, Emitter};

use crate::udp::structs::FrontendPacket;
use crate::udp::structs::car_damage::PacketCarDamageData;
use crate::udp::structs::car_setup::PacketCarSetupData;
use crate::udp::structs::car_status::PacketCarStatusData;
use crate::udp::structs::car_telemetry::PacketCarTelemetryData;
use crate::udp::structs::event::PacketEventData;
use crate::udp::structs::final_classification::PacketFinalClassificationData;
use crate::udp::structs::header::{HEADER_SIZE, PACKET_FORMAT, PacketId};
use crate::udp::structs::lap::PacketLapData;
use crate::udp::structs::lap_positions::PacketLapPositionsData;
use crate::udp::structs::lobby_info::PacketLobbyInfoData;
use crate::udp::structs::motion::PacketMotionData;
use crate::udp::structs::motion_ex::PacketMotionExData;
use crate::udp::structs::parser::{WirePod, expected_packet_size, read_header, read_packed};
use crate::udp::structs::participants::PacketParticipantsData;
use crate::udp::structs::session::PacketSessionData;
use crate::udp::structs::session_history::PacketSessionHistoryData;
use crate::udp::structs::time_trial::PacketTimeTrialData;
use crate::udp::structs::tyre_sets::PacketTyreSetsData;
use crate::utils::log::*;

static UDP_RUNNING: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub fn try_connect(app: AppHandle, ip: &str, port: u16) {
    if UDP_RUNNING.swap(true, Ordering::AcqRel) {
        log_info("UDP listener is already running.", true);
        return; // esqueci disso kkkskksksksk
    }

    let msg = format!("Connecting to UDP server at {}:{}", ip, port);
    log_info(&msg, true);

    // tá aqui vem a lógica mesmo
    let ip = ip.to_owned(); // move closure precisa de ownership

    thread::spawn(move || {
        let sock = match start_socket(&ip, port) {
            Ok(s) => s,
            Err(e) => {
                log_error(&format!("Failed to start UDP socket: {}", e), true);
                UDP_RUNNING.store(false, Ordering::Release);
                return;
            }
        };

        log_info("UDP socket started successfully.", true);

        // simbora ler header
        let mut buf = [0u8; 4096]; // buffer de 4kb, se passou disso loucura

        let mut receiving = false;
        let mut timeout_count = 0u8;

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, _src)) => {
                    if !receiving {
                        receiving = true;
                        timeout_count = 0;
                        // envia um packet falando q deu bom.
                        app.emit("udp_connected", true).unwrap();
                    }

                    if size < 29 {
                        log_warn(
                            "Packet is too small to contain a header, something is wrong...",
                            true,
                        );
                        continue;
                    }

                    let packet = &buf[..size]; // ler só o que realmente existe
                    let header_buf = &packet[..HEADER_SIZE]; // 29 bytes do header

                    let Some(header) = read_header(header_buf) else {
                        log_warn("Could not decode packet header, skipping packet...", true);
                        continue;
                    };

                    if header.p_car_index >= 22 {
                        log_warn("Invalid player car index, skipping packet...", true);
                        continue;
                    }

                    if header.packet_format != PACKET_FORMAT {
                        log_warn("Unsupported packet format, skipping packet...", true);
                        continue;
                    }

                    let Ok(packet_id) = PacketId::try_from(header.packet_id) else {
                        log_warn("Unknown packet ID, skipping packet...", true);
                        continue;
                    };

                    let Some(expected_size) = expected_packet_size(packet_id) else {
                        log_warn("Packet size is not defined, skipping packet...", true);
                        continue;
                    };

                    if size != expected_size {
                        log_warn(
                            &format!(
                                "Unexpected packet size for ID {}: received {}, expected {}.",
                                header.packet_id, size, expected_size
                            ),
                            true,
                        );
                        continue;
                    }

                    timeout_count = 0;

                    if !receiving {
                        receiving = true;
                        let _ = app.emit("udp_connected", true);
                        log_info("Recieving UDP telemetry.", true);
                    }

                    //let msg = format!("Packet ID = {}", header.packet_id);
                    //log_info(&msg, true);

                    dispatch_packet(&app, packet_id, packet); // faz a resenha
                }
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                    ) =>
                {
                    if receiving {
                        receiving = false;
                        let _ = app.emit("udp_timeout", true);
                    }

                    match timeout_count {
                        0..=3 => log_warn("UDP socket timed out.", true),
                        4 => log_warn("UDP timed out 5 times. Suppressing further warnings.", true),
                        _ => {}
                    }

                    timeout_count = timeout_count.saturating_add(1);
                }
                Err(e) => {
                    log_error(&format!("UDP socket error: {e}"), true);
                    break;
                }
            }
        }

        UDP_RUNNING.store(false, Ordering::Release);
    });

    // return "Connected to UDP server!".into();
}

/// Decodifica e emite um packet fixo. `read_packed` valida o tamanho exato antes
/// de fazer a cópia desalinhada encapsulada do datagrama.
fn emit_packet<T>(app: &AppHandle, packet: &[u8])
where
    T: WirePod + FrontendPacket,
{
    let Some(packet) = read_packed::<T>(packet) else {
        log_warn("Could not decode packet, skipping packet...", true);
        return;
    };

    if let Err(error) = app.emit(T::EMITTER_TAG, packet) {
        log_error(
            &format!("Failed to emit {}: {}", T::EMITTER_TAG, error),
            true,
        );
    }
}

// associa o Packet ID validado ao tipo concreto e ao seu emitter tag.
// pensa num cornojob fudido
fn dispatch_packet(app: &AppHandle, packet_id: PacketId, packet: &[u8]) {
    match packet_id {
        PacketId::MotionData => emit_packet::<PacketMotionData>(app, packet),
        PacketId::SessionData => emit_packet::<PacketSessionData>(app, packet),
        PacketId::LapData => emit_packet::<PacketLapData>(app, packet),
        PacketId::EventData => emit_packet::<PacketEventData>(app, packet),
        PacketId::ParticipantsData => emit_packet::<PacketParticipantsData>(app, packet),
        PacketId::CarSetupData => emit_packet::<PacketCarSetupData>(app, packet),
        PacketId::CarTelemetryData => emit_packet::<PacketCarTelemetryData>(app, packet),
        PacketId::CarStatusData => emit_packet::<PacketCarStatusData>(app, packet),
        PacketId::FinalClassificationData => {
            emit_packet::<PacketFinalClassificationData>(app, packet)
        }
        PacketId::LobbyInfoData => emit_packet::<PacketLobbyInfoData>(app, packet),
        PacketId::CarDamageData => emit_packet::<PacketCarDamageData>(app, packet),
        PacketId::SessionHistoryData => emit_packet::<PacketSessionHistoryData>(app, packet),
        PacketId::TyreSetsData => emit_packet::<PacketTyreSetsData>(app, packet),
        PacketId::MotionExData => emit_packet::<PacketMotionExData>(app, packet),
        PacketId::TimeTrialData => emit_packet::<PacketTimeTrialData>(app, packet),
        PacketId::LapPositionsData => emit_packet::<PacketLapPositionsData>(app, packet),
    }
}

fn start_socket(ip: &str, port: u16) -> Result<UdpSocket, std::io::Error> {
    let addr = format!("{}:{}", ip, port);

    let socket = UdpSocket::bind(&addr)?;
    // se demorou mais de 3 segundos, então parou de receber dados, então desconecta
    // de brinde deixa um log
    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(3)))
        ?;

    Ok(socket)
}
