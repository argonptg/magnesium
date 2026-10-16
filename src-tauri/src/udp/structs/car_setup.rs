// F1 25 UDP — formato 2025 — car_setup.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// CarSetupData — 50 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CarSetupData {
    pub front_wing: u8,                 // 1 byte
    pub rear_wing: u8,                  // 1 byte
    pub on_throttle: u8,                // 1 byte
    pub off_throttle: u8,               // 1 byte
    pub front_camber: f32,              // 4 bytes
    pub rear_camber: f32,               // 4 bytes
    pub front_toe: f32,                 // 4 bytes
    pub rear_toe: f32,                  // 4 bytes
    pub front_suspension: u8,           // 1 byte
    pub rear_suspension: u8,            // 1 byte
    pub front_anti_roll_bar: u8,        // 1 byte
    pub rear_anti_roll_bar: u8,         // 1 byte
    pub front_suspension_height: u8,    // 1 byte
    pub rear_suspension_height: u8,     // 1 byte
    pub brake_pressure: u8,             // 1 byte
    pub brake_bias: u8,                 // 1 byte
    pub engine_braking: u8,             // 1 byte
    pub rear_left_tyre_pressure: f32,   // 4 bytes
    pub rear_right_tyre_pressure: f32,  // 4 bytes
    pub front_left_tyre_pressure: f32,  // 4 bytes
    pub front_right_tyre_pressure: f32, // 4 bytes
    pub ballast: u8,                    // 1 byte
    pub fuel_load: f32,                 // 4 bytes
}

/// PacketCarSetupData — 1133 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketCarSetupData {
    pub header: F1Header,                   // 29 bytes
    pub car_setup_data: [CarSetupData; 22], // 1100 bytes
    pub next_front_wing_value: f32,         // 4 bytes
}

const _: [(); 50] = [(); core::mem::size_of::<CarSetupData>()];
const _: [(); 1133] = [(); core::mem::size_of::<PacketCarSetupData>()];
