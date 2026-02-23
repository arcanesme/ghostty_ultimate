use serde::{Deserialize, Serialize};

// ── Section 1: Clean Artifacts ──────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanArtifactsConfig {
    pub enabled: bool,
    pub backup_configs: bool,
    pub clean_nvim_state: bool,
    pub remove_catppuccin: bool,
    pub clean_zshrc: bool,
    pub remove_legacy_scripts: bool,
}

// ── Section 2: Packages ─────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagesConfig {
    pub enabled: bool,
    pub packages: Vec<PackageEntry>,
    pub install_jetbrains_font: bool,
    pub install_nerd_font: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageEntry {
    pub name: String,
    pub enabled: bool,
    pub category: String,
}

// ── Section 3: Ghostty Terminal ─────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhosttyTermConfig {
    pub enabled: bool,
    // Font
    pub font_family: String,
    pub font_family_bold: String,
    pub font_family_italic: String,
    pub font_family_bold_italic: String,
    pub font_size: f32,
    pub font_style: String,
    pub font_style_bold: String,
    pub font_style_italic: String,
    pub font_synthetic_style: String,
    pub font_features: Vec<String>,
    pub font_variation: Vec<String>,
    pub font_thicken: bool,
    pub font_thicken_strength: u8,
    pub font_shaping_break: String,
    pub freetype_load_flags: String,
    // Window
    pub window_padding_x: u32,
    pub window_padding_y: u32,
    pub window_padding_balance: bool,
    pub window_padding_color: String,
    pub window_decoration: String,
    pub window_colorspace: String,
    pub window_vsync: bool,
    pub window_save_state: String,
    pub window_step_resize: bool,
    pub window_new_tab_position: String,
    pub window_inherit_working_directory: bool,
    pub window_inherit_font_size: bool,
    pub maximize: bool,
    pub fullscreen: bool,
    // macOS
    pub macos_titlebar_style: String,
    pub macos_titlebar_proxy_icon: String,
    pub macos_window_shadow: bool,
    pub macos_option_as_alt: bool,
    // Background & Transparency
    pub background_opacity: f32,
    pub background_blur: u32,
    pub unfocused_split_opacity: f32,
    pub window_opacity: f32,
    pub minimum_contrast: f32,
    pub bold_is_bright: bool,
    // Cursor
    pub cursor_style: String,
    pub cursor_style_blink: Option<bool>,
    pub cursor_opacity: f32,
    pub cursor_click_to_move: bool,
    // Cell
    pub adjust_cell_height: String,
    pub adjust_cell_width: String,
    pub adjust_font_baseline: String,
    pub adjust_underline_position: String,
    pub adjust_underline_thickness: String,
    pub adjust_cursor_thickness: String,
    pub grapheme_width_method: String,
    // Terminal
    pub scrollback_limit: u32,
    pub shell_integration: String,
    pub clipboard_read: String,
    pub clipboard_write: String,
    pub clipboard_trim_trailing_spaces: bool,
    pub clipboard_paste_protection: bool,
    pub mouse_hide_while_typing: bool,
    pub mouse_scroll_multiplier: f32,
    pub focus_follows_mouse: bool,
    pub link_url: bool,
    pub confirm_close_surface: String,
    pub title: String,
    pub image_storage_limit: u32,
    // Shaders
    pub custom_shader_enabled: bool,
    pub custom_shader_path: String,
    pub install_community_shaders: bool,
    // Keybinds
    pub keybinds: Vec<KeyBind>,
    // Quick Terminal
    pub quick_terminal_position: String,
    pub quick_terminal_size: String,
    pub quick_terminal_animation_duration: f32,
    pub quick_terminal_autohide: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBind {
    pub key: String,
    pub action: String,
}

