// F1 25 UDP — formato 2025 — lap.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// LapData — 57 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct LapData {
    pub last_lap_time_in_ms: u32,               // 4 bytes
    pub current_lap_time_in_ms: u32,            // 4 bytes
    pub sector1_time_ms_part: u16,              // 2 bytes
    pub sector1_time_minutes_part: u8,          // 1 byte
    pub sector2_time_ms_part: u16,              // 2 bytes
    pub sector2_time_minutes_part: u8,          // 1 byte
    pub delta_to_car_in_front_ms_part: u16,     // 2 bytes
    pub delta_to_car_in_front_minutes_part: u8, // 1 byte
    pub delta_to_race_leader_ms_part: u16,      // 2 bytes
    pub delta_to_race_leader_minutes_part: u8,  // 1 byte
    pub lap_distance: f32,                      // 4 bytes
    pub total_distance: f32,                    // 4 bytes
    pub safety_car_delta: f32,                  // 4 bytes
    pub car_position: u8,                       // 1 byte
    pub current_lap_num: u8,                    // 1 byte
    pub pit_status: u8,                         // 1 byte
    pub num_pit_stops: u8,                      // 1 byte
    pub sector: u8,                             // 1 byte
    pub current_lap_invalid: u8,                // 1 byte
    pub penalties: u8,                          // 1 byte
    pub total_warnings: u8,                     // 1 byte
    pub corner_cutting_warnings: u8,            // 1 byte
    pub num_unserved_drive_through_pens: u8,    // 1 byte
    pub num_unserved_stop_go_pens: u8,          // 1 byte
    pub grid_position: u8,                      // 1 byte
    pub driver_status: u8,                      // 1 byte
    pub result_status: u8,                      // 1 byte
    pub pit_lane_timer_active: u8,              // 1 byte
    pub pit_lane_time_in_lane_in_ms: u16,       // 2 bytes
    pub pit_stop_timer_in_ms: u16,              // 2 bytes
    pub pit_stop_should_serve_pen: u8,          // 1 byte
    pub speed_trap_fastest_speed: f32,          // 4 bytes
    pub speed_trap_fastest_lap: u8,             // 1 byte
}

/// PacketLapData — 1285 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketLapData {
    pub header: F1Header,             // 29 bytes
    pub lap_data: [LapData; 22],      // 1254 bytes
    pub time_trial_pb_car_idx: u8,    // 1 byte
    pub time_trial_rival_car_idx: u8, // 1 byte
}

const _: [(); 57] = [(); core::mem::size_of::<LapData>()];
const _: [(); 1285] = [(); core::mem::size_of::<PacketLapData>()];
