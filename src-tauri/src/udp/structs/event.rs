// F1 25 UDP — formato 2025 — event.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// PacketEventData — 33 bytes fixos (cabeçalho + código); dados do evento têm tamanho variável.
/// Use `event_payload()` para obter os bytes após o código ASCII.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketEventData {
    pub header: F1Header, // 29 bytes
    pub event_string_code: [u8; 4], // 4 bytes
}

/// FastestLapEvent — 5 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FastestLapEvent {
    pub vehicle_idx: u8, // 1 byte
    pub lap_time: f32, // 4 bytes
}

/// RetirementEvent — 2 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RetirementEvent {
    pub vehicle_idx: u8, // 1 byte
    pub reason: u8, // 1 byte
}

/// DRSDisabledEvent — 1 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct DRSDisabledEvent {
    pub reason: u8, // 1 byte
}

/// TeamMateInPitsEvent — 1 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TeamMateInPitsEvent {
    pub vehicle_idx: u8, // 1 byte
}

/// RaceWinnerEvent — 1 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RaceWinnerEvent {
    pub vehicle_idx: u8, // 1 byte
}

/// PenaltyEvent — 7 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PenaltyEvent {
    pub penalty_type: u8, // 1 byte
    pub infringement_type: u8, // 1 byte
    pub vehicle_idx: u8, // 1 byte
    pub other_vehicle_idx: u8, // 1 byte
    pub time: u8, // 1 byte
    pub lap_num: u8, // 1 byte
    pub places_gained: u8, // 1 byte
}

/// SpeedTrapEvent — 12 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SpeedTrapEvent {
    pub vehicle_idx: u8, // 1 byte
    pub speed: f32, // 4 bytes
    pub is_overall_fastest_in_session: u8, // 1 byte
    pub is_driver_fastest_in_session: u8, // 1 byte
    pub fastest_vehicle_idx_in_session: u8, // 1 byte
    pub fastest_speed_in_session: f32, // 4 bytes
}

/// StartLightsEvent — 1 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StartLightsEvent {
    pub num_lights: u8, // 1 byte
}

/// DriveThroughPenaltyServedEvent — 1 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct DriveThroughPenaltyServedEvent {
    pub vehicle_idx: u8, // 1 byte
}

/// StopGoPenaltyServedEvent — 5 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StopGoPenaltyServedEvent {
    pub vehicle_idx: u8, // 1 byte
    pub stop_time: f32, // 4 bytes
}

/// FlashbackEvent — 8 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FlashbackEvent {
    pub flashback_frame_identifier: u32, // 4 bytes
    pub flashback_session_time: f32, // 4 bytes
}

/// ButtonsEvent — 4 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ButtonsEvent {
    pub button_status: u32, // 4 bytes
}

/// OvertakeEvent — 2 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct OvertakeEvent {
    pub overtaking_vehicle_idx: u8, // 1 byte
    pub being_overtaken_vehicle_idx: u8, // 1 byte
}

/// SafetyCarEvent — 2 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct SafetyCarEvent {
    pub safety_car_type: u8, // 1 byte
    pub event_type: u8, // 1 byte
}

/// CollisionEvent — 2 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CollisionEvent {
    pub vehicle1_idx: u8, // 1 byte
    pub vehicle2_idx: u8, // 1 byte
}

const _: [(); 33] = [(); core::mem::size_of::<PacketEventData>()];
const _: [(); 5] = [(); core::mem::size_of::<FastestLapEvent>()];
const _: [(); 2] = [(); core::mem::size_of::<RetirementEvent>()];
const _: [(); 1] = [(); core::mem::size_of::<DRSDisabledEvent>()];
const _: [(); 1] = [(); core::mem::size_of::<TeamMateInPitsEvent>()];
const _: [(); 1] = [(); core::mem::size_of::<RaceWinnerEvent>()];
const _: [(); 7] = [(); core::mem::size_of::<PenaltyEvent>()];
const _: [(); 12] = [(); core::mem::size_of::<SpeedTrapEvent>()];
const _: [(); 1] = [(); core::mem::size_of::<StartLightsEvent>()];
const _: [(); 1] = [(); core::mem::size_of::<DriveThroughPenaltyServedEvent>()];
const _: [(); 5] = [(); core::mem::size_of::<StopGoPenaltyServedEvent>()];
const _: [(); 8] = [(); core::mem::size_of::<FlashbackEvent>()];
const _: [(); 4] = [(); core::mem::size_of::<ButtonsEvent>()];
const _: [(); 2] = [(); core::mem::size_of::<OvertakeEvent>()];
const _: [(); 2] = [(); core::mem::size_of::<SafetyCarEvent>()];
const _: [(); 2] = [(); core::mem::size_of::<CollisionEvent>()];
