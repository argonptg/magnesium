type TrackDetails = {
    name: string;
    track: string;
    track_name: string;
};

// im not doing allat 😂😂😂
export interface PacketSessionData {
  header: F1Header;
  weather: number;
  track_temperature: number;
  air_temperature: number;
  total_laps: number;
  track_length: number;
  session_type: number;
  track_id: number;
  formula: number;
  session_time_left: number;
  session_duration: number;
  pit_speed_limit: number;
  game_paused: number;
  is_spectating: number;
  spectator_car_index: number;
  sli_pro_native_support: number;
  num_marshal_zones: number;
  marshal_zones: MarshalZone[]; // Standard array
  safety_car_status: number;
  network_game: number;
  num_weather_forecast_samples: number;
  weather_forecast_samples: WeatherForecastSample[]; // Standard array
  forecast_accuracy: number;
  ai_difficulty: number;
  season_link_identifier: number;
  weekend_link_identifier: number;
  session_link_identifier: number;
  pit_stop_window_ideal_lap: number;
  pit_stop_window_latest_lap: number;
  pit_stop_rejoin_position: number;
  steering_assist: number;
  braking_assist: number;
  gearbox_assist: number;
  pit_assist: number;
  pit_release_assist: number;
  ers_assist: number;
  drs_assist: number;
  dynamic_racing_line: number;
  dynamic_racing_line_type: number;
  game_mode: number;
  rule_set: number;
  time_of_day: number;
  session_length: number;
  speed_units_lead_player: number;
  temperature_units_lead_player: number;
  speed_units_secondary_player: number;
  temperature_units_secondary_player: number;
  num_safety_car_periods: number;
  num_virtual_safety_car_periods: number;
  num_red_flag_periods: number;
  equal_car_performance: number;
  recovery_mode: number;
  flashback_limit: number;
  surface_type: number;
  low_fuel_mode: number;
  race_starts: number;
  tyre_temperature: number;
  pit_lane_tyre_sim: number;
  car_damage: number;
  car_damage_rate: number;
  collisions: number;
  collisions_off_for_first_lap_only: number;
  mp_unsafe_pit_release: number;
  mp_off_for_griefing: number;
  corner_cutting_stringency: number;
  parc_ferme_rules: number;
  pit_stop_experience: number;
  safety_car: number;
  safety_car_experience: number;
  formation_lap: number;
  formation_lap_experience: number;
  red_flags: number;
  affects_licence_level_solo: number;
  affects_licence_level_mp: number;
  num_sessions_in_weekend: number;
  weekend_structure: number[]; // Standard array
  sector2_lap_distance_start: number;
  sector3_lap_distance_start: number;
}

interface MarshalZone {
  zone_start: number;
  zone_flag: number;
}

interface WeatherForecastSample {
  session_type: number;
  time_offset: number;
  weather: number;
  track_temperature: number;
  track_temperature_change: number;
  air_temperature: number;
  air_temperature_change: number;
  rain_percentage: number;
}