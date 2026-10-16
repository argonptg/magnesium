// F1 25 UDP — formato 2025 — motion_ex.rs
// Structs compactados com layout fiel ao protocolo (little-endian).

use super::header::F1Header;

/// PacketMotionExData — 273 bytes.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketMotionExData {
    pub header: F1Header,                  // 29 bytes
    pub suspension_position: [f32; 4],     // 16 bytes
    pub suspension_velocity: [f32; 4],     // 16 bytes
    pub suspension_acceleration: [f32; 4], // 16 bytes
    pub wheel_speed: [f32; 4],             // 16 bytes
    pub wheel_slip_ratio: [f32; 4],        // 16 bytes
    pub wheel_slip_angle: [f32; 4],        // 16 bytes
    pub wheel_lat_force: [f32; 4],         // 16 bytes
    pub wheel_long_force: [f32; 4],        // 16 bytes
    pub height_of_cog_above_ground: f32,   // 4 bytes
    pub local_velocity_x: f32,             // 4 bytes
    pub local_velocity_y: f32,             // 4 bytes
    pub local_velocity_z: f32,             // 4 bytes
    pub angular_velocity_x: f32,           // 4 bytes
    pub angular_velocity_y: f32,           // 4 bytes
    pub angular_velocity_z: f32,           // 4 bytes
    pub angular_acceleration_x: f32,       // 4 bytes
    pub angular_acceleration_y: f32,       // 4 bytes
    pub angular_acceleration_z: f32,       // 4 bytes
    pub front_wheels_angle: f32,           // 4 bytes
    pub wheel_vert_force: [f32; 4],        // 16 bytes
    pub front_aero_height: f32,            // 4 bytes
    pub rear_aero_height: f32,             // 4 bytes
    pub front_roll_angle: f32,             // 4 bytes
    pub rear_roll_angle: f32,              // 4 bytes
    pub chassis_yaw: f32,                  // 4 bytes
    pub chassis_pitch: f32,                // 4 bytes
    pub wheel_camber: [f32; 4],            // 16 bytes
    pub wheel_camber_gain: [f32; 4],       // 16 bytes
}

const _: [(); 273] = [(); core::mem::size_of::<PacketMotionExData>()];
