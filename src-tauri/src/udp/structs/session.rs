// F1 25 UDP — formato 2025 — session.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// MarshalZone — 5 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MarshalZone {
    pub zone_start: f32, // 4 bytes
    pub zone_flag: i8,   // 1 byte
}

/// WeatherForecastSample — 8 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct WeatherForecastSample {
    pub session_type: u8,             // 1 byte
    pub time_offset: u8,              // 1 byte
    pub weather: u8,                  // 1 byte
    pub track_temperature: i8,        // 1 byte
    pub track_temperature_change: i8, // 1 byte
    pub air_temperature: i8,          // 1 byte
    pub air_temperature_change: i8,   // 1 byte
    pub rain_percentage: u8,          // 1 byte
}

/// PacketSessionData — 753 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketSessionData {
    pub header: F1Header,                                      // 29 bytes
    pub weather: u8,                                           // 1 byte
    pub track_temperature: i8,                                 // 1 byte
    pub air_temperature: i8,                                   // 1 byte
    pub total_laps: u8,                                        // 1 byte
    pub track_length: u16,                                     // 2 bytes
    pub session_type: u8,                                      // 1 byte
    pub track_id: i8,                                          // 1 byte
    pub formula: u8,                                           // 1 byte
    pub session_time_left: u16,                                // 2 bytes
    pub session_duration: u16,                                 // 2 bytes
    pub pit_speed_limit: u8,                                   // 1 byte
    pub game_paused: u8,                                       // 1 byte
    pub is_spectating: u8,                                     // 1 byte
    pub spectator_car_index: u8,                               // 1 byte
    pub sli_pro_native_support: u8,                            // 1 byte
    pub num_marshal_zones: u8,                                 // 1 byte
    pub marshal_zones: [MarshalZone; 21],                      // 105 bytes
    pub safety_car_status: u8,                                 // 1 byte
    pub network_game: u8,                                      // 1 byte
    pub num_weather_forecast_samples: u8,                      // 1 byte
    pub weather_forecast_samples: [WeatherForecastSample; 64], // 512 bytes
    pub forecast_accuracy: u8,                                 // 1 byte
    pub ai_difficulty: u8,                                     // 1 byte
    pub season_link_identifier: u32,                           // 4 bytes
    pub weekend_link_identifier: u32,                          // 4 bytes
    pub session_link_identifier: u32,                          // 4 bytes
    pub pit_stop_window_ideal_lap: u8,                         // 1 byte
    pub pit_stop_window_latest_lap: u8,                        // 1 byte
    pub pit_stop_rejoin_position: u8,                          // 1 byte
    pub steering_assist: u8,                                   // 1 byte
    pub braking_assist: u8,                                    // 1 byte
    pub gearbox_assist: u8,                                    // 1 byte
    pub pit_assist: u8,                                        // 1 byte
    pub pit_release_assist: u8,                                // 1 byte
    pub ers_assist: u8,                                        // 1 byte
    pub drs_assist: u8,                                        // 1 byte
    pub dynamic_racing_line: u8,                               // 1 byte
    pub dynamic_racing_line_type: u8,                          // 1 byte
    pub game_mode: u8,                                         // 1 byte
    pub rule_set: u8,                                          // 1 byte
    pub time_of_day: u32,                                      // 4 bytes
    pub session_length: u8,                                    // 1 byte
    pub speed_units_lead_player: u8,                           // 1 byte
    pub temperature_units_lead_player: u8,                     // 1 byte
    pub speed_units_secondary_player: u8,                      // 1 byte
    pub temperature_units_secondary_player: u8,                // 1 byte
    pub num_safety_car_periods: u8,                            // 1 byte
    pub num_virtual_safety_car_periods: u8,                    // 1 byte
    pub num_red_flag_periods: u8,                              // 1 byte
    pub equal_car_performance: u8,                             // 1 byte
    pub recovery_mode: u8,                                     // 1 byte
    pub flashback_limit: u8,                                   // 1 byte
    pub surface_type: u8,                                      // 1 byte
    pub low_fuel_mode: u8,                                     // 1 byte
    pub race_starts: u8,                                       // 1 byte
    pub tyre_temperature: u8,                                  // 1 byte
    pub pit_lane_tyre_sim: u8,                                 // 1 byte
    pub car_damage: u8,                                        // 1 byte
    pub car_damage_rate: u8,                                   // 1 byte
    pub collisions: u8,                                        // 1 byte
    pub collisions_off_for_first_lap_only: u8,                 // 1 byte
    pub mp_unsafe_pit_release: u8,                             // 1 byte
    pub mp_off_for_griefing: u8,                               // 1 byte
    pub corner_cutting_stringency: u8,                         // 1 byte
    pub parc_ferme_rules: u8,                                  // 1 byte
    pub pit_stop_experience: u8,                               // 1 byte
    pub safety_car: u8,                                        // 1 byte
    pub safety_car_experience: u8,                             // 1 byte
    pub formation_lap: u8,                                     // 1 byte
    pub formation_lap_experience: u8,                          // 1 byte
    pub red_flags: u8,                                         // 1 byte
    pub affects_licence_level_solo: u8,                        // 1 byte
    pub affects_licence_level_mp: u8,                          // 1 byte
    pub num_sessions_in_weekend: u8,                           // 1 byte
    pub weekend_structure: [u8; 12],                           // 12 bytes
    pub sector2_lap_distance_start: f32,                       // 4 bytes
    pub sector3_lap_distance_start: f32,                       // 4 bytes
}

const _: [(); 5] = [(); core::mem::size_of::<MarshalZone>()];
const _: [(); 8] = [(); core::mem::size_of::<WeatherForecastSample>()];
const _: [(); 753] = [(); core::mem::size_of::<PacketSessionData>()];
