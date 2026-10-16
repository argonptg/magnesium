// F1 25 UDP — formato 2025 — time_trial.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// TimeTrialDataSet — 24 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TimeTrialDataSet {
    pub car_idx: u8, // 1 byte
    pub team_id: u8, // 1 byte
    pub lap_time_in_ms: u32, // 4 bytes
    pub sector1_time_in_ms: u32, // 4 bytes
    pub sector2_time_in_ms: u32, // 4 bytes
    pub sector3_time_in_ms: u32, // 4 bytes
    pub traction_control: u8, // 1 byte
    pub gearbox_assist: u8, // 1 byte
    pub anti_lock_brakes: u8, // 1 byte
    pub equal_car_performance: u8, // 1 byte
    pub custom_setup: u8, // 1 byte
    pub valid: u8, // 1 byte
}

/// PacketTimeTrialData — 101 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketTimeTrialData {
    pub header: F1Header, // 29 bytes
    pub player_session_best_data_set: TimeTrialDataSet, // 24 bytes
    pub personal_best_data_set: TimeTrialDataSet, // 24 bytes
    pub rival_data_set: TimeTrialDataSet, // 24 bytes
}

const _: [(); 24] = [(); core::mem::size_of::<TimeTrialDataSet>()];
const _: [(); 101] = [(); core::mem::size_of::<PacketTimeTrialData>()];
