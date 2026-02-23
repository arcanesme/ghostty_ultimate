mod config;
mod executor;
mod generator;
mod profiles;

use config::schema::GhosttyConfig;
use config::palette::NamedPalette;
use executor::parser::{ExecutionEvent, parse_line};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::ipc::Channel;

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn get_default_config() -> GhosttyConfig {
    GhosttyConfig::default()
}

#[tauri::command]
fn validate_config(config: GhosttyConfig) -> Result<Vec<String>, String> {
    config::schema::validate(&config)
}

#[tauri::command]
fn generate_script(config: GhosttyConfig) -> Result<String, String> {
    generator::engine::generate(&config)
}

#[tauri::command]
fn preview_section(config: GhosttyConfig, section: String) -> Result<String, String> {
    generator::engine::preview_section(&config, &section)
}

#[tauri::command]
fn get_builtin_palettes() -> Vec<NamedPalette> {
    config::palette::builtin_palettes()
}

#[tauri::command]
fn save_profile(name: String, config: GhosttyConfig) -> Result<(), String> {
    profiles::manager::save(&name, &config)
}

#[tauri::command]
fn load_profile(name: String) -> Result<GhosttyConfig, String> {
    profiles::manager::load(&name)
}

#[tauri::command]
fn list_profiles() -> Result<Vec<String>, String> {
    profiles::manager::list()
}

#[tauri::command]
fn delete_profile(name: String) -> Result<(), String> {
    profiles::manager::delete(&name)
}

#[tauri::command]
fn export_profile(config: GhosttyConfig, path: String) -> Result<(), String> {
    profiles::manager::export(&config, &path)
}

#[tauri::command]
fn import_profile(path: String) -> Result<GhosttyConfig, String> {
    profiles::manager::import(&path)
}

#[tauri::command]
fn detect_installed_packages() -> Result<Vec<String>, String> {
    executor::runner::detect_packages()
}

#[tauri::command]
fn detect_installed_fonts() -> Result<Vec<String>, String> {
    executor::runner::detect_fonts()
}

#[tauri::command]
async fn execute_script(script_path: String, on_event: Channel<ExecutionEvent>) -> Result<(), String> {
    use std::io::{BufRead, BufReader};
    use std::process::{Command, Stdio};

    CANCEL_FLAG.store(false, Ordering::SeqCst);

    let mut child = Command::new("bash")
        .arg(&script_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn script: {}", e))?;

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if CANCEL_FLAG.load(Ordering::SeqCst) {
            let _ = child.kill();
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let _ = on_event.send(ExecutionEvent {
                event_type: "error".into(),
                section: None,
                total_sections: None,
                message: "Execution cancelled by user".into(),
                timestamp: ts,
            });
            return Ok(());
        }

        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;

        // Check for progress events
        if let Some(event) = parse_line(&line) {
            let _ = on_event.send(event);
        }

        // Always send log events
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let _ = on_event.send(ExecutionEvent {
            event_type: "log".into(),
            section: None,
            total_sections: None,
            message: line,
            timestamp: ts,
        });
    }

    let status = child.wait().map_err(|e| format!("Failed to wait for script: {}", e))?;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if status.success() {
        let _ = on_event.send(ExecutionEvent {
            event_type: "complete".into(),
            section: Some(13),
            total_sections: Some(13),
            message: "Script completed successfully".into(),
            timestamp: ts,
        });
    } else {
        let _ = on_event.send(ExecutionEvent {
            event_type: "error".into(),
            section: None,
            total_sections: None,
            message: format!("Script exited with code: {}", status.code().unwrap_or(-1)),
            timestamp: ts,
        });
    }

    Ok(())
}

#[tauri::command]
fn cancel_execution() -> Result<(), String> {
    CANCEL_FLAG.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn apply_config(config: GhosttyConfig, section: String) -> Result<String, String> {
    generator::applier::apply_section(&config, &section)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_default_config,
            validate_config,
            generate_script,
            preview_section,
            get_builtin_palettes,
            save_profile,
            load_profile,
            list_profiles,
            delete_profile,
            export_profile,
            import_profile,
            detect_installed_packages,
            detect_installed_fonts,
            execute_script,
            cancel_execution,
            apply_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
