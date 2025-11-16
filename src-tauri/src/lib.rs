use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::State;
use tauri::Manager;

// Data structures for settings
#[derive(Debug, Serialize, Deserialize)]
struct AppSettings {
    personal: PersonalInfo,
    design: DesignSettings,
    cnc: CncSettings,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonalInfo {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "birthDate")]
    birth_date: String,
    #[serde(rename = "deathDate")]
    death_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DesignSettings {
    layout: String,
    #[serde(rename = "nameSize")]
    name_size: f64,
    #[serde(rename = "dateSize")]
    date_size: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CncSettings {
    depth: f64,
    #[serde(rename = "feedRate")]
    feed_rate: i32,
    #[serde(rename = "plungeRate")]
    plunge_rate: i32,
    #[serde(rename = "spindleSpeed")]
    spindle_speed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MachineStatus {
    status: String,
    machine_position: Position,
    work_position: Position,
    progress: f64,
    is_homed: bool,
}

// CNC Machine State
#[derive(Debug)]
struct CncMachine {
    port: Option<Box<dyn SerialPort>>,
    is_connected: bool,
    current_line: usize,
    total_lines: usize,
}

impl CncMachine {
    fn new() -> Self {
        Self {
            port: None,
            is_connected: false,
            current_line: 0,
            total_lines: 0,
        }
    }
}

type CncState = Arc<Mutex<CncMachine>>;

// Tauri Commands

#[tauri::command]
fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    let config_dir = app.path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;
    
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    let settings_file = config_dir.join("settings.json");
    fs::write(settings_file, settings_json)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let config_dir = app.path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;
    
    let settings_file = config_dir.join("settings.json");
    
    if !settings_file.exists() {
        return Err("Settings file does not exist".to_string());
    }
    
    let settings_json = fs::read_to_string(settings_file)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    
    let settings: AppSettings = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings: {}", e))?;
    
    Ok(settings)
}

#[tauri::command]
fn get_available_ports() -> Result<Vec<String>, String> {
    let ports = serialport::available_ports()
        .map_err(|e| format!("Failed to get available ports: {}", e))?;
    
    Ok(ports.into_iter().map(|p| p.port_name).collect())
}

#[tauri::command]
fn connect_cnc(cnc_state: State<'_, CncState>, port: String) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if machine.is_connected {
        return Err("Already connected".to_string());
    }
    
    let serial_port = serialport::new(&port, 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| format!("Failed to open port {}: {}", port, e))?;
    
    machine.port = Some(serial_port);
    machine.is_connected = true;
    
    // Send initial setup commands
    if let Some(ref mut port) = machine.port {
        // Wake up GRBL
        port.write_all(b"\r\n\r\n").map_err(|e| format!("Write error: {}", e))?;
        thread::sleep(Duration::from_millis(2000));
        
        // Clear any startup messages
        let mut buffer = [0u8; 1024];
        let _ = port.read(&mut buffer);
        
        // Get GRBL status
        port.write_all(b"?\n").map_err(|e| format!("Write error: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
fn disconnect_cnc(cnc_state: State<'_, CncState>) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    machine.port = None;
    machine.is_connected = false;
    machine.current_line = 0;
    machine.total_lines = 0;
    
    Ok(())
}

#[tauri::command]
fn home_machine(cnc_state: State<'_, CncState>) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    if let Some(ref mut port) = machine.port {
        port.write_all(b"$H\n").map_err(|e| format!("Write error: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
fn jog_machine(cnc_state: State<'_, CncState>, axis: String, distance: f64) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    let command = format!("$J=G91{}{}F500\n", axis.to_uppercase(), distance);
    
    if let Some(ref mut port) = machine.port {
        port.write_all(command.as_bytes()).map_err(|e| format!("Write error: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
fn start_engraving(cnc_state: State<'_, CncState>, gcode: String) -> Result<(), String> {
    let lines: Vec<&str> = gcode.lines().collect();
    let total = lines.len();
    
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    machine.total_lines = total;
    machine.current_line = 0;
    
    // Send G-code line by line (simplified - in production, you'd want proper flow control)
    if let Some(ref mut port) = machine.port {
        for (idx, line) in lines.iter().enumerate() {
            if !line.trim().is_empty() && !line.starts_with(';') {
                port.write_all(format!("{}\n", line).as_bytes())
                    .map_err(|e| format!("Write error: {}", e))?;
                
                // Wait for OK response (simplified)
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
    
    // Update current line after writing is complete
    machine.current_line = total;
    
    Ok(())
}

#[tauri::command]
fn pause_engraving(cnc_state: State<'_, CncState>) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    if let Some(ref mut port) = machine.port {
        port.write_all(b"!\n").map_err(|e| format!("Write error: {}", e))?; // Feed hold
    }
    
    Ok(())
}

#[tauri::command]
fn resume_engraving(cnc_state: State<'_, CncState>) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    if let Some(ref mut port) = machine.port {
        port.write_all(b"~\n").map_err(|e| format!("Write error: {}", e))?; // Resume
    }
    
    Ok(())
}

#[tauri::command]
fn stop_engraving(cnc_state: State<'_, CncState>) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    if let Some(ref mut port) = machine.port {
        port.write_all(b"!\n").map_err(|e| format!("Write error: {}", e))?; // Feed hold first
        thread::sleep(Duration::from_millis(100));
        port.write_all(b"\x18\n").map_err(|e| format!("Write error: {}", e))?; // Soft reset
    }
    
    machine.current_line = 0;
    machine.total_lines = 0;
    
    Ok(())
}

#[tauri::command]
fn emergency_stop(cnc_state: State<'_, CncState>) -> Result<(), String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Ok(()); // Not connected, nothing to stop
    }
    
    if let Some(ref mut port) = machine.port {
        // Send immediate stop commands
        let _ = port.write_all(b"!\n"); // Feed hold
        thread::sleep(Duration::from_millis(10));
        let _ = port.write_all(b"\x18\n"); // Soft reset
        thread::sleep(Duration::from_millis(10));
        let _ = port.write_all(b"M5\n"); // Stop spindle
    }
    
    machine.current_line = 0;
    machine.total_lines = 0;
    
    Ok(())
}

#[tauri::command]
fn get_machine_status(cnc_state: State<'_, CncState>) -> Result<MachineStatus, String> {
    let mut machine = cnc_state.lock().unwrap();
    
    if !machine.is_connected {
        return Err("Not connected".to_string());
    }
    
    // Request status from GRBL
    if let Some(ref mut port) = machine.port {
        port.write_all(b"?\n").map_err(|e| format!("Write error: {}", e))?;
        
        // Read response (simplified - in production, you'd parse the actual GRBL response)
        let mut buffer = String::new();
        let mut reader = BufReader::new(port.as_mut());
        let _ = reader.read_line(&mut buffer);
        
        // Parse GRBL status response (simplified)
        let progress = if machine.total_lines > 0 {
            (machine.current_line as f64 / machine.total_lines as f64) * 100.0
        } else {
            0.0
        };
        
        let status = MachineStatus {
            status: "Idle".to_string(), // Would parse from actual response
            machine_position: Position { x: 0.0, y: 0.0, z: 0.0 },
            work_position: Position { x: 0.0, y: 0.0, z: 0.0 },
            progress,
            is_homed: true, // Would determine from actual status
        };
        
        return Ok(status);
    }
    
    Err("No serial connection".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cnc_state: CncState = Arc::new(Mutex::new(CncMachine::new()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(cnc_state)
        .invoke_handler(tauri::generate_handler![
            save_settings,
            load_settings,
            get_available_ports,
            connect_cnc,
            disconnect_cnc,
            home_machine,
            jog_machine,
            start_engraving,
            pause_engraving,
            resume_engraving,
            stop_engraving,
            emergency_stop,
            get_machine_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}