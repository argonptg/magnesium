// F1 25 UDP — formato 2025 — tyre_sets.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// TyreSetData — 10 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TyreSetData {
    pub actual_tyre_compound: u8, // 1 byte
    pub visual_tyre_compound: u8, // 1 byte
    pub wear: u8,                 // 1 byte
    pub available: u8,            // 1 byte
    pub recommended_session: u8,  // 1 byte
    pub life_span: u8,            // 1 byte
    pub usable_life: u8,          // 1 byte
    pub lap_delta_time: i16,      // 2 bytes
    pub fitted: u8,               // 1 byte
}

/// PacketTyreSetsData — 231 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketTyreSetsData {
    pub header: F1Header,                 // 29 bytes
    pub car_idx: u8,                      // 1 byte
    pub tyre_set_data: [TyreSetData; 20], // 200 bytes
    pub fitted_idx: u8,                   // 1 byte
}

const _: [(); 10] = [(); core::mem::size_of::<TyreSetData>()];
const _: [(); 231] = [(); core::mem::size_of::<PacketTyreSetsData>()];
