// F1 25 UDP — formato 2025 — motion.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// CarMotionData — 60 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CarMotionData {
    pub world_position_x: f32,     // 4 bytes
    pub world_position_y: f32,     // 4 bytes
    pub world_position_z: f32,     // 4 bytes
    pub world_velocity_x: f32,     // 4 bytes
    pub world_velocity_y: f32,     // 4 bytes
    pub world_velocity_z: f32,     // 4 bytes
    pub world_forward_dir_x: i16,  // 2 bytes
    pub world_forward_dir_y: i16,  // 2 bytes
    pub world_forward_dir_z: i16,  // 2 bytes
    pub world_right_dir_x: i16,    // 2 bytes
    pub world_right_dir_y: i16,    // 2 bytes
    pub world_right_dir_z: i16,    // 2 bytes
    pub g_force_lateral: f32,      // 4 bytes
    pub g_force_longitudinal: f32, // 4 bytes
    pub g_force_vertical: f32,     // 4 bytes
    pub yaw: f32,                  // 4 bytes
    pub pitch: f32,                // 4 bytes
    pub roll: f32,                 // 4 bytes
}

/// PacketMotionData — 1349 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketMotionData {
    pub header: F1Header,                     // 29 bytes
    pub car_motion_data: [CarMotionData; 22], // 1320 bytes
}

const _: [(); 60] = [(); core::mem::size_of::<CarMotionData>()];
const _: [(); 1349] = [(); core::mem::size_of::<PacketMotionData>()];
