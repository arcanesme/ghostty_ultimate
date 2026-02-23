use crate::config::schema::GhosttyConfig;
use super::engine::render_apply_template;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Apply a specific section's config to disk.
/// Returns a status message on success.
pub fn apply_section(config: &GhosttyConfig, section: &str) -> Result<String, String> {
    match section {
        "ghostty" => apply_ghostty(config),
        "starship" => apply_starship(config),
        "tmux" => apply_tmux(config),
        "neovim" => apply_neovim(config),
        "tool_configs" => apply_tool_configs(config),
        "git" => apply_git(config),
        "themes" => apply_themes(config),
        "fastfetch" => apply_fastfetch(config),
        "zsh_plugins" => apply_zsh_plugins(config),
        "zshrc" => apply_zshrc(config),
        _ => Err(format!("Section '{}' is not live-applyable", section)),
    }
}

fn home_dir() -> Result<PathBuf, String> {
    dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())
}

fn ensure_parent(path: &PathBuf) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }
    Ok(())
}

fn write_config(path: &PathBuf, content: &str) -> Result<(), String> {
    ensure_parent(path)?;
    fs::write(path, content)
        .map_err(|e| format!("Failed to write {}: {}", path.display(), e))
}

// ── Ghostty ─────────────────────────────────────────────────────────

fn apply_ghostty(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let ghostty_dir = home.join(".config/ghostty");

    // Write theme file
    let theme_content = render_apply_template(config, "apply/ghostty_theme.tera")?;
    let theme_path = ghostty_dir.join("themes/ayu-dark");
    write_config(&theme_path, &theme_content)?;

    // Write config file
    let config_content = render_apply_template(config, "apply/ghostty_config.tera")?;
    let config_path = ghostty_dir.join("config");
    write_config(&config_path, &config_content)?;

    // Ghostty auto-reloads its config - no post-hook needed
    Ok("Ghostty config applied (auto-reloads)".to_string())
}

// ── Starship ────────────────────────────────────────────────────────

fn apply_starship(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let content = render_apply_template(config, "apply/starship_config.tera")?;
    let path = home.join(".config/starship.toml");
    write_config(&path, &content)?;
    Ok("Starship config applied (next prompt picks it up)".to_string())
}

// ── tmux ────────────────────────────────────────────────────────────

fn apply_tmux(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let content = render_apply_template(config, "apply/tmux_config.tera")?;
    let path = home.join(".tmux.conf");
    write_config(&path, &content)?;

    // Post-hook: source the config if tmux is running
    let _ = Command::new("tmux")
        .args(["source-file", &path.to_string_lossy()])
        .output();

    Ok("tmux config applied and sourced".to_string())
}

// ── Neovim ──────────────────────────────────────────────────────────

fn apply_neovim(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let content = render_apply_template(config, "apply/neovim_config.tera")?;
    let path = home.join(".config/nvim/init.lua");
    write_config(&path, &content)?;
    Ok("Neovim config applied (manual :source to reload)".to_string())
}

// ── Tool Configs (Atuin + Lazygit) ──────────────────────────────────

fn apply_tool_configs(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let mut messages = Vec::new();

    // Atuin
    let atuin_content = render_apply_template(config, "apply/atuin_config.tera")?;
    let atuin_path = home.join(".config/atuin/config.toml");
    write_config(&atuin_path, &atuin_content)?;
    messages.push("Atuin");

    // Lazygit (macOS path)
    let lazygit_content = render_apply_template(config, "apply/lazygit_config.tera")?;
    let lazygit_path = home.join("Library/Application Support/lazygit/config.yml");
    write_config(&lazygit_path, &lazygit_content)?;
    messages.push("Lazygit");

    Ok(format!("Tool configs applied: {}", messages.join(", ")))
}

// ── Git ─────────────────────────────────────────────────────────────

