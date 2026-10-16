// F1 25 UDP — formato 2025 — car_status.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// CarStatusData — 55 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CarStatusData {
    pub traction_control: u8, // 1 byte
    pub anti_lock_brakes: u8, // 1 byte
    pub fuel_mix: u8, // 1 byte
    pub front_brake_bias: u8, // 1 byte
    pub pit_limiter_status: u8, // 1 byte
    pub fuel_in_tank: f32, // 4 bytes
    pub fuel_capacity: f32, // 4 bytes
    pub fuel_remaining_laps: f32, // 4 bytes
    pub max_rpm: u16, // 2 bytes
    pub idle_rpm: u16, // 2 bytes
    pub max_gears: u8, // 1 byte
    pub drs_allowed: u8, // 1 byte
    pub drs_activation_distance: u16, // 2 bytes
    pub actual_tyre_compound: u8, // 1 byte
    pub visual_tyre_compound: u8, // 1 byte
    pub tyres_age_laps: u8, // 1 byte
    pub vehicle_fia_flags: i8, // 1 byte
    pub engine_power_ice: f32, // 4 bytes
    pub engine_power_mguk: f32, // 4 bytes
    pub ers_store_energy: f32, // 4 bytes
    pub ers_deploy_mode: u8, // 1 byte
    pub ers_harvested_this_lap_mguk: f32, // 4 bytes
    pub ers_harvested_this_lap_mguh: f32, // 4 bytes
    pub ers_deployed_this_lap: f32, // 4 bytes
    pub network_paused: u8, // 1 byte
}

/// PacketCarStatusData — 1239 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketCarStatusData {
    pub header: F1Header, // 29 bytes
    pub car_status_data: [CarStatusData; 22], // 1210 bytes
}

const _: [(); 55] = [(); core::mem::size_of::<CarStatusData>()];
const _: [(); 1239] = [(); core::mem::size_of::<PacketCarStatusData>()];
