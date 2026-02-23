use serde::{Deserialize, Serialize};
use super::palette::AyuPalette;
use super::sections::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhosttyConfig {
    pub palette: AyuPalette,
    pub clean_artifacts: CleanArtifactsConfig,
    pub packages: PackagesConfig,
    pub ghostty: GhosttyTermConfig,
    pub starship: StarshipConfig,
    pub fastfetch: FastfetchConfig,
    pub zsh_plugins: ZshPluginsConfig,
    pub tmux: TmuxConfig,
    pub neovim: NeovimConfig,
    pub tool_configs: ToolConfigsConfig,
    pub git: GitConfig,
    pub themes: ThemesConfig,
    pub tmux_scripts: TmuxScriptsConfig,
    pub zshrc: ZshrcConfig,
}

impl Default for GhosttyConfig {
    fn default() -> Self {
        super::defaults::default_config()
    }
}

pub fn validate(config: &GhosttyConfig) -> Result<Vec<String>, String> {
    let mut warnings = Vec::new();

    // Validate palette colors are valid hex
    let palette_colors = [
        (&config.palette.bg, "bg"),
        (&config.palette.fg, "fg"),
        (&config.palette.surface, "surface"),
        (&config.palette.selection, "selection"),
        (&config.palette.red, "red"),
        (&config.palette.green, "green"),
        (&config.palette.yellow, "yellow"),
        (&config.palette.blue, "blue"),
        (&config.palette.purple, "purple"),
        (&config.palette.cyan, "cyan"),
    ];

    for (color, name) in &palette_colors {
        if !color.starts_with('#') || color.len() != 7 {
            warnings.push(format!("Palette color '{}' is not a valid hex color: {}", name, color));
        }
    }

    // Validate numeric ranges
    if config.ghostty.font_size < 8.0 || config.ghostty.font_size > 72.0 {
        warnings.push("Font size should be between 8 and 72".into());
    }

    if config.ghostty.background_opacity < 0.0 || config.ghostty.background_opacity > 1.0 {
        warnings.push("Background opacity should be between 0.0 and 1.0".into());
    }

    if config.ghostty.scrollback_limit > 1_000_000 {
        warnings.push("Scrollback limit is very high, may use excessive memory".into());
    }

    if config.tmux.history_limit > 1_000_000 {
        warnings.push("tmux history limit is very high".into());
    }

    // Check for package dependencies
    let enabled_pkgs: Vec<&str> = config.packages.packages.iter()
        .filter(|p| p.enabled)
        .map(|p| p.name.as_str())
        .collect();

    if config.zshrc.aliases.iter().any(|a| a.enabled && a.command.contains("eza")) && !enabled_pkgs.contains(&"eza") {
        warnings.push("Aliases reference 'eza' but it's not in enabled packages".into());
    }
    if config.zshrc.aliases.iter().any(|a| a.enabled && a.command.contains("bat")) && !enabled_pkgs.contains(&"bat") {
        warnings.push("Aliases reference 'bat' but it's not in enabled packages".into());
    }

    Ok(warnings)
}
