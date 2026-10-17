/*
pub struct F1Header {
    pub packet_format: u16,  // 2 bytes
    pub game_year: u8,       // 1 byte
    pub major_version: u8,   // 1 byte
    pub minor_version: u8,   // 1 byte
    pub packet_version: u8,  // 1 byte
    pub packet_id: u8,       // 1 byte
    pub session_uid: u64,    // 8 bytes
    pub session_time: f32,   // 4 bytes
    pub frame_id: u32,       // 4 bytes
    pub ov_frame_id: u32,    // 4 bytes
    pub p_car_index: u8,     // 1 byte
    pub sec_p_car_index: u8, // 1 byte
}
*/

export enum PacketId {
    MotionData = 0,
    SessionData = 1,
    LapData = 2,
    EventData = 3,
    ParticipantsData = 4,
    CarSetupData = 5,
    CarTelemetryData = 6,
    CarStatusData = 7,
    FinalClassificationData = 8,
    LobbyInfoData = 9,
    CarDamageData = 10,
    SessionHistoryData = 11,
    TyreSetsData = 12,
    MotionExData = 13,
    TimeTrialData = 14,
    LapPositionsData = 15,
}

export interface F1Header {
    packet_format: number; 
    game_year: number; 
    major_version: number; 
    minor_version: number;
    packet_version: number;
    packet_id: PacketId;
    session_uid: number;
    session_time: number;
    frame_id: number;
    ov_frame_id: number;
    p_car_index: number;
    sec_p_car_index: number;
}