fn apply_git(config: &GhosttyConfig) -> Result<String, String> {
    let delta = &config.git.delta;

    // Set delta config via git config --global
    let settings = vec![
        ("core.pager", "delta".to_string()),
        ("interactive.diffFilter", "delta --color-only".to_string()),
        ("delta.navigate", delta.navigate.to_string()),
        ("delta.dark", delta.dark.to_string()),
        ("delta.line-numbers", delta.line_numbers.to_string()),
        ("delta.side-by-side", delta.side_by_side.to_string()),
        ("delta.syntax-theme", delta.syntax_theme.clone()),
        ("delta.file-style", delta.file_style.clone()),
        ("delta.file-decoration-style", delta.file_decoration_style.clone()),
        ("delta.hunk-header-style", delta.hunk_header_style.clone()),
        ("delta.hunk-header-decoration-style", delta.hunk_header_decoration_style.clone()),
        ("delta.minus-style", delta.minus_style.clone()),
        ("delta.plus-style", delta.plus_style.clone()),
        ("delta.minus-emph-style", delta.minus_emph_style.clone()),
        ("delta.plus-emph-style", delta.plus_emph_style.clone()),
        ("delta.zero-style", delta.zero_style.clone()),
        ("delta.whitespace-error-style", delta.whitespace_error_style.clone()),
        ("merge.conflictstyle", config.git.merge_conflictstyle.clone()),
        ("diff.algorithm", config.git.diff_algorithm.clone()),
        ("diff.colorMoved", config.git.diff_color_moved.clone()),
        ("pull.rebase", config.git.pull_rebase.to_string()),
        ("rebase.autostash", config.git.rebase_autostash.to_string()),
        ("push.autoSetupRemote", config.git.push_auto_setup_remote.to_string()),
        ("push.default", config.git.push_default.clone()),
        ("init.defaultBranch", config.git.init_default_branch.clone()),
        ("core.editor", config.git.core_editor.clone()),
        ("help.autocorrect", config.git.help_autocorrect.to_string()),
    ];

    for (key, value) in &settings {
        let _ = Command::new("git")
            .args(["config", "--global", key, value])
            .output();
    }

    // Set aliases
    for alias in &config.git.aliases {
        if alias.enabled {
            let key = format!("alias.{}", alias.name);
            let _ = Command::new("git")
                .args(["config", "--global", &key, &alias.command])
                .output();
        }
    }

    Ok("Git config applied (immediate effect)".to_string())
}

// ── Themes (bat, btop, yazi) ────────────────────────────────────────

fn apply_themes(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let mut messages = Vec::new();

    // bat theme
    if config.themes.bat_theme_enabled {
        let bat_content = render_apply_template(config, "apply/bat_theme.tera")?;
        // Get bat config dir
        let bat_dir = Command::new("bat")
            .arg("--config-dir")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| PathBuf::from(s.trim()))
            .unwrap_or_else(|| home.join(".config/bat"));
        let bat_path = bat_dir.join("themes/Ayu Dark.tmTheme");
        write_config(&bat_path, &bat_content)?;
        // Post-hook: rebuild bat cache
        let _ = Command::new("bat").args(["cache", "--build"]).output();
        messages.push("bat");
    }

    // btop
    let btop_content = render_apply_template(config, "apply/btop_config.tera")?;
    let btop_path = home.join(".config/btop/btop.conf");
    write_config(&btop_path, &btop_content)?;
    messages.push("btop");

    // yazi theme
    if config.themes.yazi_theme_enabled {
        let yazi_content = render_apply_template(config, "apply/yazi_theme.tera")?;
        let yazi_path = home.join(".config/yazi/theme.toml");
        write_config(&yazi_path, &yazi_content)?;
        messages.push("yazi theme");
    }

    Ok(format!("Themes applied: {}", messages.join(", ")))
}

// ── Fastfetch ───────────────────────────────────────────────────────

fn apply_fastfetch(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let ff_dir = home.join(".config/fastfetch");

    // Write quotes file
    let quotes_content = config.fastfetch.quotes.join("\n");
    let quotes_path = ff_dir.join("quotes.txt");
    write_config(&quotes_path, &quotes_content)?;

    // Write config.jsonc
    let content = render_apply_template(config, "apply/fastfetch_config.tera")?;
    let config_path = ff_dir.join("config.jsonc");
    write_config(&config_path, &content)?;

    Ok("Fastfetch config applied".to_string())
}

// ── Zsh Plugins (yazi config only) ──────────────────────────────────

fn apply_zsh_plugins(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;

    // Write yazi config
    let content = render_apply_template(config, "apply/yazi_config.tera")?;
    let path = home.join(".config/yazi/yazi.toml");
    write_config(&path, &content)?;

    Ok("Yazi config applied".to_string())
}

// ── Zshrc ───────────────────────────────────────────────────────────

fn apply_zshrc(config: &GhosttyConfig) -> Result<String, String> {
    let home = home_dir()?;
    let zshrc_path = home.join(".zshrc");

    let new_block = render_apply_template(config, "apply/zshrc_config.tera")?;

    let start_marker = "# ══ GHOSTTY ULTIMATE ══════════════════════════════════════════";
    let end_marker = "# ══ END GHOSTTY ULTIMATE ═════════════════════════════════════";

    // Read existing .zshrc if it exists
    let existing = fs::read_to_string(&zshrc_path).unwrap_or_default();

    let final_content = if let (Some(start_idx), Some(end_idx)) = (
        existing.find(start_marker),
        existing.find(end_marker),
    ) {
        // Replace existing Ghostty Ultimate block
        let before = &existing[..start_idx];
        let after_end = end_idx + end_marker.len();
        let after = if after_end < existing.len() {
            &existing[after_end..]
        } else {
            ""
        };
        format!("{}{}{}", before, new_block, after)
    } else {
        // Append to end
        if existing.is_empty() {
            new_block
        } else {
            format!("{}\n\n{}", existing.trim_end(), new_block)
        }
    };

    write_config(&zshrc_path, &final_content)?;
    Ok("Zshrc applied (run 'source ~/.zshrc' to reload)".to_string())
}
