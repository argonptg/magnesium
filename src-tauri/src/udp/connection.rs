#[tauri::command]
pub fn connect() -> String {
    println!("Connecting to UDP server...");
    return "Connected to UDP server!".into();
}