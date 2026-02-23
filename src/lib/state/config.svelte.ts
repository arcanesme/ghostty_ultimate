import type { GhosttyConfig, AyuPalette } from "$lib/api/types";
import { getDefaultConfig, applyConfig } from "$lib/api/invoke";
import { uiState } from "$lib/state/ui.svelte";

// Deep clone helper
function clone<T>(obj: T): T {
  return JSON.parse(JSON.stringify(obj));
}

class ConfigState {
  config = $state<GhosttyConfig>(getPlaceholderConfig());
  loaded = $state(false);
  dirty = $state(false);

  // Live Apply state
  autoApply = $state(false);
  applyStatus = $state<"idle" | "applying" | "success" | "error">("idle");
  lastApplyMessage = $state("");

  private debounceTimers: Map<string, ReturnType<typeof setTimeout>> = new Map();

  async init() {
    try {
      this.config = await getDefaultConfig();
    } catch {
      // Use placeholder if Tauri not available (dev mode)
      this.config = getPlaceholderConfig();
    }
    this.loaded = true;
    this.dirty = false;
  }

  updatePalette(palette: AyuPalette) {
    this.config.palette = palette;
    this.dirty = true;
  }

  updateSection<K extends keyof GhosttyConfig>(
    section: K,
    value: GhosttyConfig[K],
  ) {
    this.config[section] = value;
    this.dirty = true;
  }

  updateField<K extends keyof GhosttyConfig>(
    section: K,
    field: string,
    value: unknown,
  ) {
    const s = this.config[section] as Record<string, unknown>;
    s[field] = value;
    this.config[section] = { ...s } as GhosttyConfig[K];
    this.dirty = true;
  }

  loadConfig(config: GhosttyConfig) {
    this.config = clone(config);
    this.dirty = false;
  }

  resetToDefaults() {
    this.init();
  }

  // Debounced apply — call this from section components when autoApply is on
  debouncedApply(section: string, delay = 500) {
    if (!this.autoApply) return;

    const existing = this.debounceTimers.get(section);
    if (existing) clearTimeout(existing);

    const timer = setTimeout(() => {
      this.debounceTimers.delete(section);
      this.applySection(section);
    }, delay);
    this.debounceTimers.set(section, timer);
  }

  // Apply a specific section's config to disk
  async applySection(section: string) {
    this.applyStatus = "applying";
    try {
      const message = await applyConfig(this.config, section);
      this.applyStatus = "success";
      this.lastApplyMessage = message;
      uiState.toast(message, "success");
    } catch (e) {
      this.applyStatus = "error";
      const msg = `Apply failed: ${e}`;
      this.lastApplyMessage = msg;
      uiState.toast(msg, "error");
    }
  }
}

export const configState = new ConfigState();

