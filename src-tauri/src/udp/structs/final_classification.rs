// F1 25 UDP — formato 2025 — final_classification.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// FinalClassificationData — 46 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FinalClassificationData {
    pub position: u8, // 1 byte
    pub num_laps: u8, // 1 byte
    pub grid_position: u8, // 1 byte
    pub points: u8, // 1 byte
    pub num_pit_stops: u8, // 1 byte
    pub result_status: u8, // 1 byte
    pub result_reason: u8, // 1 byte
    pub best_lap_time_in_ms: u32, // 4 bytes
    pub total_race_time: f64, // 8 bytes
    pub penalties_time: u8, // 1 byte
    pub num_penalties: u8, // 1 byte
    pub num_tyre_stints: u8, // 1 byte
    pub tyre_stints_actual: [u8; 8], // 8 bytes
    pub tyre_stints_visual: [u8; 8], // 8 bytes
    pub tyre_stints_end_laps: [u8; 8], // 8 bytes
}

/// PacketFinalClassificationData — 1042 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketFinalClassificationData {
    pub header: F1Header, // 29 bytes
    pub num_cars: u8, // 1 byte
    pub classification_data: [FinalClassificationData; 22], // 1012 bytes
}

const _: [(); 46] = [(); core::mem::size_of::<FinalClassificationData>()];
const _: [(); 1042] = [(); core::mem::size_of::<PacketFinalClassificationData>()];
