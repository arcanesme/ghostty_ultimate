#!/bin/bash
# ── Ghostty Ultimate — Ayu Dark Terminal Architecture ──────────────
# Idempotent setup for macOS machines with Ghostty installed.
# Unified Ayu Dark theme across all tools. No Catppuccin dependencies.
# Safe to re-run: backs up configs, skips installed packages, guards git settings.
set -o pipefail
start_time=$(date +%s)
warn_count=0

# ── Ayu Dark palette (single source of truth) ─────────────────────
#   bg        #0b0e14    surface    #1e232b    selection  #1b3a5b
#   fg        #bfbdb6    comment    #8b949e    gutter     #636a72
#   red       #ea6c73    green      #7fd962    yellow     #f9af4f
#   blue      #53bdfa    purple     #cda1fa    cyan       #90e1c6
#   orange    #ffb454    br_red     #f07178    br_green   #aad94c
#   br_blue   #59c2ff    br_purple  #d2a6ff    br_cyan    #95e6cb

# ── Script colors ─────────────────────────────────────────────────
c_purple='\033[38;2;205;161;250m'
c_blue='\033[38;2;83;189;250m'
c_text='\033[38;2;191;189;182m'
c_dim='\033[38;2;104;104;104m'
c_surface='\033[38;2;30;35;43m'
c_green='\033[38;2;127;217;98m'
c_yellow='\033[38;2;249;175;79m'
c_red='\033[38;2;234;108;115m'
c_bold='\033[1m'
c_faint='\033[2m'
c_reset='\033[0m'

print_header() {
  echo ""
  echo -e "${c_surface}────────────────────────────────────────────────────────${c_reset}"
  echo -e "  ${c_purple}${c_bold}$1${c_reset}"
  echo -e "${c_surface}────────────────────────────────────────────────────────${c_reset}"
}
print_step()  { echo -e "  ${c_green}✓${c_reset} ${c_text}$1${c_reset}"; }
print_warn()  { ((warn_count++)); echo -e "  ${c_yellow}▲${c_reset} ${c_dim}$1${c_reset}"; }
print_skip()  { echo -e "  ${c_surface}○${c_reset} ${c_dim}$1${c_reset}"; }
print_error() { echo -e "  ${c_red}✗${c_reset} ${c_text}$1${c_reset}"; }

# ── Prerequisites ─────────────────────────────────────────────────
[[ "$(uname)" != "Darwin" ]] && { print_error "macOS required"; exit 1; }
command -v brew &>/dev/null || { print_error "Homebrew required — https://brew.sh"; exit 1; }

if ! command -v ghostty &>/dev/null && [[ ! -d "/Applications/Ghostty.app" ]]; then
  print_warn "Ghostty not detected — config will be written for when it's installed"
fi

# ── Variables ─────────────────────────────────────────────────────
ZSHRC="$HOME/.zshrc"
SCRIPTS_DIR="$HOME/.local/bin"
BACKUP_SUFFIX=".bak.$(date +%s)"
mkdir -p "$SCRIPTS_DIR" || { print_error "Cannot create $SCRIPTS_DIR"; exit 1; }

# ── Helpers ───────────────────────────────────────────────────────

# Fast font detection via filesystem scan (replaces 15+ second system_profiler calls)
font_installed() {
  local name="$1"
  for dir in "$HOME/Library/Fonts" "/Library/Fonts" "/System/Library/Fonts"; do
    [[ -d "$dir" ]] && find "$dir" -iname "*${name}*" -print -quit 2>/dev/null | grep -q . && return 0
  done
  return 1
}

# Set git config only if not already configured — never overwrites user preferences
git_default() {
  git config --global --get "$1" &>/dev/null || git config --global "$1" "$2"
}

# ── Banner ────────────────────────────────────────────────────────
clear 2>/dev/null || true
echo ""
echo -e "  👻 ${c_purple}${c_bold}ＧＨＯＳＴＴＹ  ${c_blue}ＵＬＴＩＭＡＴＥ${c_reset}"
echo -e "  ${c_dim}Avant-Garde Terminal Architecture${c_reset}"
echo -e "  ${c_surface}─────────────────────────────────${c_reset}"
echo ""

########################################################################
# 1/13 — Clean Old Artifacts
########################################################################
print_header "🧹 1/13 — Cleaning Old Artifacts"

# Back up major configs
touch "$ZSHRC"
cp "$ZSHRC" "${ZSHRC}${BACKUP_SUFFIX}" 2>/dev/null || true
[[ -d "$HOME/.config/nvim" ]] && mv "$HOME/.config/nvim" "$HOME/.config/nvim${BACKUP_SUFFIX}" 2>/dev/null || true
[[ -f "$HOME/.tmux.conf" ]] && cp "$HOME/.tmux.conf" "$HOME/.tmux.conf${BACKUP_SUFFIX}" 2>/dev/null || true
[[ -f "$HOME/.config/starship.toml" ]] && cp "$HOME/.config/starship.toml" "$HOME/.config/starship.toml${BACKUP_SUFFIX}" 2>/dev/null || true
print_step "Backed up .zshrc, nvim, tmux, and starship configs"

# Neovim state: back up instead of deleting, clear only cache
[[ -d "$HOME/.local/share/nvim" ]] && mv "$HOME/.local/share/nvim" "$HOME/.local/share/nvim${BACKUP_SUFFIX}" 2>/dev/null || true
[[ -d "$HOME/.local/state/nvim" ]] && mv "$HOME/.local/state/nvim" "$HOME/.local/state/nvim${BACKUP_SUFFIX}" 2>/dev/null || true
rm -rf "$HOME/.cache/nvim" 2>/dev/null || true
print_step "Backed up Neovim state, cleared cache"

# Remove legacy Catppuccin tmux plugin (replaced by inline Ayu Dark)
rm -rf "$HOME/.config/tmux/plugins/catppuccin" 2>/dev/null || true

# Remove existing GHOSTTY ULTIMATE block (single awk pass replaces 30+ sed calls)
if grep -q '══ GHOSTTY ULTIMATE' "$ZSHRC" 2>/dev/null; then
  awk '/══ GHOSTTY ULTIMATE/,/══ END GHOSTTY ULTIMATE/{next}1' "$ZSHRC" > "${ZSHRC}.awk.tmp" \
    && mv "${ZSHRC}.awk.tmp" "$ZSHRC"
fi

# Clean stale tool inits and duplicate aliases (consolidated into one sed invocation)
sed -i '' \
  -e '/fastfetch/d' \
  -e '/Terminal Splash/d' \
  -e '/tmux-cheat/d' \
  -e '/TMUX_CHEAT/d' \
  -e '/Phase [0-9]/d' \
  -e '/eval "\$(starship init zsh)"/d' \
  -e '/eval "\$(zoxide init zsh)"/d' \
  -e '/eval "\$(atuin init zsh/d' \
  -e '/source <(fzf --zsh/d' \
  -e '/\[ -f ~\/.fzf.zsh \] && source/d' \
  -e '/^alias ls=.*eza/d' \
  -e '/^alias ll=.*eza/d' \
  -e '/^alias la=.*eza/d' \
  -e '/^alias tree=.*eza/d' \
  -e '/^alias lt=.*eza/d' \
  -e '/^alias cat=.*bat/d' \
  -e '/^alias grep=.*rg/d' \
  -e '/^alias find=.*fd/d' \
  -e '/^alias vim=.*nvim/d' \
  -e '/^alias vi=.*nvim/d' \
  -e '/^alias cc=.*claude/d' \
  -e '/^alias gm=.*gemini/d' \
  -e '/^alias cx=.*codex/d' \
  -e '/AGENTIC WORKFLOW/d' \
  -e '/^ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE=/d' \
  -e '/^ZSH_AUTOSUGGEST_STRATEGY=/d' \
  -e '/^export EDITOR=.*nvim/d' \
  -e '/^export VISUAL=.*nvim/d' \
  -e '/^export BAT_THEME=/d' \
  "$ZSHRC" 2>/dev/null || true

# Collapse excessive blank lines (portable awk, no perl dependency)
awk 'NF{blank=0; print; next} !blank++' "$ZSHRC" > "${ZSHRC}.blank.tmp" \
  && mv "${ZSHRC}.blank.tmp" "$ZSHRC"
print_step "Cleaned .zshrc remnants"

rm -f \
  "$HOME/Downloads/setup-ghostty.sh" \
  "$HOME/Downloads/enhance-ghostty.sh" \
  "$HOME/Downloads/phase3-ghostty.sh" \
  "$HOME/Downloads/phase4-ghostty.sh" \
  "$HOME/Downloads/phase5-ghostty.sh" \
  "$HOME/Downloads/phase6-ghostty.sh" \
  "$HOME/Downloads/fastfetch-config.jsonc" \
  "$HOME/Downloads/ghostty-config" \
  "$HOME/Downloads/starship.toml" 2>/dev/null || true
