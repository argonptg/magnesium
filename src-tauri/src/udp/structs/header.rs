// F1 25 UDP — formato 2025 — header.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

pub const MAX_CARS: usize = 22; // meio descritivel, 22 carros/11 equipes
pub const MAX_PARTICIPANT_NAME_LEN: usize = 32;
pub const MAX_TYRE_STINTS: usize = 8;
pub const MAX_TYRE_SETS: usize = 20;
pub const MAX_MARSHAL_ZONES: usize = 21;
pub const MAX_WEATHER_SAMPLES: usize = 64;
pub const MAX_WEEKEND_SESSIONS: usize = 12;
pub const MAX_HISTORY_LAPS: usize = 100;
pub const MAX_POSITION_LAPS: usize = 50;
pub const PACKET_FORMAT: u16 = 2025;
pub const HEADER_SIZE: usize = 29;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketId {
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

impl TryFrom<u8> for PacketId {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MotionData),
            1 => Ok(Self::SessionData),
            2 => Ok(Self::LapData),
            3 => Ok(Self::EventData),
            4 => Ok(Self::ParticipantsData),
            5 => Ok(Self::CarSetupData),
            6 => Ok(Self::CarTelemetryData),
            7 => Ok(Self::CarStatusData),
            8 => Ok(Self::FinalClassificationData),
            9 => Ok(Self::LobbyInfoData),
            10 => Ok(Self::CarDamageData),
            11 => Ok(Self::SessionHistoryData),
            12 => Ok(Self::TyreSetsData),
            13 => Ok(Self::MotionExData),
            14 => Ok(Self::TimeTrialData),
            15 => Ok(Self::LapPositionsData),
            _ => Err(()),
        }
    }
}

/// F1Header — 29 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
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

const _: [(); 29] = [(); core::mem::size_of::<F1Header>()];
