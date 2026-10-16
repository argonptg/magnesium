// F1 25 UDP — formato 2025 — car_telemetry.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// CarTelemetryData — 60 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CarTelemetryData {
    pub speed: u16,                         // 2 bytes
    pub throttle: f32,                      // 4 bytes
    pub steer: f32,                         // 4 bytes
    pub brake: f32,                         // 4 bytes
    pub clutch: u8,                         // 1 byte
    pub gear: i8,                           // 1 byte
    pub engine_rpm: u16,                    // 2 bytes
    pub drs: u8,                            // 1 byte
    pub rev_lights_percent: u8,             // 1 byte
    pub rev_lights_bit_value: u16,          // 2 bytes
    pub brakes_temperature: [u16; 4],       // 8 bytes
    pub tyres_surface_temperature: [u8; 4], // 4 bytes
    pub tyres_inner_temperature: [u8; 4],   // 4 bytes
    pub engine_temperature: u16,            // 2 bytes
    pub tyres_pressure: [f32; 4],           // 16 bytes
    pub surface_type: [u8; 4],              // 4 bytes
}

/// PacketCarTelemetryData — 1352 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketCarTelemetryData {
    pub header: F1Header,                           // 29 bytes
    pub car_telemetry_data: [CarTelemetryData; 22], // 1320 bytes
    pub mfd_panel_index: u8,                        // 1 byte
    pub mfd_panel_index_secondary_player: u8,       // 1 byte
    pub suggested_gear: i8,                         // 1 byte
}

const _: [(); 60] = [(); core::mem::size_of::<CarTelemetryData>()];
const _: [(); 1352] = [(); core::mem::size_of::<PacketCarTelemetryData>()];