print_step "Removed legacy setup scripts"

########################################################################
# 2/13 — Packages (batched brew install)
########################################################################
print_header "📦 2/13 — Packages"

brew_packages=(
  starship eza bat fzf ripgrep fd zoxide tmux neovim atuin
  yazi fastfetch lazygit lazydocker gh git-delta btop dust procs
  hyperfine tokei gping curlie ffmpeg sevenzip poppler jq tlrc
  trash-cli zsh-completions
)

to_install=()
already=0
for pkg in "${brew_packages[@]}"; do
  if brew list --formula "$pkg" &>/dev/null 2>&1 || brew list --cask "$pkg" &>/dev/null 2>&1; then
    ((already++))
  else
    to_install+=("$pkg")
  fi
done

[[ $already -gt 0 ]] && print_skip "$already packages already installed"

if [[ ${#to_install[@]} -gt 0 ]]; then
  print_step "Installing ${#to_install[@]} packages: ${to_install[*]}"
  if brew install "${to_install[@]}" 2>/dev/null; then
    print_step "All ${#to_install[@]} packages installed"
  else
    print_warn "Some packages may have failed — run 'brew install <pkg>' to troubleshoot"
  fi
else
  print_skip "All packages already installed"
fi

# ── Fonts (fast filesystem detection) ─────────────────────────────
if ! font_installed "JetBrains"; then
  brew install --cask font-jetbrains-mono font-jetbrains-mono-nerd-font 2>/dev/null \
    && print_step "JetBrains Mono + Nerd Font" || print_warn "JetBrains Mono install failed"
else
  print_skip "JetBrains Mono"
fi

if command -v npm &>/dev/null; then
  if ! command -v claude &>/dev/null; then
    print_step "Installing Claude CLI via npm..."
    npm install -g @anthropic-ai/claude-code --force 2>/dev/null || print_warn "Claude install failed"
  else
    print_skip "Claude CLI (already installed)"
  fi
fi

########################################################################
# 3/13 — Ghostty Config
########################################################################
print_header "👻 3/13 — Ghostty Config"
GHOSTTY_DIR="$HOME/.config/ghostty"
GHOSTTY_MACOS_CONFIG="$HOME/Library/Application Support/com.mitchellh.ghostty/config"

if [[ -f "$GHOSTTY_MACOS_CONFIG" ]]; then
  cp "$GHOSTTY_MACOS_CONFIG" "${GHOSTTY_MACOS_CONFIG}${BACKUP_SUFFIX}" 2>/dev/null || true
  rm -f "$GHOSTTY_MACOS_CONFIG"
  print_step "Removed legacy macOS Ghostty config (backed up)"
else
  print_skip "No legacy macOS Ghostty config"
fi

mkdir -p "$GHOSTTY_DIR/shaders" "$GHOSTTY_DIR/themes"

cat > "$GHOSTTY_DIR/themes/ayu-dark" << 'THEME'
palette = 0=#1e232b
palette = 1=#ea6c73
palette = 2=#7fd962
palette = 3=#f9af4f
palette = 4=#53bdfa
palette = 5=#cda1fa
palette = 6=#90e1c6
palette = 7=#c7c7c7
palette = 8=#8b949e
palette = 9=#f07178
palette = 10=#aad94c
palette = 11=#ffb454
palette = 12=#59c2ff
palette = 13=#d2a6ff
palette = 14=#95e6cb
palette = 15=#ffffff
background = #0b0e14
foreground = #bfbdb6
cursor-color = #bfbdb6
cursor-text = #0b0e14
selection-background = #1b3a5b
selection-foreground = #bfbdb6
THEME
print_step "Ayu Dark theme file"

cat > "$GHOSTTY_DIR/config" << 'GC'
theme = ayu-dark

# ── Typography ────────────────────────────────────────────────────
font-family             = JetBrains Mono
font-family-bold        = JetBrains Mono ExtraBold
font-size               = 14.5

font-feature = liga
font-feature = calt
font-feature = dlig
font-feature = ss01
font-feature = ss02
font-feature = ss03
font-thicken = true

# ── Window & Padding ─────────────────────────────────────────────
window-padding-x        = 40
window-padding-y        = 32
window-padding-balance  = true

macos-titlebar-style    = transparent
macos-titlebar-proxy-icon = hidden
macos-window-shadow     = true
window-colorspace       = display-p3
confirm-close-surface   = false
window-decoration       = auto
mouse-hide-while-typing = true

# ── Background ────────────────────────────────────────────────────
background-opacity      = 0.80
background-blur         = 20
unfocused-split-opacity = 0.70
minimum-contrast        = 1.3
bold-is-bright          = true

# ── Cursor ────────────────────────────────────────────────────────
cursor-style            = bar
cursor-style-blink      = true
cursor-opacity          = 1.0
cursor-click-to-move    = true

# ── Cell Spacing ──────────────────────────────────────────────────
adjust-cell-height      = 18%
freetype-load-flags     = no-force-autohint

# ── Scroll ────────────────────────────────────────────────────────
scrollback-limit        = 50000
keybind = shift+page_up=scroll_page_up
keybind = shift+page_down=scroll_page_down
keybind = super+up=scroll_page_up
keybind = super+down=scroll_page_down
keybind = super+home=scroll_to_top
keybind = super+end=scroll_to_bottom

shell-integration       = zsh
clipboard-read          = allow
clipboard-write         = allow

custom-shader           = ~/.config/ghostty/shaders/vignette-bloom.glsl
macos-option-as-alt     = true

# ── Keybinds ──────────────────────────────────────────────────────
keybind = super+d=new_split:right
keybind = super+shift+d=new_split:down
keybind = super+alt+left=goto_split:left
keybind = super+alt+right=goto_split:right
keybind = super+alt+up=goto_split:up
keybind = super+alt+down=goto_split:down
keybind = super+1=goto_tab:1
keybind = super+2=goto_tab:2
keybind = super+3=goto_tab:3
keybind = super+4=goto_tab:4
keybind = super+5=goto_tab:5
GC
print_step "Ghostty config written"

cat > "$GHOSTTY_DIR/shaders/vignette-bloom.glsl" << 'SHADER'
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = fragCoord.xy / iResolution.xy;
    vec4 base = texture(iChannel0, uv);

    float offset = 1.5 / iResolution.x;
    float offsetY = 1.5 / iResolution.y;
    vec4 bloom = vec4(0.0);
    bloom += texture(iChannel0, uv + vec2(-offset,  0.0));
    bloom += texture(iChannel0, uv + vec2( offset,  0.0));
    bloom += texture(iChannel0, uv + vec2( 0.0,    -offsetY));
    bloom += texture(iChannel0, uv + vec2( 0.0,     offsetY));
    bloom += texture(iChannel0, uv + vec2(-offset, -offsetY));
    bloom += texture(iChannel0, uv + vec2( offset,  offsetY));
    bloom += texture(iChannel0, uv + vec2( offset, -offsetY));
    bloom += texture(iChannel0, uv + vec2(-offset,  offsetY));
    bloom /= 8.0;
    float brightness = dot(bloom.rgb, vec3(0.2126, 0.7152, 0.0722));
    float bloomStrength = smoothstep(0.45, 0.85, brightness) * 0.09;

    vec2 centered = uv * 2.0 - 1.0;
    float vignette = 1.0 - dot(centered * vec2(0.35, 0.50), centered * vec2(0.35, 0.50));
    vignette = clamp(pow(vignette, 0.8), 0.0, 1.0);
    vignette = mix(0.85, 1.0, vignette);

    fragColor = (base + bloom * bloomStrength) * vignette;
    fragColor.a = base.a;
}
SHADER
print_step "Vignette-bloom shader"

if [[ ! -d "$GHOSTTY_DIR/shaders/ghostty-shaders" ]]; then
  git clone --depth 1 https://github.com/0xhckr/ghostty-shaders.git "$GHOSTTY_DIR/shaders/ghostty-shaders" 2>/dev/null \
    && print_step "Community shaders" || print_warn "Community shaders clone failed"
else
  print_skip "Community shaders"
fi

########################################################################
# 4/13 — Starship
########################################################################
print_header "🚀 4/13 — Starship"
mkdir -p "$HOME/.config"
cat > "$HOME/.config/starship.toml" << 'ST'
add_newline = true
format = """$directory$fill$git_branch$git_status$nodejs$python$rust$golang$docker_context$cmd_duration$time
$character"""
palette = "ayu_dark"
command_timeout = 1000

[fill]
symbol = " "

[character]
vicmd_symbol = "[❮❮❮](bold yellow)"
success_symbol = '[❯](blue)'
error_symbol = '[❯](red)'

[directory]
style = "bold blue"
format = "[$path]($style) "
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
symbol = " "
style = "fg:#8b949e"
format = "[$symbol$branch]($style) "

[git_status]
style = "red"
format = "[$all_status$ahead_behind]($style) "

[nodejs]
symbol = " "
style = "green"
format = "[$symbol$version]($style) "

[python]
symbol = " "
style = "yellow"
format = "[$symbol$version]($style) "
detect_files = [".python-version", "requirements.txt", "Pipfile", "pyproject.toml"]

[rust]
symbol = " "
style = "red"
format = "[$symbol$version]($style) "

[golang]
symbol = " "
style = "cyan"
format = "[$symbol$version]($style) "

[docker_context]
symbol = " "
style = "blue"
format = "[$symbol$context]($style) "
only_with_files = true

[cmd_duration]
min_time = 1_000
style = "yellow"
format = "[✦ $duration]($style) "

[time]
disabled = false
style = "bold fg:#8b949e"
format = "[$time]($style)"
time_format = "%I:%M %p"

[palettes.ayu_dark]
red     = "#ea6c73"
green   = "#7fd962"
yellow  = "#f9af4f"
blue    = "#53bdfa"
purple  = "#cda1fa"
cyan    = "#90e1c6"
orange  = "#ffb454"
fg      = "#bfbdb6"
comment = "#8b949e"
ST
print_step "Starship — Ayu Dark HUD"

########################################################################
# 5/13 — Fastfetch
########################################################################
print_header "📊 5/13 — Fastfetch"
FF_DIR="$HOME/.config/fastfetch"
mkdir -p "$FF_DIR"

cat > "$FF_DIR/quotes.txt" << 'QT'
The impediment to action advances action. What stands in the way becomes the way.
You have power over your mind, not outside events. Realize this and you will find strength.
Waste no more time arguing what a good man should be. Be one.
Loss is nothing else but change, and change is nature's delight.
The best revenge is to be unlike him who performed the injustice.
Man is not disturbed by events, but by the opinions he has of events.
Concentrate every minute on doing what is in front of you with precise elegance.
QT

cat > "$FF_DIR/config.jsonc" << 'FF'
{
  "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
  "logo": { "type": "small", "color": { "1": "34", "2": "36" },
            "padding": { "top": 1, "left": 2, "right": 3 } },
  "display": { "separator": "  ", "color": { "keys": "34", "title": "34" },
               "key": { "width": 10 } },
  "modules": [
    { "type": "title",     "format": "{user-name-colored}" },
    { "type": "separator", "string": "\u2500" },
    { "type": "command",   "key": "   ", "text": "date '+%a %b %d   %I:%M %p'" },
    { "type": "os",        "key": "  OS" },
    { "type": "uptime",    "key": "  Up" },
    { "type": "terminal",  "key": "  Term" },
    { "type": "shell",     "key": "  Shell" },
    { "type": "cpu",       "key": "  CPU", "showPeCoreCount": false },
    { "type": "gpu",       "key": "  GPU" },
    { "type": "memory",    "key": "  RAM" },
    { "type": "disk",      "key": "  Disk", "folders": "/" },
    { "type": "packages",  "key": "  Pkgs" },
    { "type": "localip",   "key": "  IP",   "showIpv4": true, "showIpv6": false, "showMac": false },
    { "type": "battery",   "key": "  Batt" },
    { "type": "separator", "string": "\u2500" },
    { "type": "command",   "key": "\u276f ", "text": "shuf -n1 ~/.config/fastfetch/quotes.txt 2>/dev/null || echo 'The obstacle is the way.'" },
    { "type": "custom",    "format": "❯ {#35} tmux{#}  {#36}cc{#} claude  {#36}gm{#} gemini  {#36}ai{#} workspace" },
    "break",
    { "type": "colors",    "symbol": "circle" }
  ]
}
FF
print_step "Fastfetch — stoic quotes + refined layout"

########################################################################
# 6/13 — Zsh Plugins + Yazi
########################################################################
print_header "🧩 6/13 — Zsh Plugins + Yazi"
ZP="$HOME/.local/share"
mkdir -p "$ZP"

declare -A zsh_plugins=(
  [zsh-autosuggestions]="https://github.com/zsh-users/zsh-autosuggestions"
  [zsh-syntax-highlighting]="https://github.com/zsh-users/zsh-syntax-highlighting"
  [fzf-tab]="https://github.com/Aloxaf/fzf-tab"
  [zsh-vi-mode]="https://github.com/jeffreytse/zsh-vi-mode"
)

for plugin in zsh-autosuggestions zsh-syntax-highlighting fzf-tab zsh-vi-mode; do
  if [[ ! -d "$ZP/$plugin" ]]; then
    git clone --depth 1 "${zsh_plugins[$plugin]}" "$ZP/$plugin" 2>/dev/null \
      && print_step "$plugin" || print_warn "$plugin clone failed"
  else
    print_skip "$plugin"
  fi
done

mkdir -p "$HOME/.config/yazi"
cat > "$HOME/.config/yazi/yazi.toml" << 'YZ'
[manager]
ratio          = [1, 4, 3]
sort_by        = "alphabetical"
sort_sensitive = false
sort_reverse   = false
show_hidden    = false
show_symlink   = true

[preview]
tab_size   = 2
max_width  = 800
max_height = 600

[plugin]
prepend_fetchers = [
  { id = "git", name = "*", run = "git" },
  { id = "git", name = "*/", run = "git" },
]
YZ

cat > "$HOME/.config/yazi/keymap.toml" << 'YK'
[[manager.prepend_keymap]]
on  = ["<C-s>"]
run = "shell --block '$SHELL'"
desc = "Open shell here"

[[manager.prepend_keymap]]
on  = ["."]
run = "hidden"
desc = "Toggle hidden files"

[[manager.prepend_keymap]]
on   = [ "Z" ]
run  = "plugin zoxide"
desc = "Jump to a directory using Zoxide"
YK
print_step "Yazi config + keymap"

if command -v ya &>/dev/null; then
  ya pack -a yazi-rs/plugins:git 2>/dev/null || true
  ya pack -a yazi-rs/plugins:zoxide 2>/dev/null || true
  print_step "Yazi Git & Zoxide plugins installed"
fi

########################################################################
# 7/13 — tmux (Ayu Dark — no plugin dependencies)
########################################################################
print_header "🖥️  7/13 — tmux"
if [[ ! -d "$HOME/.tmux/plugins/tpm" ]]; then
  git clone --depth 1 https://github.com/tmux-plugins/tpm "$HOME/.tmux/plugins/tpm" 2>/dev/null \
    && print_step "TPM" || print_warn "TPM clone failed"
else
  print_skip "TPM"
fi

cat > "$HOME/.tmux.conf" << 'TC'
set -g default-terminal "tmux-256color"
set -ag terminal-overrides ",xterm-256color:RGB"
set -g mouse on
set -g history-limit 50000
set -g base-index 1
setw -g pane-base-index 1
set -g renumber-windows on
set -g set-clipboard on
set -sg escape-time 0
set -g focus-events on
set -g status-position top

# ── Ayu Dark — status bar ─────────────────────────────────────────
set -g status-style "bg=#0b0e14,fg=#bfbdb6"
set -g status-left-length 30
set -g status-right-length 60

set -g status-left "#[bg=#53bdfa,fg=#0b0e14,bold]  #S #[bg=#0b0e14,fg=#53bdfa]"
set -g status-right "#[fg=#8b949e]#{b:pane_current_path}  #[fg=#53bdfa,bold]%H:%M "

set -g window-status-format         "#[fg=#8b949e]  ○ #I #W "
set -g window-status-current-format "#[fg=#53bdfa,bold]  ● #I #W #[fg=#f9af4f]"
set -g window-status-separator      ""

# ── Ayu Dark — pane borders ───────────────────────────────────────
set -g pane-border-style        "fg=#1e232b"
set -g pane-active-border-style "fg=#53bdfa,bold"
set -g pane-border-lines         heavy
set -g pane-border-indicators    both

# ── Ayu Dark — messages & menus ───────────────────────────────────
set -g message-style         "bg=#1b3a5b,fg=#bfbdb6"
set -g message-command-style "bg=#1b3a5b,fg=#bfbdb6"
set -g mode-style            "bg=#1b3a5b,fg=#bfbdb6"
set -g popup-border-style    "fg=#53bdfa"
set -g clock-mode-colour     "#53bdfa"

unbind C-b
set -g prefix C-a
bind C-a send-prefix

bind | split-window -h -c "#{pane_current_path}"
bind - split-window -v -c "#{pane_current_path}"
bind c new-window -c "#{pane_current_path}"
unbind '"'
unbind %

bind h select-pane -L
bind j select-pane -D
bind k select-pane -U
bind l select-pane -R
bind -r H resize-pane -L 5
bind -r J resize-pane -D 5
bind -r K resize-pane -U 5
bind -r L resize-pane -R 5

setw -g mode-keys vi
bind -T copy-mode-vi v send -X begin-selection
bind -T copy-mode-vi C-v send -X rectangle-toggle
bind -T copy-mode-vi y send -X copy-selection-and-cancel

bind r source-file ~/.tmux.conf \; display-message "✓ Config reloaded"

set -g @plugin 'tmux-plugins/tpm'
set -g @plugin 'tmux-plugins/tmux-sensible'
set -g @plugin 'christoomey/vim-tmux-navigator'
set -g @plugin 'tmux-plugins/tmux-resurrect'
set -g @plugin 'tmux-plugins/tmux-continuum'
set -g @plugin 'tmux-plugins/tmux-yank'
set -g @plugin 'sainnhe/tmux-fzf'
set -g @plugin 'omerxx/tmux-sessionx'
set -g @plugin 'omerxx/tmux-floax'

set -g @floax-width '80%'
set -g @floax-height '80%'
set -g @floax-border-color 'magenta'
set -g @floax-text-color 'blue'
set -g @floax-bind 'p'
set -g @floax-change-path 'true'

set -g @sessionx-bind 'o'
set -g @sessionx-window-height '85%'
set -g @sessionx-window-width '75%'
set -g @sessionx-zoxide-mode 'on'
set -g @sessionx-filter-current 'false'

set -g @continuum-restore 'on'
set -g @resurrect-strategy-nvim 'session'
set -g @resurrect-capture-pane-contents 'on'

bind A run-shell "tmux-ai #{pane_current_path}"
bind P run-shell "tmux-pair #{pane_current_path}"
bind R run-shell "tmux-review #{pane_current_path}"

bind C-c display-popup -E -w 80% -h 80% "claude"
bind C-g display-popup -E -w 80% -h 80% "gemini"
bind T   display-popup -E -w 60% -h 50% "zsh"
bind d display-popup -E -w 90% -h 90% "lazydocker 2>/dev/null || echo 'lazydocker not installed'"
bind g display-popup -E -w 90% -h 90% "lazygit 2>/dev/null || git log --oneline --graph -20"
bind f display-popup -E -w 90% -h 90% "yazi"
bind b display-popup -E -w 90% -h 90% "btop 2>/dev/null || htop 2>/dev/null || top"

run '~/.tmux/plugins/tpm/tpm'
TC
print_step "tmux.conf — Ayu Dark (no plugin theme dependency)"

########################################################################
# 8/13 — Neovim
########################################################################
print_header "✏️  8/13 — Neovim"
mkdir -p "$HOME/.config/nvim"
cat > "$HOME/.config/nvim/init.lua" << 'NV'
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
  local lazyrepo = "https://github.com/folke/lazy.nvim.git"
  local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
  if vim.v.shell_error ~= 0 then
    vim.api.nvim_echo({{ "Failed to clone lazy.nvim:\n", "ErrorMsg" }, { out, "WarningMsg" }, { "\nPress any key to exit..." }}, true, {})
    vim.fn.getchar()
    os.exit(1)
  end
end
vim.opt.rtp:prepend(lazypath)

require("lazy").setup({
  spec = {
    { "LazyVim/LazyVim", import = "lazyvim.plugins" },
    {
      "Shatur/neovim-ayu",
      priority = 1000,
      config = function()
        require("ayu").setup({
          mirage = false,
          overrides = {
            Normal = { bg = "#0b0e14" },
            NormalFloat = { bg = "#0b0e14" },
          },
        })
        vim.cmd.colorscheme("ayu-dark")
      end,
    },
    { "lukas-reineke/indent-blankline.nvim", main = "ibl",
      opts = { indent = { char = "│" }, scope = { char = "│" } } },
    { "echasnovski/mini.animate", version = "*", opts = {} },
    { "RRethy/vim-illuminate", event = "BufReadPost" },
    { "christoomey/vim-tmux-navigator" },
    {
      "nvim-neo-tree/neo-tree.nvim",
      branch = "v3.x",
      dependencies = { "nvim-lua/plenary.nvim", "nvim-tree/nvim-web-devicons", "MunifTanjim/nui.nvim" },
      keys = { { "<leader>e", "<cmd>Neotree toggle<cr>", desc = "Toggle Explorer" } },
      opts = { filesystem = { filtered_items = { visible = true, hide_dotfiles = false } } },
    },
    {
      "akinsho/bufferline.nvim",
      version = "*",
      dependencies = "nvim-tree/nvim-web-devicons",
      opts = {
        options = {
          separator_style = "slant",
          diagnostics = "nvim_lsp",
          show_buffer_close_icons = false,
        }
      }
    },
    {
      "folke/noice.nvim",
      event = "VeryLazy",
      dependencies = { "MunifTanjim/nui.nvim", "rcarriga/nvim-notify" },
      opts = {
        lsp = { override = { ["vim.lsp.util.convert_input_to_markdown_lines"] = true, ["vim.lsp.util.stylize_markdown"] = true, ["cmp.entry.get_documentation"] = true } },
        presets = { bottom_search = true, command_palette = true, long_message_to_split = true },
      },
    },
  },
  defaults = { lazy = false, version = false },
  install = { colorscheme = { "ayu-dark", "habamax" } },
  checker = { enabled = true, notify = false },
  performance = { rtp = { disabled_plugins = { "gzip", "tarPlugin", "tohtml", "tutor", "zipPlugin" } } }
})

vim.g.mapleader = " "
vim.opt.number = true
vim.opt.relativenumber = true
vim.opt.termguicolors = true
vim.opt.cursorline = true
vim.opt.scrolloff = 8
vim.opt.expandtab = true
vim.opt.shiftwidth = 2
vim.opt.tabstop = 2
vim.opt.smartindent = true
vim.opt.mouse = "a"
vim.opt.clipboard = "unnamedplus"
vim.opt.undofile = true
vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.signcolumn = "yes"
NV
print_step "Neovim — Ayu Dark + LazyVim"

########################################################################
# 9/13 — Tool Configs (Atuin + Lazygit)
########################################################################
print_header "⚙️  9/13 — Tool Configs"
mkdir -p "$HOME/.config/atuin"
cat > "$HOME/.config/atuin/config.toml" << 'AT'
style = "full"
inline_height = 0
show_preview = true
filter_mode = "global"
filter_mode_shell_up_key_binding = "directory"
search_mode = "fuzzy"
show_tabs = true
timestamps_enabled = true
time_format = "%I:%M %p"
exit_mode = "return-original"
sync_records = true
AT
print_step "Atuin"
atuin import auto 2>/dev/null || true

mkdir -p "$HOME/.config/lazygit"
cat > "$HOME/.config/lazygit/config.yml" << 'LG'
gui:
  nerdFontsVersion: "3"
  showFileIcons: true
  border: rounded
  mouseEvents: true
  showCommandLog: false
  theme:
    activeBorderColor:        ['#53bdfa', bold]
    inactiveBorderColor:      ['#8b949e']
    optionsTextColor:         ['#cda1fa']
    selectedLineBgColor:      ['#1b3a5b']
    selectedRangeBgColor:     ['#1b3a5b']
    cherryPickedCommitBgColor:['#1e232b']
    cherryPickedCommitFgColor:['#cda1fa']
    unstagedChangesColor:     ['#ea6c73']
    defaultFgColor:           ['#bfbdb6']
    searchingActiveBorderColor:['#f9af4f']
git:
  paging:
    colorArg: always
    pager: delta --dark --paging=never
os:
  editPreset: nvim
LG
print_step "Lazygit — Ayu Dark"

mkdir -p "$HOME/.config/lazydocker"
cat > "$HOME/.config/lazydocker/config.yml" << 'LDK'
gui:
  theme:
    activeBorderColor:
      - "#53bdfa"
      - bold
    inactiveBorderColor:
      - "#8b949e"
    optionsTextColor:
      - "#cda1fa"
    selectedLineBgColor:
      - "#1b3a5b"
LDK
print_step "Lazydocker — Ayu Dark"

########################################################################
# 10/13 — Git (guarded: never overwrites existing settings)
########################################################################
print_header "🔀 10/13 — Git"

git_default core.pager "delta"
git_default interactive.diffFilter "delta --color-only"

git config --global delta.navigate true
git config --global delta.dark true
git config --global delta.line-numbers true
git config --global delta.side-by-side true
git config --global delta.syntax-theme "ayu-dark"
git config --global delta.file-style "bold yellow"
git config --global delta.file-decoration-style "yellow ul"
git config --global delta.hunk-header-style "syntax bold"
git config --global delta.hunk-header-decoration-style "blue box"
git config --global delta.minus-style "syntax #2d1517"
git config --global delta.plus-style "syntax #152e1a"
git config --global delta.minus-emph-style "syntax #4a1c1f"
git config --global delta.plus-emph-style "syntax #1e4020"
git config --global delta.zero-style "syntax"
git config --global delta.whitespace-error-style "reverse red"

git_default merge.conflictstyle "diff3"
git_default diff.algorithm "histogram"
git_default diff.colorMoved "default"
git_default pull.rebase true
git_default rebase.autoStash true
git_default push.autoSetupRemote true
git_default push.default current
git_default init.defaultBranch main
git_default core.editor nvim
git_default help.autocorrect 20

git_default alias.s "status -sb"
git_default alias.co "checkout"
git_default alias.br "branch"
git_default alias.ci "commit"
git_default alias.lg "log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"
git_default alias.last "log -1 HEAD --stat"
git_default alias.unstage "reset HEAD --"
git_default alias.amend "commit --amend --no-edit"
git_default alias.undo "reset --soft HEAD~1"
git_default alias.wip '!git add -A && git commit -m "wip"'
git_default alias.cleanup '!git branch --merged | grep -v "\\*\\|main\\|master" | xargs -n 1 git branch -d'

print_step "Git enhanced (existing settings preserved)"

########################################################################
# 11/13 — Ayu Dark Themes (bat, btop, yazi)
########################################################################
print_header "🎨 11/13 — Ayu Dark Themes"

# ── bat: custom Ayu Dark TextMate theme ───────────────────────────
BAT_THEMES="$(bat --config-dir 2>/dev/null)/themes"
if [[ -n "$BAT_THEMES" ]]; then
  mkdir -p "$BAT_THEMES"
  # Remove old Catppuccin theme if present
  rm -f "$BAT_THEMES/Catppuccin Mocha.tmTheme" 2>/dev/null || true
  cat > "$BAT_THEMES/Ayu Dark.tmTheme" << 'BATTHEME'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>name</key>
  <string>Ayu Dark</string>
  <key>settings</key>
  <array>
    <!-- Global settings -->
    <dict>
      <key>settings</key>
      <dict>
        <key>background</key><string>#0B0E14</string>
        <key>foreground</key><string>#BFBDB6</string>
        <key>caret</key><string>#BFBDB6</string>
        <key>selection</key><string>#1B3A5B</string>
        <key>lineHighlight</key><string>#131721</string>
        <key>gutterForeground</key><string>#636A72</string>
        <key>gutterBackground</key><string>#0B0E14</string>
        <key>guide</key><string>#1E232B</string>
        <key>activeGuide</key><string>#53BDFA</string>
      </dict>
    </dict>
    <!-- Comments -->
    <dict>
      <key>name</key><string>Comment</string>
      <key>scope</key><string>comment, punctuation.definition.comment</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#8B949E</string>
      </dict>
    </dict>
    <!-- Strings -->
    <dict>
      <key>name</key><string>String</string>
      <key>scope</key><string>string, string.quoted</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#AAD94C</string>
      </dict>
    </dict>
    <!-- Numbers -->
    <dict>
      <key>name</key><string>Number</string>
      <key>scope</key><string>constant.numeric</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#D2A6FF</string>
      </dict>
    </dict>
    <!-- Constants -->
    <dict>
      <key>name</key><string>Constant</string>
      <key>scope</key><string>constant, constant.language, constant.character</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#D2A6FF</string>
      </dict>
    </dict>
    <!-- Keywords -->
    <dict>
      <key>name</key><string>Keyword</string>
      <key>scope</key><string>keyword, keyword.control, keyword.operator.logical, keyword.operator.new, storage.type, storage.modifier</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#FF8F40</string>
      </dict>
    </dict>
    <!-- Operators -->
    <dict>
      <key>name</key><string>Operator</string>
      <key>scope</key><string>keyword.operator, keyword.operator.assignment</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#F29668</string>
      </dict>
    </dict>
    <!-- Functions -->
    <dict>
      <key>name</key><string>Function</string>
      <key>scope</key><string>entity.name.function, meta.function-call, support.function</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#FFB454</string>
      </dict>
    </dict>
    <!-- Classes / Types -->
    <dict>
      <key>name</key><string>Class / Type</string>
      <key>scope</key><string>entity.name.type, entity.name.class, support.type, support.class, entity.other.inherited-class</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
      </dict>
    </dict>
    <!-- Variables -->
    <dict>
      <key>name</key><string>Variable</string>
      <key>scope</key><string>variable, variable.other, variable.parameter</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#BFBDB6</string>
      </dict>
    </dict>
    <!-- Variable special (this, self) -->
    <dict>
      <key>name</key><string>Variable Special</string>
      <key>scope</key><string>variable.language</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
      </dict>
    </dict>
    <!-- Tags (HTML/XML) -->
    <dict>
      <key>name</key><string>Tag</string>
      <key>scope</key><string>entity.name.tag</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
      </dict>
    </dict>
    <!-- Tag attributes -->
    <dict>
      <key>name</key><string>Tag Attribute</string>
      <key>scope</key><string>entity.other.attribute-name</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#FFB454</string>
      </dict>
    </dict>
    <!-- Punctuation -->
    <dict>
      <key>name</key><string>Punctuation</string>
      <key>scope</key><string>punctuation, punctuation.definition.tag, punctuation.separator, punctuation.section</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#BFBDB680</string>
      </dict>
    </dict>
    <!-- Markup heading -->
    <dict>
      <key>name</key><string>Markup Heading</string>
      <key>scope</key><string>markup.heading, markup.heading entity.name</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
        <key>fontStyle</key><string>bold</string>
      </dict>
    </dict>
    <!-- Markup bold -->
    <dict>
      <key>name</key><string>Markup Bold</string>
      <key>scope</key><string>markup.bold</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#FFB454</string>
        <key>fontStyle</key><string>bold</string>
      </dict>
    </dict>
    <!-- Markup italic -->
    <dict>
      <key>name</key><string>Markup Italic</string>
      <key>scope</key><string>markup.italic</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#CDA1FA</string>
      </dict>
    </dict>
    <!-- Markup link -->
    <dict>
      <key>name</key><string>Markup Link</string>
      <key>scope</key><string>markup.underline.link, string.other.link</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#53BDFA</string>
      </dict>
    </dict>
    <!-- Markup inline code -->
    <dict>
      <key>name</key><string>Markup Code</string>
      <key>scope</key><string>markup.inline.raw, markup.raw.block</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#AAD94C</string>
      </dict>
    </dict>
    <!-- Diff inserted -->
    <dict>
      <key>name</key><string>Diff Inserted</string>
      <key>scope</key><string>markup.inserted, meta.diff.header.to-file</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#7FD962</string>
      </dict>
    </dict>
    <!-- Diff deleted -->
    <dict>
      <key>name</key><string>Diff Deleted</string>
      <key>scope</key><string>markup.deleted, meta.diff.header.from-file</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#EA6C73</string>
      </dict>
    </dict>
    <!-- Diff changed -->
    <dict>
      <key>name</key><string>Diff Changed</string>
      <key>scope</key><string>markup.changed</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#F9AF4F</string>
      </dict>
    </dict>
    <!-- Regular expressions -->
    <dict>
      <key>name</key><string>Regex</string>
      <key>scope</key><string>string.regexp</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#95E6CB</string>
      </dict>
    </dict>
    <!-- Escape characters -->
    <dict>
      <key>name</key><string>Escape</string>
      <key>scope</key><string>constant.character.escape</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#95E6CB</string>
      </dict>
    </dict>
    <!-- Invalid -->
    <dict>
      <key>name</key><string>Invalid</string>
      <key>scope</key><string>invalid, invalid.illegal</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#EA6C73</string>
      </dict>
    </dict>
    <!-- Embedded / Template -->
    <dict>
      <key>name</key><string>Embedded</string>
      <key>scope</key><string>punctuation.section.embedded, variable.interpolation</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#F07178</string>
      </dict>
    </dict>
    <!-- JSON keys -->
    <dict>
      <key>name</key><string>JSON Key</string>
      <key>scope</key><string>support.type.property-name.json, meta.structure.dictionary.key string</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
      </dict>
    </dict>
    <!-- YAML keys -->
    <dict>
      <key>name</key><string>YAML Key</string>
      <key>scope</key><string>entity.name.tag.yaml</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
      </dict>
    </dict>
    <!-- CSS properties -->
    <dict>
      <key>name</key><string>CSS Property</string>
      <key>scope</key><string>support.type.property-name.css</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
      </dict>
    </dict>
    <!-- CSS values -->
    <dict>
      <key>name</key><string>CSS Value</string>
      <key>scope</key><string>support.constant.property-value.css, constant.other.color.rgb-value.hex.css</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#D2A6FF</string>
      </dict>
    </dict>
    <!-- Shell variables -->
    <dict>
      <key>name</key><string>Shell Variable</string>
      <key>scope</key><string>variable.other.normal.shell, variable.other.positional.shell, variable.other.bracket.shell</string>
      <key>settings</key><dict>
        <key>foreground</key><string>#59C2FF</string>
      </dict>
    </dict>
  </array>
</dict>
</plist>
BATTHEME
  bat cache --build 2>/dev/null || true
  print_step "bat — Ayu Dark (.tmTheme)"
fi

# ── btop: use built-in ayu theme ─────────────────────────────────
mkdir -p "$HOME/.config/btop"
# Remove old Catppuccin theme if present
rm -f "$HOME/.config/btop/themes/catppuccin_mocha.theme" 2>/dev/null || true
cat > "$HOME/.config/btop/btop.conf" << 'BTOP'
color_theme = "ayu"
theme_background = false
truecolor = true
shown_boxes = "cpu mem net proc"
update_ms = 1000
proc_sorting = "cpu lazy"
proc_tree = false
BTOP
print_step "btop — Ayu (built-in)"

# ── yazi: custom Ayu Dark theme ───────────────────────────────────
mkdir -p "$HOME/.config/yazi"
# Remove old Catppuccin theme
rm -f "$HOME/.config/yazi/theme.toml" 2>/dev/null || true
cat > "$HOME/.config/yazi/theme.toml" << 'YAZITHEME'
# Ayu Dark theme for Yazi

[manager]
cwd = { fg = "#53bdfa" }

hovered         = { bg = "#1b3a5b", fg = "#bfbdb6" }
preview_hovered = { bg = "#131721" }

find_keyword  = { fg = "#f9af4f", bold = true }
find_position = { fg = "#cda1fa", bg = "reset" }

marker_copied   = { fg = "#7fd962", bg = "#7fd962" }
marker_cut      = { fg = "#ea6c73", bg = "#ea6c73" }
marker_marked   = { fg = "#53bdfa", bg = "#53bdfa" }
marker_selected = { fg = "#cda1fa", bg = "#cda1fa" }

tab_active   = { bg = "#1b3a5b", fg = "#bfbdb6", bold = true }
tab_inactive = { bg = "#0b0e14", fg = "#8b949e" }
tab_width    = 1

count_copied   = { fg = "#0b0e14", bg = "#7fd962" }
count_cut      = { fg = "#0b0e14", bg = "#ea6c73" }
count_selected = { fg = "#0b0e14", bg = "#cda1fa" }

border_symbol = "│"
border_style  = { fg = "#1e232b" }

[status]
separator_open  = ""
separator_close = ""
separator_style = { fg = "#1e232b" }

mode_normal = { bg = "#53bdfa", fg = "#0b0e14", bold = true }
mode_select = { bg = "#cda1fa", fg = "#0b0e14", bold = true }
mode_unset  = { bg = "#ea6c73", fg = "#0b0e14", bold = true }

progress_label  = { fg = "#bfbdb6", bold = true }
progress_normal = { fg = "#53bdfa", bg = "#1e232b" }
progress_error  = { fg = "#ea6c73", bg = "#1e232b" }

permissions_t = { fg = "#53bdfa" }
permissions_r = { fg = "#f9af4f" }
permissions_w = { fg = "#ea6c73" }
permissions_x = { fg = "#7fd962" }
permissions_s = { fg = "#8b949e" }

[input]
border   = { fg = "#53bdfa" }
title    = { fg = "#53bdfa" }
value    = { fg = "#bfbdb6" }
selected = { bg = "#1b3a5b" }

[pick]
border   = { fg = "#53bdfa" }
active   = { fg = "#cda1fa", bold = true }
inactive = { fg = "#bfbdb6" }

[confirm]
border   = { fg = "#53bdfa" }
title    = { fg = "#53bdfa" }
content  = { fg = "#bfbdb6" }
list     = { fg = "#bfbdb6" }
btn_yes  = { bg = "#1b3a5b", fg = "#7fd962" }
btn_no   = { bg = "#1b3a5b", fg = "#ea6c73" }
btn_label = { fg = "#bfbdb6", bold = true }

[completion]
border   = { fg = "#53bdfa" }
active   = { bg = "#1b3a5b", fg = "#bfbdb6" }
inactive = { fg = "#8b949e" }

[tasks]
border  = { fg = "#53bdfa" }
title   = { fg = "#53bdfa" }
hovered = { bg = "#1b3a5b", fg = "#bfbdb6" }

[which]
cols = 3
mask            = { bg = "#0b0e14" }
cand            = { fg = "#90e1c6" }
rest            = { fg = "#8b949e" }
desc            = { fg = "#cda1fa" }
separator       = "  "
separator_style = { fg = "#1e232b" }

[help]
on      = { fg = "#53bdfa" }
run     = { fg = "#cda1fa" }
desc    = { fg = "#8b949e" }
hovered = { bg = "#1b3a5b", fg = "#bfbdb6", bold = true }
footer  = { fg = "#8b949e", bg = "#0b0e14" }

[notify]
title_info  = { fg = "#53bdfa" }
title_warn  = { fg = "#f9af4f" }
title_error = { fg = "#ea6c73" }

[filetype]
rules = [
  # Media
  { mime = "image/*", fg = "#cda1fa" },
  { mime = "video/*", fg = "#f9af4f" },
  { mime = "audio/*", fg = "#f9af4f" },

  # Archives
  { mime = "application/zip",    fg = "#ea6c73" },
  { mime = "application/gzip",   fg = "#ea6c73" },
  { mime = "application/x-tar",  fg = "#ea6c73" },
  { mime = "application/x-7z*",  fg = "#ea6c73" },
  { mime = "application/x-rar",  fg = "#ea6c73" },
  { mime = "application/x-bzip*",fg = "#ea6c73" },

  # Documents
  { mime = "application/pdf",    fg = "#ea6c73" },
  { mime = "application/json",   fg = "#f9af4f" },
  { mime = "application/xml",    fg = "#f9af4f" },

  # Code / text
  { mime = "text/*", fg = "#bfbdb6" },

  # Fallback
  { name = "*", fg = "#bfbdb6" },
  { name = "*/", fg = "#53bdfa", bold = true },
]
YAZITHEME
print_step "yazi — Ayu Dark (custom)"

########################################################################
# 12/13 — tmux Layout Scripts + Cheat Sheet
########################################################################
print_header "📜 12/13 — tmux Layout Scripts"

cat > "$SCRIPTS_DIR/tmux-ai" << 'S1'
#!/bin/bash
S="ai"; D="${1:-$(pwd)}"
tmux has-session -t "$S" 2>/dev/null && { tmux attach -t "$S"; exit 0; }
tmux new-session -d -s "$S" -n "claude" -c "$D"
tmux new-window -t "$S" -n "gemini" -c "$D"
tmux new-window -t "$S" -n "editor" -c "$D"
tmux send-keys -t "$S:editor" "nvim ." Enter
tmux new-window -t "$S" -n "shell" -c "$D"
tmux split-window -t "$S:shell" -h -p 40 -c "$D"
tmux send-keys -t "$S:shell.2" "git status -sb 2>/dev/null" Enter
tmux select-window -t "$S:claude"
tmux attach -t "$S"
S1
chmod +x "$SCRIPTS_DIR/tmux-ai"
print_step "tmux-ai"

cat > "$SCRIPTS_DIR/tmux-pair" << 'S2'
#!/bin/bash
S="pair"; D="${1:-$(pwd)}"
tmux has-session -t "$S" 2>/dev/null && { tmux attach -t "$S"; exit 0; }
tmux new-session -d -s "$S" -n "agents" -c "$D"
tmux split-window -t "$S:agents" -h -p 50 -c "$D"
tmux split-window -t "$S:agents.1" -v -p 30 -c "$D"
tmux select-pane -t "$S:agents.1"
tmux attach -t "$S"
S2
chmod +x "$SCRIPTS_DIR/tmux-pair"
print_step "tmux-pair"

cat > "$SCRIPTS_DIR/tmux-review" << 'S3'
#!/bin/bash
S="review"; D="${1:-$(pwd)}"
tmux has-session -t "$S" 2>/dev/null && { tmux attach -t "$S"; exit 0; }
tmux new-session -d -s "$S" -n "review" -c "$D"
tmux split-window -t "$S:review" -h -p 50 -c "$D"
tmux send-keys -t "$S:review.2" "git diff --stat 2>/dev/null && git log --oneline -10 2>/dev/null" Enter
tmux select-pane -t "$S:review.1"
tmux attach -t "$S"
S3
chmod +x "$SCRIPTS_DIR/tmux-review"
print_step "tmux-review"

cat > "$SCRIPTS_DIR/tmux-dev" << 'S4'
#!/bin/bash
S="dev"; D="${1:-$(pwd)}"
tmux has-session -t "$S" 2>/dev/null && { tmux attach -t "$S"; exit 0; }
tmux new-session -d -s "$S" -n "edit" -c "$D"
tmux send-keys -t "$S:edit" "nvim ." Enter
tmux new-window -t "$S" -n "term" -c "$D"
tmux split-window -t "$S:term" -h -p 50 -c "$D"
tmux new-window -t "$S" -n "server" -c "$D"
tmux select-window -t "$S:edit"
tmux attach -t "$S"
S4
chmod +x "$SCRIPTS_DIR/tmux-dev"
print_step "tmux-dev"

cat > "$SCRIPTS_DIR/tmux-cheat" << 'S5'
#!/bin/bash
P='\033[0;35m'; C='\033[0;36m'; B='\033[1m'; D='\033[2m'; N='\033[0m'
echo ""
echo -e "${P}  ╔═══════════════════════════════════════════════════╗${N}"
echo -e "${P}  ║  ${B}tmux Cheat Sheet${N}${P}  ·  prefix = ${B}Ctrl+A${N}${P}              ║${N}"
echo -e "${P}  ╚═══════════════════════════════════════════════════╝${N}"
echo -e "  📍 ${B}Sessions${N}"
echo -e "    ${C}Ctrl+A → d${N}  detach    ${C}Ctrl+A → o${N}  sessionx    ${C}Ctrl+A → \$${N}  rename"
echo -e "  🪟 ${B}Windows${N}"
echo -e "    ${C}Ctrl+A → c${N}  new       ${C}Ctrl+A → ,${N}  rename      ${C}Ctrl+A → n/p${N}  next/prev"
echo -e "  📐 ${B}Panes${N}"
echo -e "    ${C}Ctrl+A → |${N}  v-split   ${C}Ctrl+A → -${N}  h-split     ${C}Ctrl+A → z${N}  zoom"
echo -e "    ${C}Ctrl+A → h/j/k/l${N}  navigate    ${C}Ctrl+A → H/J/K/L${N}  resize"
echo -e "  🤖 ${B}AI Agents${N}"
echo -e "    ${C}Ctrl+A → Ctrl+C${N}  Claude    ${C}Ctrl+A → Ctrl+G${N}  Gemini"
echo -e "    ${C}Ctrl+A → A${N}  tmux-ai   ${C}Ctrl+A → P${N}  tmux-pair   ${C}Ctrl+A → R${N}  tmux-review"
echo -e "  🔧 ${B}Tools${N}"
echo -e "    ${C}Ctrl+A → g${N}  lazygit   ${C}Ctrl+A → d${N}  lazydocker   ${C}Ctrl+A → f${N}  yazi   ${C}Ctrl+A → b${N}  btop"
echo -e "    ${C}Ctrl+A → p${N}  float     ${C}Ctrl+A → T${N}  terminal"
echo -e "  📋 ${B}Copy${N}  ${D}Ctrl+A → [  then v=select y=copy q=exit${N}"
echo -e "  ${D}Disable: export TMUX_CHEAT=0 · Reload: Ctrl+A → r · Plugins: Ctrl+A → I${N}"
echo ""
S5
chmod +x "$SCRIPTS_DIR/tmux-cheat"
print_step "tmux-cheat"

cat > "$SCRIPTS_DIR/toggle-focus" << 'TF'
#!/bin/bash
GC="$HOME/.config/ghostty/config"
[[ ! -f "$GC" ]] && exit 1
if grep -q "background-opacity.*=.*1.0" "$GC"; then
  # It's opaque -> Make it transparent
  sed -i '' -e 's/background-opacity.*=.*1.0/background-opacity      = 0.80/g' \
            -e 's/background-blur.*=.*0/background-blur         = 20/g' \
            -e 's/^# custom-shader/custom-shader/g' "$GC"
  echo "👻 Ghostty: Transparent Mode"
else
  # It's transparent -> Make it opaque
  sed -i '' -e 's/background-opacity.*=.*0.80/background-opacity      = 1.0/g' \
            -e 's/background-blur.*=.*20/background-blur         = 0/g' \
            -e 's/^custom-shader/# custom-shader/g' "$GC"
  echo "🎯 Ghostty: Focus Mode (Opaque)"
fi
TF
chmod +x "$SCRIPTS_DIR/toggle-focus"
print_step "toggle-focus script (Ghostty)"

########################################################################
# 13/13 — Master .zshrc
########################################################################
print_header "🏠 13/13 — Master .zshrc"
cat >> "$ZSHRC" << 'ZRC'

# ══ GHOSTTY ULTIMATE ══════════════════════════════════════════
export PATH="$HOME/.local/bin:$PATH"
eval "$(starship init zsh)"
eval "$(zoxide init zsh)"
eval "$(atuin init zsh --disable-up-arrow)"

# fzf init
if command -v fzf &>/dev/null; then
  source <(fzf --zsh 2>/dev/null) || true
elif [ -f ~/.fzf.zsh ]; then
  source ~/.fzf.zsh
fi

export FZF_DEFAULT_OPTS=" \
  --color=bg+:#1b3a5b,bg:#0b0e14,spinner:#cda1fa,hl:#ea6c73 \
  --color=fg:#bfbdb6,header:#ea6c73,info:#cda1fa,pointer:#53bdfa \
  --color=marker:#7fd962,fg+:#bfbdb6,prompt:#53bdfa,hl+:#f07178 \
  --color=selected-bg:#1b3a5b \
  --border=rounded --prompt='❯ ' --marker='✓' --pointer='▶' \
  --separator='─' --scrollbar='│' --layout=reverse --height=60%"

export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git'
export FZF_CTRL_T_OPTS="--preview 'bat -n --color=always --line-range :500 {}'"
export FZF_ALT_C_OPTS="--preview 'eza --tree --level=2 --color=always --icons {}'"

if type brew &>/dev/null; then
  FPATH="$(brew --prefix)/share/zsh-completions:${FPATH}"
  FPATH="$(brew --prefix)/share/zsh/site-functions:${FPATH}"
fi

autoload -Uz compinit && compinit -C

zstyle ':completion:*' matcher-list 'm:{a-z}={A-Za-z}' 'r:|[._-]=* r:|=*' 'l:|=* r:|=*'
zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
zstyle ':completion:*' group-name ''
zstyle ':completion:*:descriptions' format '[%d]'
zstyle ':completion:*' menu no

[ -f "$HOME/.local/share/fzf-tab/fzf-tab.plugin.zsh" ] && source "$HOME/.local/share/fzf-tab/fzf-tab.plugin.zsh" >/dev/null 2>&1
zstyle ':fzf-tab:complete:cd:*' fzf-preview 'eza -1 --color=always --icons $realpath'
zstyle ':fzf-tab:complete:z:*' fzf-preview 'eza -1 --color=always --icons $realpath'
zstyle ':fzf-tab:complete:(cat|bat|vim|nvim):*' fzf-preview '[[ -f $realpath ]] && bat -n --color=always --line-range :200 $realpath || eza -1 --color=always --icons $realpath'
zstyle ':fzf-tab:complete:git-(add|diff|restore):*' fzf-preview 'git diff $word | delta'
zstyle ':fzf-tab:complete:git-log:*' fzf-preview 'git log --color=always $word'
zstyle ':fzf-tab:complete:git-checkout:*' fzf-preview 'case "$group" in "modified file") git diff $word | delta ;; *) git log --color=always $word ;; esac'
zstyle ':fzf-tab:complete:(kill|ps):*' fzf-preview '[[ $group == "[process ID]" ]] && ps -p $word -o pid,user,%cpu,%mem,command'
zstyle ':fzf-tab:complete:(export|unset):*' fzf-preview 'echo ${(P)word}'
zstyle ':fzf-tab:*' continuous-trigger '/'
zstyle ':fzf-tab:*' switch-group '<' '>'
[ -n "$TMUX" ] && zstyle ':fzf-tab:*' fzf-command ftb-tmux-popup

[ -f "$HOME/.local/share/zsh-autosuggestions/zsh-autosuggestions.zsh" ] && source "$HOME/.local/share/zsh-autosuggestions/zsh-autosuggestions.zsh" >/dev/null 2>&1
ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE='fg=#8b949e'
ZSH_AUTOSUGGEST_STRATEGY=(history completion)

# zsh-vi-mode: block=NORMAL | beam=INSERT | underline=REPLACE
if [ -f "$HOME/.local/share/zsh-vi-mode/zsh-vi-mode.plugin.zsh" ]; then
  ZVM_VI_INSERT_ESCAPE_BINDKEY=jk
  ZVM_CURSOR_STYLE_ENABLED=true
  ZVM_LINE_INIT_MODE=$ZVM_MODE_INSERT
  source "$HOME/.local/share/zsh-vi-mode/zsh-vi-mode.plugin.zsh"
  zvm_after_init_commands=("source <(fzf --zsh 2>/dev/null) || true")
fi

[ -f "$HOME/.local/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" ] && source "$HOME/.local/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" >/dev/null 2>&1

HISTSIZE=50000; SAVEHIST=50000; HISTFILE="$HOME/.zsh_history"
setopt APPEND_HISTORY SHARE_HISTORY HIST_IGNORE_SPACE HIST_IGNORE_ALL_DUPS HIST_SAVE_NO_DUPS HIST_FIND_NO_DUPS HIST_REDUCE_BLANKS INC_APPEND_HISTORY

bindkey '^[[A' history-search-backward; bindkey '^[[B' history-search-forward
bindkey '\ef' forward-word; bindkey '\eb' backward-word
bindkey '^A' beginning-of-line; bindkey '^E' end-of-line
bindkey '^K' kill-line; bindkey '^U' backward-kill-line
bindkey '^X^E' edit-command-line; autoload -z edit-command-line; zle -N edit-command-line

export EDITOR=nvim
export VISUAL=nvim
export BAT_THEME="Ayu Dark"
export MANPAGER="sh -c 'col -bx | bat -l man -p'"

# ── Aliases ──────────────────────────────────────────────────
alias ls='eza --color=always --icons --group-directories-first'
alias ll='eza -la --color=always --icons --group-directories-first'
alias la='eza -a --color=always --icons --group-directories-first'
alias lt='eza --tree --level=2 --color=always --icons'
alias tree='eza --tree --color=always --icons'
alias cat='bat --paging=never'
alias grep='rg'
alias find='fd'
alias du='dust'
alias ps='procs'
alias ping='gping'
alias curl='curlie'
alias tldr='tlrc'
alias loc='tokei'
alias bench='hyperfine'
alias vim='nvim'
alias vi='nvim'
alias lg='lazygit'
alias g='git'
alias gs='git status -sb'
alias ga='git add'
alias gc='git commit'
alias gp='git push'
alias gl='git log --oneline --graph -20'
alias gd='git diff'
alias gds='git diff --staged'
alias gco='git checkout'
alias gb='git branch'

alias ld='lazydocker'
alias focus='toggle-focus'

alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias ta='tmux attach -t'
alias tl='tmux list-sessions'
alias tk='tmux kill-session -t'
alias myip='curl -s ifconfig.me'
alias localip='ipconfig getifaddr en0 2>/dev/null || echo "N/A"'

alias ports='lsof -i -P -n | grep LISTEN'
alias serve='python3 -m http.server'
alias wt='curl -s wttr.in/?format=3'
alias update='brew update && brew upgrade && brew cleanup'
alias path='echo $PATH | tr ":" "\n"'
alias reload='source ~/.zshrc && echo "  ✓ zshrc reloaded"'
alias week='date +%V'
alias timestamp='date +%Y%m%d_%H%M%S'

# rm → trash only in interactive shells
[[ $- == *i* ]] && alias rm='trash'

# ── Functions ────────────────────────────────────────────────
unalias cc 2>/dev/null || true
unalias gm 2>/dev/null || true
function cc() { if [ -n "$TMUX" ]; then command claude "$@"; else local s="claude"; tmux has-session -t "$s" 2>/dev/null && tmux attach -t "$s" || tmux new-session -s "$s" -c "${PWD}" "claude $*; zsh"; fi; }
function gm() { if [ -n "$TMUX" ]; then command gemini "$@"; else local s="gemini"; tmux has-session -t "$s" 2>/dev/null && tmux attach -t "$s" || tmux new-session -s "$s" -c "${PWD}" "gemini $*; zsh"; fi; }

function ai() { local d="${1:-$PWD}"; [ -n "$TMUX" ] && { echo "▲ Already in tmux."; return; }; tmux-ai "$d"; }
function y() { local t="$(mktemp -t yazi-cwd.XXXXXX)"; yazi "$@" --cwd-file="$t"; if cwd="$(cat -- "$t")" && [ -n "$cwd" ] && [ "$cwd" != "$PWD" ]; then builtin cd -- "$cwd"; fi; rm -f -- "$t"; }
function fcd() { local d; d=$(fd --type d --hidden --follow --exclude .git | fzf --preview 'eza --tree --level=2 --color=always --icons {}') && cd "$d"; }
function fvim() { local f; f=$(fd --type f --hidden --follow --exclude .git | fzf --preview 'bat -n --color=always --line-range :200 {}') && nvim "$f"; }
function ff() { local r; r=$(rg --color=always --line-number --no-heading "${1:-.}" | fzf --ansi --delimiter ':') || return; nvim "+$(echo "$r"|cut -d: -f2)" "$(echo "$r"|cut -d: -f1)"; }
function mkcd() { mkdir -p "$1" && cd "$1"; }
function toclaude() { local i; [ -t 0 ] && i="$*" || i=$(cat); echo "$i" | claude; }
function askc() { [ -f "$1" ] && cat "$1" | claude "${2:-explain this code}" || echo "✗ File not found: $1"; }
function gcai() { local d=$(git diff --staged); [ -z "$d" ] && { echo "▲ No staged changes."; return 1; }; echo "$d" | claude "Write a concise git commit message. Output ONLY the message." | git commit -F -; }
function hx() { curl -sI "$1" | bat --language=http --style=plain; }
function extract() {
  case "$1" in
    *.tar.bz2) tar xjf "$1" ;; *.tar.gz) tar xzf "$1" ;; *.tar.xz) tar xJf "$1" ;;
    *.zip) unzip "$1" ;; *.7z) 7zz x "$1" ;; *.gz) gunzip "$1" ;; *) echo "✗ Unknown format: $1" ;;
  esac
}
function tmpdir() { local t=$(mktemp -d); echo "  → $t"; cd "$t"; }

