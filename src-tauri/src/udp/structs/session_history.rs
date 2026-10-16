// F1 25 UDP — formato 2025 — session_history.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// LapHistoryData — 14 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct LapHistoryData {
    pub lap_time_in_ms: u32,           // 4 bytes
    pub sector1_time_ms_part: u16,     // 2 bytes
    pub sector1_time_minutes_part: u8, // 1 byte
    pub sector2_time_ms_part: u16,     // 2 bytes
    pub sector2_time_minutes_part: u8, // 1 byte
    pub sector3_time_ms_part: u16,     // 2 bytes
    pub sector3_time_minutes_part: u8, // 1 byte
    pub lap_valid_bit_flags: u8,       // 1 byte
}

/// TyreStintHistoryData — 3 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TyreStintHistoryData {
    pub end_lap: u8,              // 1 byte
    pub tyre_actual_compound: u8, // 1 byte
    pub tyre_visual_compound: u8, // 1 byte
}

/// PacketSessionHistoryData — 1460 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketSessionHistoryData {
    pub header: F1Header,                                    // 29 bytes
    pub car_idx: u8,                                         // 1 byte
    pub num_laps: u8,                                        // 1 byte
    pub num_tyre_stints: u8,                                 // 1 byte
    pub best_lap_time_lap_num: u8,                           // 1 byte
    pub best_sector1_lap_num: u8,                            // 1 byte
    pub best_sector2_lap_num: u8,                            // 1 byte
    pub best_sector3_lap_num: u8,                            // 1 byte
    pub lap_history_data: [LapHistoryData; 100],             // 1400 bytes
    pub tyre_stints_history_data: [TyreStintHistoryData; 8], // 24 bytes
}

const _: [(); 14] = [(); core::mem::size_of::<LapHistoryData>()];
const _: [(); 3] = [(); core::mem::size_of::<TyreStintHistoryData>()];
const _: [(); 1460] = [(); core::mem::size_of::<PacketSessionHistoryData>()];
