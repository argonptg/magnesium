use chrono::prelude::*;

#[tauri::command]
pub fn log_info(msg: &str, from_rust: bool) {
    let local_time = Local::now();
    let formatted_time = local_time.format("%H:%M:%S").to_string();

    let log_message: String;

    if from_rust {
        log_message = format!("\x1b[34m[INFO - RS | {}]\x1b[0m {}", formatted_time, msg);
    } else {
        log_message = format!("\x1b[34m[INFO - TS | {}]\x1b[0m {}", formatted_time, msg);
    }

    println!("{}", log_message);
}

#[tauri::command]
pub fn log_warn(msg: &str, from_rust: bool) {
    let local_time = Local::now();
    let formatted_time = local_time.format("%H:%M:%S").to_string();

    let log_message: String;

    if from_rust {
        log_message = format!("\x1b[33m[WARN - RS | {}]\x1b[0m {}", formatted_time, msg);
    } else {
        log_message = format!("\x1b[33m[WARN - TS | {}]\x1b[0m {}", formatted_time, msg);
    }

    println!("{}", log_message);
}

#[tauri::command]
pub fn log_error(msg: &str, from_rust: bool) {
    let local_time = Local::now();
    let formatted_time = local_time.format("%H:%M:%S").to_string();

    let log_message: String;

    if from_rust {
        log_message = format!("\x1b[31m[ERROR - RS | {}]\x1b[0m {}", formatted_time, msg);
    } else {
        log_message = format!("\x1b[31m[ERROR - TS | {}]\x1b[0m {}", formatted_time, msg);
    }

    println!("{}", log_message);
}