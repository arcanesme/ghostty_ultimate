use crate::config::schema::GhosttyConfig;
use tera::{Tera, Context};
use super::filters;

fn create_tera() -> Result<Tera, String> {
    let mut tera = Tera::default();

    // Register custom filters
    tera.register_filter("bool_to_on_off", filters::bool_to_on_off);
    tera.register_filter("bool_to_str", filters::bool_to_str);
    tera.register_filter("shell_escape", filters::shell_escape);
    tera.register_filter("hex_strip", filters::hex_strip);

    // Add all templates as raw strings
    let templates = get_all_templates();
    for (name, content) in &templates {
        tera.add_raw_template(name, content)
            .map_err(|e| format!("Template '{}' error: {}", name, e))?;
    }

    // Add apply (config-only) templates
    let apply_templates = get_apply_templates();
    for (name, content) in &apply_templates {
        tera.add_raw_template(name, content)
            .map_err(|e| format!("Apply template '{}' error: {}", name, e))?;
    }

    Ok(tera)
}

fn build_context(config: &GhosttyConfig) -> Result<Context, String> {
    let mut ctx = Context::new();
    let json = serde_json::to_value(config).map_err(|e| e.to_string())?;
    ctx.insert("config", &json);
    // Also insert palette at top level for convenience
    let palette_json = serde_json::to_value(&config.palette).map_err(|e| e.to_string())?;
    ctx.insert("palette", &palette_json);
    Ok(ctx)
}

pub fn generate(config: &GhosttyConfig) -> Result<String, String> {
    let tera = create_tera()?;
    let ctx = build_context(config)?;
    tera.render("main.sh.tera", &ctx).map_err(|e| format!("Render error: {}", e))
}

pub fn preview_section(config: &GhosttyConfig, section: &str) -> Result<String, String> {
    let tera = create_tera()?;
    let ctx = build_context(config)?;
    let template_name = format!("{}.sh.tera", section);
    tera.render(&template_name, &ctx).map_err(|e| format!("Render error: {}", e))
}

/// Render a config-only apply template (used by applier module).
pub fn render_apply_template(config: &GhosttyConfig, template_name: &str) -> Result<String, String> {
    let tera = create_tera()?;
    let ctx = build_context(config)?;
    tera.render(template_name, &ctx).map_err(|e| format!("Render error: {}", e))
}

fn get_all_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("main.sh.tera", include_str!("templates/main.sh.tera")),
        ("header.sh.tera", include_str!("templates/header.sh.tera")),
        ("clean_artifacts.sh.tera", include_str!("templates/clean_artifacts.sh.tera")),
        ("packages.sh.tera", include_str!("templates/packages.sh.tera")),
        ("ghostty.sh.tera", include_str!("templates/ghostty.sh.tera")),
        ("starship.sh.tera", include_str!("templates/starship.sh.tera")),
        ("fastfetch.sh.tera", include_str!("templates/fastfetch.sh.tera")),
        ("zsh_plugins.sh.tera", include_str!("templates/zsh_plugins.sh.tera")),
        ("tmux.sh.tera", include_str!("templates/tmux.sh.tera")),
        ("neovim.sh.tera", include_str!("templates/neovim.sh.tera")),
        ("tool_configs.sh.tera", include_str!("templates/tool_configs.sh.tera")),
        ("git.sh.tera", include_str!("templates/git.sh.tera")),
        ("themes.sh.tera", include_str!("templates/themes.sh.tera")),
        ("tmux_scripts.sh.tera", include_str!("templates/tmux_scripts.sh.tera")),
        ("zshrc.sh.tera", include_str!("templates/zshrc.sh.tera")),
        ("done.sh.tera", include_str!("templates/done.sh.tera")),
    ]
}

fn get_apply_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("apply/ghostty_config.tera", include_str!("templates/apply/ghostty_config.tera")),
        ("apply/ghostty_theme.tera", include_str!("templates/apply/ghostty_theme.tera")),
        ("apply/starship_config.tera", include_str!("templates/apply/starship_config.tera")),
        ("apply/tmux_config.tera", include_str!("templates/apply/tmux_config.tera")),
        ("apply/neovim_config.tera", include_str!("templates/apply/neovim_config.tera")),
        ("apply/atuin_config.tera", include_str!("templates/apply/atuin_config.tera")),
        ("apply/lazygit_config.tera", include_str!("templates/apply/lazygit_config.tera")),
        ("apply/bat_theme.tera", include_str!("templates/apply/bat_theme.tera")),
        ("apply/btop_config.tera", include_str!("templates/apply/btop_config.tera")),
        ("apply/yazi_theme.tera", include_str!("templates/apply/yazi_theme.tera")),
        ("apply/yazi_config.tera", include_str!("templates/apply/yazi_config.tera")),
        ("apply/fastfetch_config.tera", include_str!("templates/apply/fastfetch_config.tera")),
        ("apply/zshrc_config.tera", include_str!("templates/apply/zshrc_config.tera")),
    ]
}
