use std::net::UdpSocket;
use crate::utils::log::*;

#[tauri::command]
pub fn connect(ip: &str, port: u16) -> String {
    let msg = format!("Connecting to UDP server at {}:{}", ip, port);
    log_info(&msg, true);
    
    return "Connected to UDP server!".into();
}