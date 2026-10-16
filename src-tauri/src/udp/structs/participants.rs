// F1 25 UDP — formato 2025 — participants.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// LiveryColour — 3 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct LiveryColour {
    pub red: u8,   // 1 byte
    pub green: u8, // 1 byte
    pub blue: u8,  // 1 byte
}

/// ParticipantData — 57 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ParticipantData {
    pub ai_controlled: u8,                 // 1 byte
    pub driver_id: u8,                     // 1 byte
    pub network_id: u8,                    // 1 byte
    pub team_id: u8,                       // 1 byte
    pub my_team: u8,                       // 1 byte
    pub race_number: u8,                   // 1 byte
    pub nationality: u8,                   // 1 byte
    pub name: [u8; 32],                    // 32 bytes
    pub your_telemetry: u8,                // 1 byte
    pub show_online_names: u8,             // 1 byte
    pub tech_level: u16,                   // 2 bytes
    pub platform: u8,                      // 1 byte
    pub num_colours: u8,                   // 1 byte
    pub livery_colours: [LiveryColour; 4], // 12 bytes
}

/// PacketParticipantsData — 1284 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketParticipantsData {
    pub header: F1Header,                    // 29 bytes
    pub num_active_cars: u8,                 // 1 byte
    pub participants: [ParticipantData; 22], // 1254 bytes
}

const _: [(); 3] = [(); core::mem::size_of::<LiveryColour>()];
const _: [(); 57] = [(); core::mem::size_of::<ParticipantData>()];
const _: [(); 1284] = [(); core::mem::size_of::<PacketParticipantsData>()];
