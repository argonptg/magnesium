/*
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

    pub header: F1Header,             // 29 bytes
    pub lap_data: [LapData; 22],      // 1254 bytes
    pub time_trial_pb_car_idx: u8,    // 1 byte
    pub time_trial_rival_car_idx: u8, // 1 byte
 */

import type { F1Header } from "./header";

export interface PacketLapData {
    header: F1Header;
    lap_data: LapData[];
    time_trial_pb_car_idx: number;
    time_trial_rival_car_idx: number;
}

export interface LapData {
    last_lap_time_in_ms: number;
    current_lap_time_in_ms: number;
    sector1_time_ms_part: number;
    sector1_time_minutes_part: number;
    sector2_time_ms_part: number;
    sector2_time_minutes_part: number;
    delta_to_car_in_front_ms_part: number;
    delta_to_car_in_front_minutes_part: number;
    delta_to_race_leader_ms_part: number;
    delta_to_race_leader_minutes_part: number;
    lap_distance: number;
    total_distance: number;
    safety_car_delta: number;
    car_position: number;
    current_lap_num: number;
    pit_status: number;
    num_pit_stops: number;
    sector: number;
    current_lap_invalid: number;
    penalties: number;
    total_warnings: number;
    corner_cutting_warnings: number;
    num_unserved_drive_through_pens: number;
    num_unserved_stop_go_pens: number;
    grid_position: number;
    driver_status: number;
    result_status: number;
    pit_lane_timer_active: number;
    pit_lane_time_in_lane_in_ms: number;
    pit_stop_timer_in_ms: number;
    pit_stop_should_serve_pen: number;
    speed_trap_fastest_speed: number;
    speed_trap_fastest_lap: number;
}