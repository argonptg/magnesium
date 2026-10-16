// F1 25 UDP — formato 2025 — lap_positions.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// PacketLapPositionsData — 1131 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketLapPositionsData {
    pub header: F1Header,                         // 29 bytes
    pub num_laps: u8,                             // 1 byte
    pub lap_start: u8,                            // 1 byte
    pub position_for_vehicle_idx: [[u8; 22]; 50], // 1100 bytes
}

const _: [(); 1131] = [(); core::mem::size_of::<PacketLapPositionsData>()];
