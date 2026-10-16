// Serializa structs `repr(C, packed)` sem criar referências a campos desalinhados.
// Cada campo é copiado para uma variável local antes de ser emprestado ao Serde.

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

macro_rules! impl_packed_serialize {
    ($type:path { $($field:ident),+ $(,)? }) => {
        impl Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_struct(
                    stringify!($type),
                    0 $(+ { let _ = stringify!($field); 1 })+,
                )?;
                $(
                    let $field = self.$field;
                    state.serialize_field(stringify!($field), &$field)?;
                )+
                state.end()
            }
        }
    };
}

macro_rules! impl_packed_serialize_with_slices {
    ($type:path { $($field:ident),* $(,)? }; $($slice:ident),+ $(,)?) => {
        impl Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_struct(
                    stringify!($type),
                    0 $(+ { let _ = stringify!($field); 1 })*
                      $(+ { let _ = stringify!($slice); 1 })+,
                )?;
                $(
                    let $field = self.$field;
                    state.serialize_field(stringify!($field), &$field)?;
                )*
                $(
                    let $slice = self.$slice;
                    state.serialize_field(stringify!($slice), $slice.as_slice())?;
                )+
                state.end()
            }
        }
    };
}

macro_rules! impl_frontend_packet {
    ($type:path, $tag:literal) => {
        impl super::FrontendPacket for $type {
            const EMITTER_TAG: &'static str = $tag;
        }
    };
}

impl_packed_serialize!(super::header::F1Header {
    packet_format,
    game_year,
    major_version,
    minor_version,
    packet_version,
    packet_id,
    session_uid,
    session_time,
    frame_id,
    ov_frame_id,
    p_car_index,
    sec_p_car_index,
});

impl_packed_serialize!(super::motion::CarMotionData {
    world_position_x,
    world_position_y,
    world_position_z,
    world_velocity_x,
    world_velocity_y,
    world_velocity_z,
    world_forward_dir_x,
    world_forward_dir_y,
    world_forward_dir_z,
    world_right_dir_x,
    world_right_dir_y,
    world_right_dir_z,
    g_force_lateral,
    g_force_longitudinal,
    g_force_vertical,
    yaw,
    pitch,
    roll,
});
impl_packed_serialize!(super::motion::PacketMotionData {
    header,
    car_motion_data
});

impl_packed_serialize!(super::session::MarshalZone {
    zone_start,
    zone_flag
});
impl_packed_serialize!(super::session::WeatherForecastSample {
    session_type,
    time_offset,
    weather,
    track_temperature,
    track_temperature_change,
    air_temperature,
    air_temperature_change,
    rain_percentage,
});
impl_packed_serialize_with_slices!(super::session::PacketSessionData {
    header, weather, track_temperature, air_temperature, total_laps, track_length, session_type,
    track_id, formula, session_time_left, session_duration, pit_speed_limit, game_paused,
    is_spectating, spectator_car_index, sli_pro_native_support, num_marshal_zones, marshal_zones,
    safety_car_status, network_game, num_weather_forecast_samples,
    forecast_accuracy, ai_difficulty, season_link_identifier, weekend_link_identifier,
    session_link_identifier, pit_stop_window_ideal_lap, pit_stop_window_latest_lap,
    pit_stop_rejoin_position, steering_assist, braking_assist, gearbox_assist, pit_assist,
    pit_release_assist, ers_assist, drs_assist, dynamic_racing_line, dynamic_racing_line_type,
    game_mode, rule_set, time_of_day, session_length, speed_units_lead_player,
    temperature_units_lead_player, speed_units_secondary_player, temperature_units_secondary_player,
    num_safety_car_periods, num_virtual_safety_car_periods, num_red_flag_periods,
    equal_car_performance, recovery_mode, flashback_limit, surface_type, low_fuel_mode,
    race_starts, tyre_temperature, pit_lane_tyre_sim, car_damage, car_damage_rate, collisions,
    collisions_off_for_first_lap_only, mp_unsafe_pit_release, mp_off_for_griefing,
    corner_cutting_stringency, parc_ferme_rules, pit_stop_experience, safety_car,
    safety_car_experience, formation_lap, formation_lap_experience, red_flags,
    affects_licence_level_solo, affects_licence_level_mp, num_sessions_in_weekend,
    weekend_structure, sector2_lap_distance_start, sector3_lap_distance_start,
}; weather_forecast_samples);

