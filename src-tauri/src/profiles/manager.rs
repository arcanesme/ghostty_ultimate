use crate::config::schema::GhosttyConfig;
use std::path::PathBuf;

fn profiles_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    let dir = home.join(".config/ghostty-ultimate-ui/profiles");
    std::fs::create_dir_all(&dir).map_err(|e| format!("Cannot create profiles dir: {}", e))?;
    Ok(dir)
}

pub fn save(name: &str, config: &GhosttyConfig) -> Result<(), String> {
    let path = profiles_dir()?.join(format!("{}.json", name));
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| format!("Cannot write profile: {}", e))
}

pub fn load(name: &str) -> Result<GhosttyConfig, String> {
    let path = profiles_dir()?.join(format!("{}.json", name));
    let json = std::fs::read_to_string(&path).map_err(|e| format!("Cannot read profile: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("Invalid profile JSON: {}", e))
}

pub fn list() -> Result<Vec<String>, String> {
    let dir = profiles_dir()?;
    let mut names = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".json") {
                names.push(name.trim_end_matches(".json").to_string());
            }
        }
    }
    names.sort();
    Ok(names)
}

pub fn delete(name: &str) -> Result<(), String> {
    let path = profiles_dir()?.join(format!("{}.json", name));
    std::fs::remove_file(&path).map_err(|e| format!("Cannot delete profile: {}", e))
}

pub fn export(config: &GhosttyConfig, path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| format!("Cannot export: {}", e))
}

pub fn import(path: &str) -> Result<GhosttyConfig, String> {
    let json = std::fs::read_to_string(path).map_err(|e| format!("Cannot read: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("Invalid JSON: {}", e))
}