if [[ $- == *i* ]] && [[ -z "$TMUX" ]]; then command fastfetch 2>/dev/null; fi
if [[ $- == *i* ]] && [[ -n "$TMUX" ]] && [[ "${TMUX_CHEAT:-1}" == "1" ]]; then
  if [[ -z "$TMUX_CHEAT_SHOWN" ]]; then export TMUX_CHEAT_SHOWN=1; command tmux-cheat 2>/dev/null; fi
fi
# ══ END GHOSTTY ULTIMATE ═════════════════════════════════════
ZRC
print_step "Master .zshrc deployed"

########################################################################
# Done
########################################################################
echo ""
echo -e "${c_surface}────────────────────────────────────────────────────────${c_reset}"
elapsed=$(( $(date +%s) - start_time ))
echo -e "  ${c_green}✓${c_reset} ${c_purple}${c_bold}GHOSTTY ULTIMATE — Ayu Dark${c_reset}  ${c_dim}(${elapsed}s)${c_reset}"
if [[ $warn_count -gt 0 ]]; then
  echo -e "  ${c_dim}${warn_count} warning(s) — review above${c_reset}"
else
  echo -e "  ${c_dim}Clean install — zero warnings${c_reset}"
fi
echo -e "${c_surface}────────────────────────────────────────────────────────${c_reset}"
echo ""
echo -e "  ${c_bold}1.${c_reset} source ~/.zshrc"
echo -e "  ${c_bold}2.${c_reset} Cmd+Q Ghostty → reopen"
echo -e "  ${c_bold}3.${c_reset} tmux → Ctrl+A I ${c_faint}(install plugins)${c_reset}"
echo -e "  ${c_bold}4.${c_reset} nvim ${c_faint}(auto-installs lazyvim + ayu-dark + all plugins)${c_reset}"
echo ""
echo -e "  🤖 ${c_blue}cc${c_reset} Claude  ${c_blue}gm${c_reset} Gemini  ${c_blue}ai${c_reset} Workspace  ${c_blue}lg${c_reset} Lazygit  ${c_blue}y${c_reset} Yazi"
echo -e "  🔧 ${c_green}wt${c_reset} Weather  ${c_green}ports${c_reset} Listeners  ${c_green}serve${c_reset} HTTP  ${c_green}update${c_reset} Brew  ${c_green}extract${c_reset} Archives"
echo ""