impl_packed_serialize!(super::lap::LapData {
    last_lap_time_in_ms,
    current_lap_time_in_ms,
    sector1_time_ms_part,
    sector1_time_minutes_part,
    sector2_time_ms_part,
    sector2_time_minutes_part,
    delta_to_car_in_front_ms_part,
    delta_to_car_in_front_minutes_part,
    delta_to_race_leader_ms_part,
    delta_to_race_leader_minutes_part,
    lap_distance,
    total_distance,
    safety_car_delta,
    car_position,
    current_lap_num,
    pit_status,
    num_pit_stops,
    sector,
    current_lap_invalid,
    penalties,
    total_warnings,
    corner_cutting_warnings,
    num_unserved_drive_through_pens,
    num_unserved_stop_go_pens,
    grid_position,
    driver_status,
    result_status,
    pit_lane_timer_active,
    pit_lane_time_in_lane_in_ms,
    pit_stop_timer_in_ms,
    pit_stop_should_serve_pen,
    speed_trap_fastest_speed,
    speed_trap_fastest_lap,
});
impl_packed_serialize!(super::lap::PacketLapData {
    header,
    lap_data,
    time_trial_pb_car_idx,
    time_trial_rival_car_idx,
});

impl_packed_serialize!(super::event::PacketEventData {
    header,
    event_string_code,
    event_details
});
impl_packed_serialize!(super::event::FastestLapEvent {
    vehicle_idx,
    lap_time
});
impl_packed_serialize!(super::event::RetirementEvent {
    vehicle_idx,
    reason
});
impl_packed_serialize!(super::event::DRSDisabledEvent { reason });
impl_packed_serialize!(super::event::TeamMateInPitsEvent { vehicle_idx });
impl_packed_serialize!(super::event::RaceWinnerEvent { vehicle_idx });
impl_packed_serialize!(super::event::PenaltyEvent {
    penalty_type,
    infringement_type,
    vehicle_idx,
    other_vehicle_idx,
    time,
    lap_num,
    places_gained,
});
impl_packed_serialize!(super::event::SpeedTrapEvent {
    vehicle_idx,
    speed,
    is_overall_fastest_in_session,
    is_driver_fastest_in_session,
    fastest_vehicle_idx_in_session,
    fastest_speed_in_session,
});
impl_packed_serialize!(super::event::StartLightsEvent { num_lights });
impl_packed_serialize!(super::event::DriveThroughPenaltyServedEvent { vehicle_idx });
impl_packed_serialize!(super::event::StopGoPenaltyServedEvent {
    vehicle_idx,
    stop_time
});
impl_packed_serialize!(super::event::FlashbackEvent {
    flashback_frame_identifier,
    flashback_session_time,
});
impl_packed_serialize!(super::event::ButtonsEvent { button_status });
impl_packed_serialize!(super::event::OvertakeEvent {
    overtaking_vehicle_idx,
    being_overtaken_vehicle_idx,
});
impl_packed_serialize!(super::event::SafetyCarEvent {
    safety_car_type,
    event_type
});
impl_packed_serialize!(super::event::CollisionEvent {
    vehicle1_idx,
    vehicle2_idx
});

impl_packed_serialize!(super::participants::LiveryColour { red, green, blue });
impl_packed_serialize!(super::participants::ParticipantData {
    ai_controlled,
    driver_id,
    network_id,
    team_id,
    my_team,
    race_number,
    nationality,
    name,
    your_telemetry,
    show_online_names,
    tech_level,
    platform,
    num_colours,
    livery_colours,
});
impl_packed_serialize!(super::participants::PacketParticipantsData {
    header,
    num_active_cars,
    participants,
});

impl_packed_serialize!(super::car_setup::CarSetupData {
    front_wing,
    rear_wing,
    on_throttle,
    off_throttle,
    front_camber,
    rear_camber,
    front_toe,
    rear_toe,
    front_suspension,
    rear_suspension,
    front_anti_roll_bar,
    rear_anti_roll_bar,
    front_suspension_height,
    rear_suspension_height,
    brake_pressure,
    brake_bias,
    engine_braking,
    rear_left_tyre_pressure,
    rear_right_tyre_pressure,
    front_left_tyre_pressure,
    front_right_tyre_pressure,
    ballast,
    fuel_load,
});
impl_packed_serialize!(super::car_setup::PacketCarSetupData {
    header,
    car_setup_data,
    next_front_wing_value,
});