// ── Section 4: Starship ─────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarshipConfig {
    pub enabled: bool,
    pub add_newline: bool,
    pub format: String,
    pub command_timeout: u32,
    // Module toggles
    pub modules: Vec<StarshipModule>,
    // Character
    pub character_success_symbol: String,
    pub character_error_symbol: String,
    pub character_vicmd_symbol: String,
    // Directory
    pub directory_style: String,
    pub directory_format: String,
    pub directory_truncation_length: u32,
    pub directory_truncation_symbol: String,
    // Git branch
    pub git_branch_symbol: String,
    pub git_branch_style: String,
    pub git_branch_format: String,
    // Git status
    pub git_status_style: String,
    pub git_status_format: String,
    // Cmd duration
    pub cmd_duration_min_time: u32,
    pub cmd_duration_style: String,
    pub cmd_duration_format: String,
    // Time
    pub time_disabled: bool,
    pub time_style: String,
    pub time_format: String,
    pub time_time_format: String,
    // Fill
    pub fill_symbol: String,
    // Language modules
    pub nodejs_symbol: String,
    pub nodejs_style: String,
    pub python_symbol: String,
    pub python_style: String,
    pub rust_symbol: String,
    pub rust_style: String,
    pub golang_symbol: String,
    pub golang_style: String,
    pub docker_symbol: String,
    pub docker_style: String,
    pub docker_only_with_files: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarshipModule {
    pub name: String,
    pub enabled: bool,
}

// ── Section 5: Fastfetch ────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastfetchConfig {
    pub enabled: bool,
    pub logo_type: String,
    pub logo_color_1: String,
    pub logo_color_2: String,
    pub separator: String,
    pub key_width: u32,
    pub modules: Vec<FastfetchModule>,
    pub quotes: Vec<String>,
    pub show_color_circles: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastfetchModule {
    #[serde(rename = "type")]
    pub module_type: String,
    pub key: Option<String>,
    pub format: Option<String>,
    pub text: Option<String>,
    pub string: Option<String>,
    pub enabled: bool,
}

