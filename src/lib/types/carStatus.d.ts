import type { F1Header } from "./header";

export interface CarStatusData {
    traction_control: number;
    anti_lock_brakes: number;
    fuel_mix: number;
    front_brake_bias: number;
    pit_limiter_status: number;
    fuel_in_tank: number;
    fuel_capacity: number;
    fuel_remaining_laps: number;
    max_rpm: number;
    idle_rpm: number;
    max_gears: number;
    drs_allowed: number;
    drs_activation_distance: number;
    actual_tyre_compound: number;
    visual_tyre_compound: number;
    tyres_age_laps: number;
    vehicle_fia_flags: number;
    engine_power_ice: number;
    engine_power_mguk: number;
    ers_store_energy: number;
    ers_deploy_mode: number;
    ers_harvested_this_lap_mguk: number;
    ers_harvested_this_lap_mguh: number;
    ers_deployed_this_lap: number;
    network_paused: number;
}

export interface PacketCarStatusData {
    header: F1Header,
    car_status_data: CarStatusData[]
}