impl_packed_serialize!(super::car_telemetry::CarTelemetryData {
    speed,
    throttle,
    steer,
    brake,
    clutch,
    gear,
    engine_rpm,
    drs,
    rev_lights_percent,
    rev_lights_bit_value,
    brakes_temperature,
    tyres_surface_temperature,
    tyres_inner_temperature,
    engine_temperature,
    tyres_pressure,
    surface_type,
});
impl_packed_serialize!(super::car_telemetry::PacketCarTelemetryData {
    header,
    car_telemetry_data,
    mfd_panel_index,
    mfd_panel_index_secondary_player,
    suggested_gear,
});

impl_packed_serialize!(super::car_status::CarStatusData {
    traction_control,
    anti_lock_brakes,
    fuel_mix,
    front_brake_bias,
    pit_limiter_status,
    fuel_in_tank,
    fuel_capacity,
    fuel_remaining_laps,
    max_rpm,
    idle_rpm,
    max_gears,
    drs_allowed,
    drs_activation_distance,
    actual_tyre_compound,
    visual_tyre_compound,
    tyres_age_laps,
    vehicle_fia_flags,
    engine_power_ice,
    engine_power_mguk,
    ers_store_energy,
    ers_deploy_mode,
    ers_harvested_this_lap_mguk,
    ers_harvested_this_lap_mguh,
    ers_deployed_this_lap,
    network_paused,
});
impl_packed_serialize!(super::car_status::PacketCarStatusData {
    header,
    car_status_data
});

impl_packed_serialize!(super::final_classification::FinalClassificationData {
    position,
    num_laps,
    grid_position,
    points,
    num_pit_stops,
    result_status,
    result_reason,
    best_lap_time_in_ms,
    total_race_time,
    penalties_time,
    num_penalties,
    num_tyre_stints,
    tyre_stints_actual,
    tyre_stints_visual,
    tyre_stints_end_laps,
});
impl_packed_serialize!(super::final_classification::PacketFinalClassificationData {
    header,
    num_cars,
    classification_data,
});

impl_packed_serialize!(super::lobby_info::LobbyInfoData {
    ai_controlled,
    team_id,
    nationality,
    platform,
    name,
    car_number,
    your_telemetry,
    show_online_names,
    tech_level,
    ready_status,
});
impl_packed_serialize!(super::lobby_info::PacketLobbyInfoData {
    header,
    num_players,
    lobby_players,
});

impl_packed_serialize!(super::car_damage::CarDamageData {
    tyres_wear,
    tyres_damage,
    brakes_damage,
    tyre_blisters,
    front_left_wing_damage,
    front_right_wing_damage,
    rear_wing_damage,
    floor_damage,
    diffuser_damage,
    sidepod_damage,
    drs_fault,
    ers_fault,
    gear_box_damage,
    engine_damage,
    engine_mguh_wear,
    engine_es_wear,
    engine_ce_wear,
    engine_ice_wear,
    engine_mguk_wear,
    engine_tc_wear,
    engine_blown,
    engine_seized,
});
impl_packed_serialize!(super::car_damage::PacketCarDamageData {
    header,
    car_damage_data
});

impl_packed_serialize!(super::session_history::LapHistoryData {
    lap_time_in_ms,
    sector1_time_ms_part,
    sector1_time_minutes_part,
    sector2_time_ms_part,
    sector2_time_minutes_part,
    sector3_time_ms_part,
    sector3_time_minutes_part,
    lap_valid_bit_flags,
});
impl_packed_serialize!(super::session_history::TyreStintHistoryData {
    end_lap,
    tyre_actual_compound,
    tyre_visual_compound,
});
impl_packed_serialize_with_slices!(super::session_history::PacketSessionHistoryData {
    header, car_idx, num_laps, num_tyre_stints, best_lap_time_lap_num, best_sector1_lap_num,
    best_sector2_lap_num, best_sector3_lap_num, tyre_stints_history_data,
}; lap_history_data);

