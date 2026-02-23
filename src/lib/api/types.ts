// Mirror types for Rust config structs — auto-generated from plan schemas

export interface AyuPalette {
  bg: string;
  fg: string;
  surface: string;
  selection: string;
  comment: string;
  gutter: string;
  red: string;
  green: string;
  yellow: string;
  blue: string;
  purple: string;
  cyan: string;
  orange: string;
  br_red: string;
  br_green: string;
  br_blue: string;
  br_purple: string;
  br_cyan: string;
}

export interface NamedPalette {
  name: string;
  palette: AyuPalette;
}

export interface GhosttyConfig {
  palette: AyuPalette;
  clean_artifacts: CleanArtifactsConfig;
  packages: PackagesConfig;
  ghostty: GhosttyTermConfig;
  starship: StarshipConfig;
  fastfetch: FastfetchConfig;
  zsh_plugins: ZshPluginsConfig;
  tmux: TmuxConfig;
  neovim: NeovimConfig;
  tool_configs: ToolConfigsConfig;
  git: GitConfig;
  themes: ThemesConfig;
  tmux_scripts: TmuxScriptsConfig;
  zshrc: ZshrcConfig;
}

// Section 1
export interface CleanArtifactsConfig {
  enabled: boolean;
  backup_configs: boolean;
  clean_nvim_state: boolean;
  remove_catppuccin: boolean;
  clean_zshrc: boolean;
  remove_legacy_scripts: boolean;
}

// Section 2
export interface PackagesConfig {
  enabled: boolean;
  packages: PackageEntry[];
  install_jetbrains_font: boolean;
  install_nerd_font: boolean;
}

export interface PackageEntry {
  name: string;
  enabled: boolean;
  category: string;
}

// Section 3
export interface GhosttyTermConfig {
  enabled: boolean;
  font_family: string;
  font_family_bold: string;
  font_family_italic: string;
  font_family_bold_italic: string;
  font_size: number;
  font_style: string;
  font_style_bold: string;
  font_style_italic: string;
  font_synthetic_style: string;
  font_features: string[];
  font_variation: string[];
  font_thicken: boolean;
  font_thicken_strength: number;
  font_shaping_break: string;
  freetype_load_flags: string;
  window_padding_x: number;
  window_padding_y: number;
  window_padding_balance: boolean;
  window_padding_color: string;
  window_decoration: string;
  window_colorspace: string;
  window_vsync: boolean;
  window_save_state: string;
  window_step_resize: boolean;
  window_new_tab_position: string;
  window_inherit_working_directory: boolean;
  window_inherit_font_size: boolean;
  maximize: boolean;
  fullscreen: boolean;
  macos_titlebar_style: string;
  macos_titlebar_proxy_icon: string;
  macos_window_shadow: boolean;
  macos_option_as_alt: boolean;
  background_opacity: number;
  background_blur: number;
  unfocused_split_opacity: number;
  window_opacity: number;
  minimum_contrast: number;
  bold_is_bright: boolean;
  cursor_style: string;
  cursor_style_blink: boolean | null;
  cursor_opacity: number;
  cursor_click_to_move: boolean;
  adjust_cell_height: string;
  adjust_cell_width: string;
  adjust_font_baseline: string;
  adjust_underline_position: string;
  adjust_underline_thickness: string;
  adjust_cursor_thickness: string;
  grapheme_width_method: string;
  scrollback_limit: number;
  shell_integration: string;
  clipboard_read: string;
  clipboard_write: string;
  clipboard_trim_trailing_spaces: boolean;
  clipboard_paste_protection: boolean;
  mouse_hide_while_typing: boolean;
  mouse_scroll_multiplier: number;
  focus_follows_mouse: boolean;
  link_url: boolean;
  confirm_close_surface: string;
  title: string;
  image_storage_limit: number;
  custom_shader_enabled: boolean;
  custom_shader_path: string;
  install_community_shaders: boolean;
  keybinds: KeyBind[];
  quick_terminal_position: string;
  quick_terminal_size: string;
  quick_terminal_animation_duration: number;
  quick_terminal_autohide: boolean;
}

export interface KeyBind {
  key: string;
  action: string;
}