// ── Section 6: Zsh Plugins + Yazi ───────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZshPluginsConfig {
    pub enabled: bool,
    pub plugins: Vec<ZshPlugin>,
    pub yazi: YaziConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZshPlugin {
    pub name: String,
    pub repo: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YaziConfig {
    pub ratio: [u32; 3],
    pub sort_by: String,
    pub sort_sensitive: bool,
    pub sort_reverse: bool,
    pub sort_dir_first: bool,
    pub show_hidden: bool,
    pub show_symlink: bool,
    pub scrolloff: u32,
    pub mouse_events: Vec<String>,
    pub linemode: String,
    pub title_format: String,
    pub preview_tab_size: u32,
    pub preview_max_width: u32,
    pub preview_max_height: u32,
    pub preview_image_filter: String,
    pub preview_image_quality: u32,
}

// ── Section 7: tmux ─────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmuxConfig {
    pub enabled: bool,
    pub prefix_key: String,
    pub default_terminal: String,
    pub mouse: bool,
    pub history_limit: u32,
    pub base_index: u32,
    pub pane_base_index: u32,
    pub renumber_windows: bool,
    pub set_clipboard: bool,
    pub escape_time: u32,
    pub focus_events: bool,
    pub status_position: String,
    pub status_left_length: u32,
    pub status_right_length: u32,
    pub mode_keys: String,
    pub pane_border_lines: String,
    pub pane_border_indicators: String,
    // Plugins
    pub plugins: Vec<TmuxPlugin>,
    // Floax
    pub floax_width: String,
    pub floax_height: String,
    pub floax_border_color: String,
    pub floax_text_color: String,
    pub floax_bind: String,
    pub floax_change_path: bool,
    // SessionX
    pub sessionx_bind: String,
    pub sessionx_window_height: String,
    pub sessionx_window_width: String,
    pub sessionx_zoxide_mode: bool,
    pub sessionx_filter_current: bool,
    // Continuum/Resurrect
    pub continuum_restore: bool,
    pub resurrect_strategy_nvim: String,
    pub resurrect_capture_pane_contents: bool,
    // Popup bindings
    pub popup_bindings: Vec<PopupBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmuxPlugin {
    pub name: String,
    pub repo: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupBinding {
    pub key: String,
    pub command: String,
    pub width: String,
    pub height: String,
}

// ── Section 8: Neovim ───────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeovimConfig {
    pub enabled: bool,
    pub theme: String,
    pub theme_bg_override: String,
    pub leader_key: String,
    pub number: bool,
    pub relative_number: bool,
    pub cursorline: bool,
    pub scrolloff: u32,
    pub expandtab: bool,
    pub shiftwidth: u32,
    pub tabstop: u32,
    pub smartindent: bool,
    pub mouse: String,
    pub clipboard: String,
    pub undofile: bool,
    pub ignorecase: bool,
    pub smartcase: bool,
    pub signcolumn: String,
    pub termguicolors: bool,
    pub plugins: Vec<NvimPlugin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvimPlugin {
    pub name: String,
    pub enabled: bool,
    pub description: String,
}

// ── Section 9: Tool Configs ─────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfigsConfig {
    pub enabled: bool,
    pub atuin: AtuinConfig,
    pub lazygit: LazygitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtuinConfig {
    pub style: String,
    pub inline_height: u32,
    pub show_preview: bool,
    pub filter_mode: String,
    pub filter_mode_shell_up_key_binding: String,
    pub search_mode: String,
    pub show_tabs: bool,
    pub timestamps_enabled: bool,
    pub time_format: String,
    pub exit_mode: String,
    pub sync_records: bool,
    pub store_failed: bool,
    pub secrets_filter: bool,
    pub enter_accept: bool,
    pub keymap_mode: String,
    pub workspaces: bool,
    pub invert: bool,
    pub show_help: bool,
    pub max_preview_height: u32,
    pub prefers_reduced_motion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazygitConfig {
    pub nerd_fonts_version: String,
    pub show_file_icons: bool,
    pub border: String,
    pub mouse_events: bool,
    pub show_command_log: bool,
    pub pager: String,
    pub edit_preset: String,
}

// ── Section 10: Git ─────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub enabled: bool,
    pub delta: DeltaConfig,
    // Git settings (guarded)
    pub merge_conflictstyle: String,
    pub diff_algorithm: String,
    pub diff_color_moved: String,
    pub pull_rebase: bool,
    pub rebase_autostash: bool,
    pub push_auto_setup_remote: bool,
    pub push_default: String,
    pub init_default_branch: String,
    pub core_editor: String,
    pub help_autocorrect: u32,
    pub aliases: Vec<GitAlias>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaConfig {
    pub navigate: bool,
    pub dark: bool,
    pub line_numbers: bool,
    pub side_by_side: bool,
    pub syntax_theme: String,
    pub file_style: String,
    pub file_decoration_style: String,
    pub hunk_header_style: String,
    pub hunk_header_decoration_style: String,
    pub minus_style: String,
    pub plus_style: String,
    pub minus_emph_style: String,
    pub plus_emph_style: String,
    pub zero_style: String,
    pub whitespace_error_style: String,
    pub tabs: u32,
    pub max_line_length: u32,
    pub wrap_max_lines: u32,
    pub true_color: String,
    pub hyperlinks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAlias {
    pub name: String,
    pub command: String,
    pub enabled: bool,
}

// ── Section 11: Themes ──────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemesConfig {
    pub enabled: bool,
    pub btop: BtopConfig,
    pub bat_theme_enabled: bool,
    pub yazi_theme_enabled: bool,
    pub fzf: FzfConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtopConfig {
    pub color_theme: String,
    pub theme_background: bool,
    pub truecolor: bool,
    pub shown_boxes: String,
    pub update_ms: u32,
    pub proc_sorting: String,
    pub proc_tree: bool,
    pub proc_per_core: bool,
    pub rounded_corners: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FzfConfig {
    pub border: String,
    pub prompt: String,
    pub marker: String,
    pub pointer: String,
    pub separator: String,
    pub scrollbar: String,
    pub layout: String,
    pub height: String,
}

// ── Section 12: tmux Layout Scripts ─────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmuxScriptsConfig {
    pub enabled: bool,
    pub tmux_ai_enabled: bool,
    pub tmux_pair_enabled: bool,
    pub tmux_review_enabled: bool,
    pub tmux_dev_enabled: bool,
    pub tmux_cheat_enabled: bool,
}

// ── Section 13: Zshrc ───────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZshrcConfig {
    pub enabled: bool,
    pub aliases: Vec<AliasEntry>,
    pub functions: Vec<FunctionEntry>,
    pub histsize: u32,
    pub savehist: u32,
    pub editor: String,
    pub vi_mode_enabled: bool,
    pub vi_mode_escape_key: String,
    pub autosuggest_style: String,
    pub autosuggest_strategy: Vec<String>,
    pub show_fastfetch_on_start: bool,
    pub show_tmux_cheat_on_start: bool,
    pub bat_theme: String,
    pub manpager_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasEntry {
    pub name: String,
    pub command: String,
    pub category: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionEntry {
    pub name: String,
    pub body: String,
    pub description: String,
    pub enabled: bool,
}
