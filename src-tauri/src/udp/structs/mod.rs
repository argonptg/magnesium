// Todos os tipos ficam disponíveis como `structs::PacketCarStatusData`, etc.

pub mod car_damage;
pub mod car_setup;
pub mod car_status;
pub mod car_telemetry;
pub mod event;
pub mod final_classification;
pub mod header;
pub mod lap;
pub mod lap_positions;
pub mod lobby_info;
pub mod motion;
pub mod motion_ex;
pub mod parser;
pub mod participants;
mod serialization;
pub mod session;
pub mod session_history;
pub mod time_trial;
pub mod tyre_sets;

/// Packet decodificado que pode ser emitido ao frontend via Tauri.
/// A tag é o nome público do evento; não a renomeie sem atualizar o frontend.
pub trait FrontendPacket: serde::Serialize {
    const EMITTER_TAG: &'static str;
}