// Section 4
export interface StarshipConfig {
  enabled: boolean;
  add_newline: boolean;
  format: string;
  command_timeout: number;
  modules: StarshipModule[];
  character_success_symbol: string;
  character_error_symbol: string;
  character_vicmd_symbol: string;
  directory_style: string;
  directory_format: string;
  directory_truncation_length: number;
  directory_truncation_symbol: string;
  git_branch_symbol: string;
  git_branch_style: string;
  git_branch_format: string;
  git_status_style: string;
  git_status_format: string;
  cmd_duration_min_time: number;
  cmd_duration_style: string;
  cmd_duration_format: string;
  time_disabled: boolean;
  time_style: string;
  time_format: string;
  time_time_format: string;
  fill_symbol: string;
  nodejs_symbol: string;
  nodejs_style: string;
  python_symbol: string;
  python_style: string;
  rust_symbol: string;
  rust_style: string;
  golang_symbol: string;
  golang_style: string;
  docker_symbol: string;
  docker_style: string;
  docker_only_with_files: boolean;
}

export interface StarshipModule {
  name: string;
  enabled: boolean;
}

// Section 5
export interface FastfetchConfig {
  enabled: boolean;
  logo_type: string;
  logo_color_1: string;
  logo_color_2: string;
  separator: string;
  key_width: number;
  modules: FastfetchModule[];
  quotes: string[];
  show_color_circles: boolean;
}

export interface FastfetchModule {
  type: string;
  key: string | null;
  format: string | null;
  text: string | null;
  string: string | null;
  enabled: boolean;
}

// Section 6
export interface ZshPluginsConfig {
  enabled: boolean;
  plugins: ZshPlugin[];
  yazi: YaziConfig;
}

export interface ZshPlugin {
  name: string;
  repo: string;
  enabled: boolean;
}

export interface YaziConfig {
  ratio: [number, number, number];
  sort_by: string;
  sort_sensitive: boolean;
  sort_reverse: boolean;
  sort_dir_first: boolean;
  show_hidden: boolean;
  show_symlink: boolean;
  scrolloff: number;
  mouse_events: string[];
  linemode: string;
  title_format: string;
  preview_tab_size: number;
  preview_max_width: number;
  preview_max_height: number;
  preview_image_filter: string;
  preview_image_quality: number;
}

// Section 7
export interface TmuxConfig {
  enabled: boolean;
  prefix_key: string;
  default_terminal: string;
  mouse: boolean;
  history_limit: number;
  base_index: number;
  pane_base_index: number;
  renumber_windows: boolean;
  set_clipboard: boolean;
  escape_time: number;
  focus_events: boolean;
  status_position: string;
  status_left_length: number;
  status_right_length: number;
  mode_keys: string;
  pane_border_lines: string;
  pane_border_indicators: string;
  plugins: TmuxPlugin[];
  floax_width: string;
  floax_height: string;
  floax_border_color: string;
  floax_text_color: string;
  floax_bind: string;
  floax_change_path: boolean;
  sessionx_bind: string;
  sessionx_window_height: string;
  sessionx_window_width: string;
  sessionx_zoxide_mode: boolean;
  sessionx_filter_current: boolean;
  continuum_restore: boolean;
  resurrect_strategy_nvim: string;
  resurrect_capture_pane_contents: boolean;
  popup_bindings: PopupBinding[];
}

export interface TmuxPlugin {
  name: string;
  repo: string;
  enabled: boolean;
}

export interface PopupBinding {
  key: string;
  command: string;
  width: string;
  height: string;
}

// Section 8
export interface NeovimConfig {
  enabled: boolean;
  theme: string;
  theme_bg_override: string;
  leader_key: string;
  number: boolean;
  relative_number: boolean;
  cursorline: boolean;
  scrolloff: number;
  expandtab: boolean;
  shiftwidth: number;
  tabstop: number;
  smartindent: boolean;
  mouse: string;
  clipboard: string;
  undofile: boolean;
  ignorecase: boolean;
  smartcase: boolean;
  signcolumn: string;
  termguicolors: boolean;
  plugins: NvimPlugin[];
}

export interface NvimPlugin {
  name: string;
  enabled: boolean;
  description: string;
}

// Section 9
export interface ToolConfigsConfig {
  enabled: boolean;
  atuin: AtuinConfig;
  lazygit: LazygitConfig;
}

export interface AtuinConfig {
  style: string;
  inline_height: number;
  show_preview: boolean;
  filter_mode: string;
  filter_mode_shell_up_key_binding: string;
  search_mode: string;
  show_tabs: boolean;
  timestamps_enabled: boolean;
  time_format: string;
  exit_mode: string;
  sync_records: boolean;
  store_failed: boolean;
  secrets_filter: boolean;
  enter_accept: boolean;
  keymap_mode: string;
  workspaces: boolean;
  invert: boolean;
  show_help: boolean;
  max_preview_height: number;
  prefers_reduced_motion: boolean;
}

