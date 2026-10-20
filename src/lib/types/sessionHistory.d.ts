import type { F1Header } from "./header";

export interface LapHistoryData {
    lap_time_in_ms: number;
    sector1_time_ms_part: number;
    sector1_time_minutes_part: number;
    sector2_time_ms_part: number;
    sector2_time_minutes_part: number;
    sector3_time_ms_part: number;
    sector3_time_minutes_part: number;
    lap_valid_bit_flags: number;
}

export interface TyreStintHistoryData {
    end_lap: number;
    tyre_actual_compound: number;
    tyre_visual_compound: number;
}

export interface PacketSessionHistoryData {
    header: F1Header;
    car_idx: number;
    num_laps: number;
    num_tyre_stints: number;
    best_lap_time_lap_num: number;
    best_sector1_lap_num: number;
    best_sector2_lap_num: number;
    best_sector3_lap_num: number;
    lap_history_data: LapHistoryData[];
    tyre_stints_history_data: TyreStintHistoryData[];
}