// Placeholder config for when Tauri isn't available (pure frontend dev)
function getPlaceholderConfig(): GhosttyConfig {
  return {
    palette: {
      bg: "#0b0e14", fg: "#bfbdb6", surface: "#1e232b", selection: "#1b3a5b",
      comment: "#8b949e", gutter: "#636a72", red: "#ea6c73", green: "#7fd962",
      yellow: "#f9af4f", blue: "#53bdfa", purple: "#cda1fa", cyan: "#90e1c6",
      orange: "#ffb454", br_red: "#f07178", br_green: "#aad94c", br_blue: "#59c2ff",
      br_purple: "#d2a6ff", br_cyan: "#95e6cb",
    },
    clean_artifacts: { enabled: true, backup_configs: true, clean_nvim_state: true, remove_catppuccin: true, clean_zshrc: true, remove_legacy_scripts: true },
    packages: { enabled: true, packages: [], install_jetbrains_font: true, install_nerd_font: true },
    ghostty: { enabled: true, font_family: "JetBrains Mono", font_family_bold: "JetBrains Mono ExtraBold", font_family_italic: "", font_family_bold_italic: "", font_size: 14.5, font_style: "", font_style_bold: "", font_style_italic: "", font_synthetic_style: "true", font_features: ["liga","calt","dlig","ss01","ss02","ss03"], font_variation: [], font_thicken: true, font_thicken_strength: 0, font_shaping_break: "cursor", freetype_load_flags: "no-force-autohint", window_padding_x: 40, window_padding_y: 32, window_padding_balance: true, window_padding_color: "background", window_decoration: "auto", window_colorspace: "display-p3", window_vsync: true, window_save_state: "default", window_step_resize: false, window_new_tab_position: "current", window_inherit_working_directory: false, window_inherit_font_size: false, maximize: false, fullscreen: false, macos_titlebar_style: "transparent", macos_titlebar_proxy_icon: "hidden", macos_window_shadow: true, macos_option_as_alt: true, background_opacity: 0.93, background_blur: 32, unfocused_split_opacity: 0.78, window_opacity: 1.0, minimum_contrast: 1.3, bold_is_bright: true, cursor_style: "bar", cursor_style_blink: true, cursor_opacity: 1.0, cursor_click_to_move: true, adjust_cell_height: "18%", adjust_cell_width: "0", adjust_font_baseline: "0", adjust_underline_position: "0", adjust_underline_thickness: "0", adjust_cursor_thickness: "0", grapheme_width_method: "unicode", scrollback_limit: 50000, shell_integration: "zsh", clipboard_read: "allow", clipboard_write: "allow", clipboard_trim_trailing_spaces: false, clipboard_paste_protection: true, mouse_hide_while_typing: true, mouse_scroll_multiplier: 3.0, focus_follows_mouse: false, link_url: true, confirm_close_surface: "false", title: "", image_storage_limit: 320, custom_shader_enabled: true, custom_shader_path: "~/.config/ghostty/shaders/vignette-bloom.glsl", install_community_shaders: true, keybinds: [], quick_terminal_position: "", quick_terminal_size: "", quick_terminal_animation_duration: 0.0, quick_terminal_autohide: true },
    starship: { enabled: true, add_newline: true, format: "", command_timeout: 1000, modules: [], character_success_symbol: "[❯](blue)", character_error_symbol: "[❯](red)", character_vicmd_symbol: "[❮❮❮](bold yellow)", directory_style: "bold blue", directory_format: "[$path]($style) ", directory_truncation_length: 3, directory_truncation_symbol: "…/", git_branch_symbol: " ", git_branch_style: "fg:#8b949e", git_branch_format: "[$symbol$branch]($style) ", git_status_style: "red", git_status_format: "[$all_status$ahead_behind]($style) ", cmd_duration_min_time: 1000, cmd_duration_style: "yellow", cmd_duration_format: "[✦ $duration]($style) ", time_disabled: false, time_style: "bold fg:#8b949e", time_format: "[$time]($style)", time_time_format: "%I:%M %p", fill_symbol: " ", nodejs_symbol: " ", nodejs_style: "green", python_symbol: " ", python_style: "yellow", rust_symbol: " ", rust_style: "red", golang_symbol: " ", golang_style: "cyan", docker_symbol: " ", docker_style: "blue", docker_only_with_files: true },
    fastfetch: { enabled: true, logo_type: "small", logo_color_1: "34", logo_color_2: "36", separator: "  ", key_width: 10, modules: [], quotes: [], show_color_circles: true },
    zsh_plugins: { enabled: true, plugins: [], yazi: { ratio: [1,4,3], sort_by: "alphabetical", sort_sensitive: false, sort_reverse: false, sort_dir_first: true, show_hidden: false, show_symlink: true, scrolloff: 5, mouse_events: ["click","scroll","touch"], linemode: "none", title_format: "", preview_tab_size: 2, preview_max_width: 800, preview_max_height: 600, preview_image_filter: "lanczos3", preview_image_quality: 75 } },
    tmux: { enabled: true, prefix_key: "C-a", default_terminal: "tmux-256color", mouse: true, history_limit: 50000, base_index: 1, pane_base_index: 1, renumber_windows: true, set_clipboard: true, escape_time: 0, focus_events: true, status_position: "top", status_left_length: 30, status_right_length: 60, mode_keys: "vi", pane_border_lines: "heavy", pane_border_indicators: "both", plugins: [], floax_width: "80%", floax_height: "80%", floax_border_color: "magenta", floax_text_color: "blue", floax_bind: "p", floax_change_path: true, sessionx_bind: "o", sessionx_window_height: "85%", sessionx_window_width: "75%", sessionx_zoxide_mode: true, sessionx_filter_current: false, continuum_restore: true, resurrect_strategy_nvim: "session", resurrect_capture_pane_contents: true, popup_bindings: [] },
    neovim: { enabled: true, theme: "ayu-dark", theme_bg_override: "#0b0e14", leader_key: " ", number: true, relative_number: true, cursorline: true, scrolloff: 8, expandtab: true, shiftwidth: 2, tabstop: 2, smartindent: true, mouse: "a", clipboard: "unnamedplus", undofile: true, ignorecase: true, smartcase: true, signcolumn: "yes", termguicolors: true, plugins: [] },
    tool_configs: { enabled: true, atuin: { style: "full", inline_height: 0, show_preview: true, filter_mode: "global", filter_mode_shell_up_key_binding: "directory", search_mode: "fuzzy", show_tabs: true, timestamps_enabled: true, time_format: "%I:%M %p", exit_mode: "return-original", sync_records: true, store_failed: true, secrets_filter: true, enter_accept: false, keymap_mode: "emacs", workspaces: false, invert: false, show_help: true, max_preview_height: 4, prefers_reduced_motion: false }, lazygit: { nerd_fonts_version: "3", show_file_icons: true, border: "rounded", mouse_events: true, show_command_log: false, pager: "delta --dark --paging=never", edit_preset: "nvim" } },
    git: { enabled: true, delta: { navigate: true, dark: true, line_numbers: true, side_by_side: true, syntax_theme: "ayu-dark", file_style: "bold yellow", file_decoration_style: "yellow ul", hunk_header_style: "syntax bold", hunk_header_decoration_style: "blue box", minus_style: "syntax #2d1517", plus_style: "syntax #152e1a", minus_emph_style: "syntax #4a1c1f", plus_emph_style: "syntax #1e4020", zero_style: "syntax", whitespace_error_style: "reverse red", tabs: 8, max_line_length: 3000, wrap_max_lines: 2, true_color: "auto", hyperlinks: false }, merge_conflictstyle: "diff3", diff_algorithm: "histogram", diff_color_moved: "default", pull_rebase: true, rebase_autostash: true, push_auto_setup_remote: true, push_default: "current", init_default_branch: "main", core_editor: "nvim", help_autocorrect: 20, aliases: [] },
    themes: { enabled: true, btop: { color_theme: "ayu", theme_background: false, truecolor: true, shown_boxes: "cpu mem net proc", update_ms: 1000, proc_sorting: "cpu lazy", proc_tree: false, proc_per_core: false, rounded_corners: true }, bat_theme_enabled: true, yazi_theme_enabled: true, fzf: { border: "rounded", prompt: "❯ ", marker: "✓", pointer: "▶", separator: "─", scrollbar: "│", layout: "reverse", height: "60%" } },
    tmux_scripts: { enabled: true, tmux_ai_enabled: true, tmux_pair_enabled: true, tmux_review_enabled: true, tmux_dev_enabled: true, tmux_cheat_enabled: true },
    zshrc: { enabled: true, aliases: [], functions: [], histsize: 50000, savehist: 50000, editor: "nvim", vi_mode_enabled: true, vi_mode_escape_key: "jk", autosuggest_style: "fg=#8b949e", autosuggest_strategy: ["history","completion"], show_fastfetch_on_start: true, show_tmux_cheat_on_start: true, bat_theme: "Ayu Dark", manpager_enabled: true },
  };
}