export interface LazygitConfig {
  nerd_fonts_version: string;
  show_file_icons: boolean;
  border: string;
  mouse_events: boolean;
  show_command_log: boolean;
  pager: string;
  edit_preset: string;
}

// Section 10
export interface GitConfig {
  enabled: boolean;
  delta: DeltaConfig;
  merge_conflictstyle: string;
  diff_algorithm: string;
  diff_color_moved: string;
  pull_rebase: boolean;
  rebase_autostash: boolean;
  push_auto_setup_remote: boolean;
  push_default: string;
  init_default_branch: string;
  core_editor: string;
  help_autocorrect: number;
  aliases: GitAlias[];
}

export interface DeltaConfig {
  navigate: boolean;
  dark: boolean;
  line_numbers: boolean;
  side_by_side: boolean;
  syntax_theme: string;
  file_style: string;
  file_decoration_style: string;
  hunk_header_style: string;
  hunk_header_decoration_style: string;
  minus_style: string;
  plus_style: string;
  minus_emph_style: string;
  plus_emph_style: string;
  zero_style: string;
  whitespace_error_style: string;
  tabs: number;
  max_line_length: number;
  wrap_max_lines: number;
  true_color: string;
  hyperlinks: boolean;
}

export interface GitAlias {
  name: string;
  command: string;
  enabled: boolean;
}

// Section 11
export interface ThemesConfig {
  enabled: boolean;
  btop: BtopConfig;
  bat_theme_enabled: boolean;
  yazi_theme_enabled: boolean;
  fzf: FzfConfig;
}

export interface BtopConfig {
  color_theme: string;
  theme_background: boolean;
  truecolor: boolean;
  shown_boxes: string;
  update_ms: number;
  proc_sorting: string;
  proc_tree: boolean;
  proc_per_core: boolean;
  rounded_corners: boolean;
}

export interface FzfConfig {
  border: string;
  prompt: string;
  marker: string;
  pointer: string;
  separator: string;
  scrollbar: string;
  layout: string;
  height: string;
}

// Section 12
export interface TmuxScriptsConfig {
  enabled: boolean;
  tmux_ai_enabled: boolean;
  tmux_pair_enabled: boolean;
  tmux_review_enabled: boolean;
  tmux_dev_enabled: boolean;
  tmux_cheat_enabled: boolean;
}

// Section 13
export interface ZshrcConfig {
  enabled: boolean;
  aliases: AliasEntry[];
  functions: FunctionEntry[];
  histsize: number;
  savehist: number;
  editor: string;
  vi_mode_enabled: boolean;
  vi_mode_escape_key: string;
  autosuggest_style: string;
  autosuggest_strategy: string[];
  show_fastfetch_on_start: boolean;
  show_tmux_cheat_on_start: boolean;
  bat_theme: string;
  manpager_enabled: boolean;
}

export interface AliasEntry {
  name: string;
  command: string;
  category: string;
  enabled: boolean;
}

export interface FunctionEntry {
  name: string;
  body: string;
  description: string;
  enabled: boolean;
}

// Execution events
export interface ExecutionEvent {
  event_type: "progress" | "log" | "error" | "complete";
  section: number | null;
  total_sections: number | null;
  message: string;
  timestamp: number;
}

// Section metadata for sidebar navigation
export interface SectionInfo {
  id: string;
  label: string;
  icon: string;
  number: number;
}

export const SECTIONS: SectionInfo[] = [
  { id: "clean_artifacts", label: "Clean Artifacts", icon: "🧹", number: 1 },
  { id: "packages", label: "Packages", icon: "📦", number: 2 },
  { id: "ghostty", label: "Ghostty", icon: "👻", number: 3 },
  { id: "starship", label: "Starship", icon: "🚀", number: 4 },
  { id: "fastfetch", label: "Fastfetch", icon: "📊", number: 5 },
  { id: "zsh_plugins", label: "Zsh Plugins", icon: "🧩", number: 6 },
  { id: "tmux", label: "tmux", icon: "🖥️", number: 7 },
  { id: "neovim", label: "Neovim", icon: "✏️", number: 8 },
  { id: "tool_configs", label: "Tool Configs", icon: "⚙️", number: 9 },
  { id: "git", label: "Git", icon: "🔀", number: 10 },
  { id: "themes", label: "Themes", icon: "🎨", number: 11 },
  { id: "tmux_scripts", label: "tmux Scripts", icon: "📜", number: 12 },
  { id: "zshrc", label: "Zshrc", icon: "🏠", number: 13 },
];