impl_packed_serialize!(super::tyre_sets::TyreSetData {
    actual_tyre_compound,
    visual_tyre_compound,
    wear,
    available,
    recommended_session,
    life_span,
    usable_life,
    lap_delta_time,
    fitted,
});
impl_packed_serialize!(super::tyre_sets::PacketTyreSetsData {
    header,
    car_idx,
    tyre_set_data,
    fitted_idx,
});

impl_packed_serialize!(super::motion_ex::PacketMotionExData {
    header,
    suspension_position,
    suspension_velocity,
    suspension_acceleration,
    wheel_speed,
    wheel_slip_ratio,
    wheel_slip_angle,
    wheel_lat_force,
    wheel_long_force,
    height_of_cog_above_ground,
    local_velocity_x,
    local_velocity_y,
    local_velocity_z,
    angular_velocity_x,
    angular_velocity_y,
    angular_velocity_z,
    angular_acceleration_x,
    angular_acceleration_y,
    angular_acceleration_z,
    front_wheels_angle,
    wheel_vert_force,
    front_aero_height,
    rear_aero_height,
    front_roll_angle,
    rear_roll_angle,
    chassis_yaw,
    chassis_pitch,
    wheel_camber,
    wheel_camber_gain,
});

impl_packed_serialize!(super::time_trial::TimeTrialDataSet {
    car_idx,
    team_id,
    lap_time_in_ms,
    sector1_time_in_ms,
    sector2_time_in_ms,
    sector3_time_in_ms,
    traction_control,
    gearbox_assist,
    anti_lock_brakes,
    equal_car_performance,
    custom_setup,
    valid,
});
impl_packed_serialize!(super::time_trial::PacketTimeTrialData {
    header,
    player_session_best_data_set,
    personal_best_data_set,
    rival_data_set,
});

impl_packed_serialize_with_slices!(super::lap_positions::PacketLapPositionsData {
    header, num_laps, lap_start,
}; position_for_vehicle_idx);

impl_frontend_packet!(super::motion::PacketMotionData, "udp_motion");
impl_frontend_packet!(super::session::PacketSessionData, "udp_session");
impl_frontend_packet!(super::lap::PacketLapData, "udp_lap");
impl_frontend_packet!(super::event::PacketEventData, "udp_event");
impl_frontend_packet!(
    super::participants::PacketParticipantsData,
    "udp_participants"
);
impl_frontend_packet!(super::car_setup::PacketCarSetupData, "udp_carsetup");
impl_frontend_packet!(
    super::car_telemetry::PacketCarTelemetryData,
    "udp_cartelemetry"
);
impl_frontend_packet!(super::car_status::PacketCarStatusData, "udp_carstatus");
impl_frontend_packet!(
    super::final_classification::PacketFinalClassificationData,
    "udp_finalclassification"
);
impl_frontend_packet!(super::lobby_info::PacketLobbyInfoData, "udp_lobbyinfo");
impl_frontend_packet!(super::car_damage::PacketCarDamageData, "udp_cardamage");
impl_frontend_packet!(
    super::session_history::PacketSessionHistoryData,
    "udp_sessionhistory"
);
impl_frontend_packet!(super::tyre_sets::PacketTyreSetsData, "udp_tyresets");
impl_frontend_packet!(super::motion_ex::PacketMotionExData, "udp_motionex");
impl_frontend_packet!(super::time_trial::PacketTimeTrialData, "udp_timetrial");
impl_frontend_packet!(
    super::lap_positions::PacketLapPositionsData,
    "udp_lappositions"
);

#[cfg(test)]
mod tests {
    use super::super::header::F1Header;

    #[test]
    fn serializes_packed_header() {
        let header = F1Header {
            packet_format: 2025,
            game_year: 25,
            major_version: 1,
            minor_version: 2,
            packet_version: 1,
            packet_id: 7,
            session_uid: 42,
            session_time: 12.5,
            frame_id: 10,
            ov_frame_id: 9,
            p_car_index: 7,
            sec_p_car_index: 255,
        };

        let json = serde_json::to_value(header).unwrap();
        assert_eq!(json["session_uid"], 42);
        assert_eq!(json["packet_id"], 7);
        assert_eq!(json["p_car_index"], 7);
    }
}
