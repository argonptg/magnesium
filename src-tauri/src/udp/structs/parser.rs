// Leitura de datagramas UDP sem referências desalinhadas.
// Pressupõe arquitetura little-endian, conforme o protocolo F1 25.
use super::car_damage::PacketCarDamageData;
use super::car_setup::PacketCarSetupData;
use super::car_status::PacketCarStatusData;
use super::car_telemetry::PacketCarTelemetryData;
use super::final_classification::PacketFinalClassificationData;
use super::header::{F1Header, PacketId, HEADER_SIZE};
use super::lap::PacketLapData;
use super::lap_positions::PacketLapPositionsData;
use super::lobby_info::PacketLobbyInfoData;
use super::motion::PacketMotionData;
use super::motion_ex::PacketMotionExData;
use super::participants::PacketParticipantsData;
use super::session::PacketSessionData;
use super::session_history::PacketSessionHistoryData;
use super::time_trial::PacketTimeTrialData;
use super::tyre_sets::PacketTyreSetsData;
use core::mem::size_of;

/// Tipos que aceitam qualquer padrão de bits e não possuem padding no wire format.
/// Não implemente para bool, enum, ponteiros ou referências.
///
/// # Safety
/// Os tipos precisam ser `Copy`, sem padding e com todos os padrões de bits válidos.
pub unsafe trait WirePod: Copy {}
unsafe impl WirePod for super::header::F1Header {}
unsafe impl WirePod for super::motion::CarMotionData {}
unsafe impl WirePod for super::motion::PacketMotionData {}
unsafe impl WirePod for super::session::MarshalZone {}
unsafe impl WirePod for super::session::WeatherForecastSample {}
unsafe impl WirePod for super::session::PacketSessionData {}
unsafe impl WirePod for super::lap::LapData {}
unsafe impl WirePod for super::lap::PacketLapData {}
unsafe impl WirePod for super::event::PacketEventData {}
unsafe impl WirePod for super::event::FastestLapEvent {}
unsafe impl WirePod for super::event::RetirementEvent {}
unsafe impl WirePod for super::event::DRSDisabledEvent {}
unsafe impl WirePod for super::event::TeamMateInPitsEvent {}
unsafe impl WirePod for super::event::RaceWinnerEvent {}
unsafe impl WirePod for super::event::PenaltyEvent {}
unsafe impl WirePod for super::event::SpeedTrapEvent {}
unsafe impl WirePod for super::event::StartLightsEvent {}
unsafe impl WirePod for super::event::DriveThroughPenaltyServedEvent {}
unsafe impl WirePod for super::event::StopGoPenaltyServedEvent {}
unsafe impl WirePod for super::event::FlashbackEvent {}
unsafe impl WirePod for super::event::ButtonsEvent {}
unsafe impl WirePod for super::event::OvertakeEvent {}
unsafe impl WirePod for super::event::SafetyCarEvent {}
unsafe impl WirePod for super::event::CollisionEvent {}
unsafe impl WirePod for super::participants::LiveryColour {}
unsafe impl WirePod for super::participants::ParticipantData {}
unsafe impl WirePod for super::participants::PacketParticipantsData {}
unsafe impl WirePod for super::car_setup::CarSetupData {}
unsafe impl WirePod for super::car_setup::PacketCarSetupData {}
unsafe impl WirePod for super::car_telemetry::CarTelemetryData {}
unsafe impl WirePod for super::car_telemetry::PacketCarTelemetryData {}
unsafe impl WirePod for super::car_status::CarStatusData {}
unsafe impl WirePod for super::car_status::PacketCarStatusData {}
unsafe impl WirePod for super::final_classification::FinalClassificationData {}
unsafe impl WirePod for super::final_classification::PacketFinalClassificationData {}
unsafe impl WirePod for super::lobby_info::LobbyInfoData {}
unsafe impl WirePod for super::lobby_info::PacketLobbyInfoData {}
unsafe impl WirePod for super::car_damage::CarDamageData {}
unsafe impl WirePod for super::car_damage::PacketCarDamageData {}
unsafe impl WirePod for super::session_history::LapHistoryData {}
unsafe impl WirePod for super::session_history::TyreStintHistoryData {}
unsafe impl WirePod for super::session_history::PacketSessionHistoryData {}
unsafe impl WirePod for super::tyre_sets::TyreSetData {}
unsafe impl WirePod for super::tyre_sets::PacketTyreSetsData {}
unsafe impl WirePod for super::motion_ex::PacketMotionExData {}
unsafe impl WirePod for super::time_trial::TimeTrialDataSet {}
unsafe impl WirePod for super::time_trial::PacketTimeTrialData {}
unsafe impl WirePod for super::lap_positions::PacketLapPositionsData {}

/// Copia os bytes recebidos para uma struct packed, sem criar referências desalinhadas.
/// O tamanho deve ser EXATAMENTE igual ao do tipo requisitado.
pub fn read_packed<T: WirePod>(data: &[u8]) -> Option<T> {
    if data.len() != size_of::<T>() {
        return None;
    }
    // SAFETY: `WirePod` garante tipos sem bit patterns inválidos; read_unaligned
    // não exige alinhamento do ponteiro. A referência só existe durante a cópia.
    Some(unsafe { core::ptr::read_unaligned(data.as_ptr().cast::<T>()) })
}

/// Lê apenas os primeiros 29 bytes de um datagrama UDP.
pub fn read_header(data: &[u8]) -> Option<F1Header> {
    read_packed::<F1Header>(data.get(..HEADER_SIZE)?)
}

/// Tamanho esperado do payload UDP por tipo.
pub fn expected_packet_size(id: PacketId) -> Option<usize> {
    match id {
        PacketId::MotionData => Some(size_of::<PacketMotionData>()),
        PacketId::SessionData => Some(size_of::<PacketSessionData>()),
        PacketId::LapData => Some(size_of::<PacketLapData>()),
        PacketId::EventData => Some(size_of::<super::event::PacketEventData>()),
        PacketId::ParticipantsData => Some(size_of::<PacketParticipantsData>()),
        PacketId::CarSetupData => Some(size_of::<PacketCarSetupData>()),
        PacketId::CarTelemetryData => Some(size_of::<PacketCarTelemetryData>()),
        PacketId::CarStatusData => Some(size_of::<PacketCarStatusData>()),
        PacketId::FinalClassificationData => Some(size_of::<PacketFinalClassificationData>()),
        PacketId::LobbyInfoData => Some(size_of::<PacketLobbyInfoData>()),
        PacketId::CarDamageData => Some(size_of::<PacketCarDamageData>()),
        PacketId::SessionHistoryData => Some(size_of::<PacketSessionHistoryData>()),
        PacketId::TyreSetsData => Some(size_of::<PacketTyreSetsData>()),
        PacketId::MotionExData => Some(size_of::<PacketMotionExData>()),
        PacketId::TimeTrialData => Some(size_of::<PacketTimeTrialData>()),
        PacketId::LapPositionsData => Some(size_of::<PacketLapPositionsData>()),
    }
}
