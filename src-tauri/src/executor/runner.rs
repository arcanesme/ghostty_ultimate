use std::process::Command;

/// Detect installed Homebrew packages
pub fn detect_packages() -> Result<Vec<String>, String> {
    let output = Command::new("brew")
        .args(["list", "--formula", "-1"])
        .output()
        .map_err(|e| format!("Failed to run brew: {}", e))?;

    if !output.status.success() {
        return Err("brew list failed".into());
    }

    let packages = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    Ok(packages)
}

/// Detect installed fonts by scanning font directories
pub fn detect_fonts() -> Result<Vec<String>, String> {
    let mut fonts = Vec::new();
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;

    let font_dirs = vec![
        home.join("Library/Fonts"),
        std::path::PathBuf::from("/Library/Fonts"),
        std::path::PathBuf::from("/System/Library/Fonts"),
    ];

    for dir in font_dirs {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.ends_with(".ttf") || name.ends_with(".otf") || name.ends_with(".ttc") {
                        // Extract font family name from filename
                        let family = name
                            .trim_end_matches(".ttf")
                            .trim_end_matches(".otf")
                            .trim_end_matches(".ttc")
                            .replace('-', " ")
                            .replace('_', " ");
                        if !fonts.contains(&family) {
                            fonts.push(family);
                        }
                    }
                }
            }
        }
    }

    fonts.sort();
    fonts.dedup();
    Ok(fonts)
}
