// F1 25 UDP — formato 2025 — car_damage.rs
// structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// CarDamageData — 46 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CarDamageData {
    pub tyres_wear: [f32; 4],        // 16 bytes
    pub tyres_damage: [u8; 4],       // 4 bytes
    pub brakes_damage: [u8; 4],      // 4 bytes
    pub tyre_blisters: [u8; 4],      // 4 bytes
    pub front_left_wing_damage: u8,  // 1 byte
    pub front_right_wing_damage: u8, // 1 byte
    pub rear_wing_damage: u8,        // 1 byte
    pub floor_damage: u8,            // 1 byte
    pub diffuser_damage: u8,         // 1 byte
    pub sidepod_damage: u8,          // 1 byte
    pub drs_fault: u8,               // 1 byte
    pub ers_fault: u8,               // 1 byte
    pub gear_box_damage: u8,         // 1 byte
    pub engine_damage: u8,           // 1 byte
    pub engine_mguh_wear: u8,        // 1 byte
    pub engine_es_wear: u8,          // 1 byte
    pub engine_ce_wear: u8,          // 1 byte
    pub engine_ice_wear: u8,         // 1 byte
    pub engine_mguk_wear: u8,        // 1 byte
    pub engine_tc_wear: u8,          // 1 byte
    pub engine_blown: u8,            // 1 byte
    pub engine_seized: u8,           // 1 byte
}

/// PacketCarDamageData — 1041 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketCarDamageData {
    pub header: F1Header,                     // 29 bytes
    pub car_damage_data: [CarDamageData; 22], // 1012 bytes
}

const _: [(); 46] = [(); core::mem::size_of::<CarDamageData>()];
const _: [(); 1041] = [(); core::mem::size_of::<PacketCarDamageData>()];
