// F1 25 UDP — formato 2025 — lobby_info.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// LobbyInfoData — 42 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct LobbyInfoData {
    pub ai_controlled: u8, // 1 byte
    pub team_id: u8, // 1 byte
    pub nationality: u8, // 1 byte
    pub platform: u8, // 1 byte
    pub name: [u8; 32], // 32 bytes
    pub car_number: u8, // 1 byte
    pub your_telemetry: u8, // 1 byte
    pub show_online_names: u8, // 1 byte
    pub tech_level: u16, // 2 bytes
    pub ready_status: u8, // 1 byte
}

/// PacketLobbyInfoData — 954 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketLobbyInfoData {
    pub header: F1Header, // 29 bytes
    pub num_players: u8, // 1 byte
    pub lobby_players: [LobbyInfoData; 22], // 924 bytes
}

const _: [(); 42] = [(); core::mem::size_of::<LobbyInfoData>()];
const _: [(); 954] = [(); core::mem::size_of::<PacketLobbyInfoData>()];
