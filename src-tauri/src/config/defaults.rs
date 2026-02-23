use super::palette::AyuPalette;
use super::sections::*;
use super::schema::GhosttyConfig;

pub fn default_config() -> GhosttyConfig {
    GhosttyConfig {
        palette: AyuPalette::default(),
        clean_artifacts: default_clean_artifacts(),
        packages: default_packages(),
        ghostty: default_ghostty(),
        starship: default_starship(),
        fastfetch: default_fastfetch(),
        zsh_plugins: default_zsh_plugins(),
        tmux: default_tmux(),
        neovim: default_neovim(),
        tool_configs: default_tool_configs(),
        git: default_git(),
        themes: default_themes(),
        tmux_scripts: default_tmux_scripts(),
        zshrc: default_zshrc(),
    }
}

fn default_clean_artifacts() -> CleanArtifactsConfig {
    CleanArtifactsConfig {
        enabled: true,
        backup_configs: true,
        clean_nvim_state: true,
        remove_catppuccin: true,
        clean_zshrc: true,
        remove_legacy_scripts: true,
    }
}

fn default_packages() -> PackagesConfig {
    PackagesConfig {
        enabled: true,
        install_jetbrains_font: true,
        install_nerd_font: true,
        packages: vec![
            // Core
            pkg("starship", "core"), pkg("eza", "core"), pkg("bat", "core"),
            pkg("fzf", "core"), pkg("ripgrep", "core"), pkg("fd", "core"),
            pkg("zoxide", "core"), pkg("tmux", "core"), pkg("neovim", "core"),
            pkg("atuin", "core"),
            // File/Nav
            pkg("yazi", "file"), pkg("lazygit", "file"), pkg("gh", "file"),
            pkg("trash-cli", "file"),
            // Monitor
            pkg("fastfetch", "monitor"), pkg("btop", "monitor"),
            // Dev tools
            pkg("git-delta", "dev"), pkg("dust", "dev"), pkg("procs", "dev"),
            pkg("hyperfine", "dev"), pkg("tokei", "dev"), pkg("gping", "dev"),
            pkg("curlie", "dev"), pkg("tlrc", "dev"),
            // Media/Util
            pkg("ffmpeg", "util"), pkg("sevenzip", "util"), pkg("poppler", "util"),
            pkg("jq", "util"), pkg("zsh-completions", "util"),
        ],
    }
}

fn pkg(name: &str, category: &str) -> PackageEntry {
    PackageEntry { name: name.into(), enabled: true, category: category.into() }
}

fn default_ghostty() -> GhosttyTermConfig {
    GhosttyTermConfig {
        enabled: true,
        // Font
        font_family: "JetBrains Mono".into(),
        font_family_bold: "JetBrains Mono ExtraBold".into(),
        font_family_italic: "".into(),
        font_family_bold_italic: "".into(),
        font_size: 14.5,
        font_style: "".into(),
        font_style_bold: "".into(),
        font_style_italic: "".into(),
        font_synthetic_style: "true".into(),
        font_features: vec!["liga".into(), "calt".into(), "dlig".into(), "ss01".into(), "ss02".into(), "ss03".into()],
        font_variation: vec![],
        font_thicken: true,
        font_thicken_strength: 0,
        font_shaping_break: "cursor".into(),
        freetype_load_flags: "no-force-autohint".into(),
        // Window
        window_padding_x: 40,
        window_padding_y: 32,
        window_padding_balance: true,
        window_padding_color: "background".into(),
        window_decoration: "auto".into(),
        window_colorspace: "display-p3".into(),
        window_vsync: true,
        window_save_state: "default".into(),
        window_step_resize: false,
        window_new_tab_position: "current".into(),
        window_inherit_working_directory: false,
        window_inherit_font_size: false,
        maximize: false,
        fullscreen: false,
        // macOS
        macos_titlebar_style: "transparent".into(),
        macos_titlebar_proxy_icon: "hidden".into(),
        macos_window_shadow: true,
        macos_option_as_alt: true,
        // Background & Transparency
        background_opacity: 0.93,
        background_blur: 32,
        unfocused_split_opacity: 0.78,
        window_opacity: 1.0,
        minimum_contrast: 1.3,
        bold_is_bright: true,
        // Cursor
        cursor_style: "bar".into(),
        cursor_style_blink: Some(true),
        cursor_opacity: 1.0,
        cursor_click_to_move: true,
        // Cell
        adjust_cell_height: "18%".into(),
        adjust_cell_width: "0".into(),
        adjust_font_baseline: "0".into(),
        adjust_underline_position: "0".into(),
        adjust_underline_thickness: "0".into(),
        adjust_cursor_thickness: "0".into(),
        grapheme_width_method: "unicode".into(),
        // Terminal
        scrollback_limit: 50000,
        shell_integration: "zsh".into(),
        clipboard_read: "allow".into(),
        clipboard_write: "allow".into(),
        clipboard_trim_trailing_spaces: false,
        clipboard_paste_protection: true,
        mouse_hide_while_typing: true,
        mouse_scroll_multiplier: 3.0,
        focus_follows_mouse: false,
        link_url: true,
        confirm_close_surface: "false".into(),
        title: "".into(),
        image_storage_limit: 320,
        // Shaders
        custom_shader_enabled: true,
        custom_shader_path: "~/.config/ghostty/shaders/vignette-bloom.glsl".into(),
        install_community_shaders: true,
        // Keybinds
        keybinds: vec![
            kb("shift+page_up", "scroll_page_up"),
            kb("shift+page_down", "scroll_page_down"),
            kb("super+up", "scroll_page_up"),
            kb("super+down", "scroll_page_down"),
            kb("super+home", "scroll_to_top"),
            kb("super+end", "scroll_to_bottom"),
            kb("super+d", "new_split:right"),
            kb("super+shift+d", "new_split:down"),
            kb("super+alt+left", "goto_split:left"),
            kb("super+alt+right", "goto_split:right"),
            kb("super+alt+up", "goto_split:up"),
            kb("super+alt+down", "goto_split:down"),
            kb("super+1", "goto_tab:1"),
            kb("super+2", "goto_tab:2"),
            kb("super+3", "goto_tab:3"),
            kb("super+4", "goto_tab:4"),
            kb("super+5", "goto_tab:5"),
        ],
        // Quick Terminal
        quick_terminal_position: "".into(),
        quick_terminal_size: "".into(),
        quick_terminal_animation_duration: 0.0,
        quick_terminal_autohide: true,
    }
}

fn kb(key: &str, action: &str) -> KeyBind {
    KeyBind { key: key.into(), action: action.into() }
}

fn default_starship() -> StarshipConfig {
    StarshipConfig {
        enabled: true,
        add_newline: true,
        format: "$directory$fill$git_branch$git_status$nodejs$python$rust$golang$docker_context$cmd_duration$time\n$character".into(),
        command_timeout: 1000,
        modules: vec![
            smod("directory", true), smod("fill", true), smod("character", true),
            smod("git_branch", true), smod("git_status", true),
            smod("nodejs", true), smod("python", true), smod("rust", true),
            smod("golang", true), smod("docker_context", true),
            smod("cmd_duration", true), smod("time", true),
            smod("java", false), smod("ruby", false), smod("php", false),
            smod("dotnet", false), smod("lua", false), smod("swift", false),
            smod("kotlin", false), smod("dart", false), smod("deno", false),
            smod("elixir", false), smod("terraform", false),
            smod("aws", false), smod("gcloud", false), smod("azure", false),
            smod("kubernetes", false), smod("package", false),
            smod("memory_usage", false), smod("battery", false),
            smod("username", false), smod("hostname", false),
        ],
        character_success_symbol: "[❯](blue)".into(),
        character_error_symbol: "[❯](red)".into(),
        character_vicmd_symbol: "[❮❮❮](bold yellow)".into(),
        directory_style: "bold blue".into(),
        directory_format: "[$path]($style) ".into(),
        directory_truncation_length: 3,
        directory_truncation_symbol: "…/".into(),
        git_branch_symbol: " ".into(),
        git_branch_style: "fg:#8b949e".into(),
        git_branch_format: "[$symbol$branch]($style) ".into(),
        git_status_style: "red".into(),
        git_status_format: "[$all_status$ahead_behind]($style) ".into(),
        cmd_duration_min_time: 1000,
        cmd_duration_style: "yellow".into(),
        cmd_duration_format: "[✦ $duration]($style) ".into(),
        time_disabled: false,
        time_style: "bold fg:#8b949e".into(),
        time_format: "[$time]($style)".into(),
        time_time_format: "%I:%M %p".into(),
        fill_symbol: " ".into(),
        nodejs_symbol: " ".into(),
        nodejs_style: "green".into(),
        python_symbol: " ".into(),
        python_style: "yellow".into(),
        rust_symbol: " ".into(),
        rust_style: "red".into(),
        golang_symbol: " ".into(),
        golang_style: "cyan".into(),
        docker_symbol: " ".into(),
        docker_style: "blue".into(),
        docker_only_with_files: true,
    }
}

fn smod(name: &str, enabled: bool) -> StarshipModule {
    StarshipModule { name: name.into(), enabled }
}

fn default_fastfetch() -> FastfetchConfig {
    FastfetchConfig {
        enabled: true,
        logo_type: "small".into(),
        logo_color_1: "34".into(),
        logo_color_2: "36".into(),
        separator: "  ".into(),
        key_width: 10,
        modules: vec![
            ffmod("title", None, Some("{user-name-colored}"), None, None),
            ffmod("separator", None, None, None, Some("─")),
            ffmod("command", Some("   "), None, Some("date '+%a %b %d   %I:%M %p'"), None),
            ffmod("os", Some("  OS"), None, None, None),
            ffmod("uptime", Some("  Up"), None, None, None),
            ffmod("terminal", Some("  Term"), None, None, None),
            ffmod("shell", Some("  Shell"), None, None, None),
            ffmod("cpu", Some("  CPU"), None, None, None),
            ffmod("gpu", Some("  GPU"), None, None, None),
            ffmod("memory", Some("  RAM"), None, None, None),
            ffmod("disk", Some("  Disk"), None, None, None),
            ffmod("packages", Some("  Pkgs"), None, None, None),
            ffmod("localip", Some("  IP"), None, None, None),
            ffmod("battery", Some("  Batt"), None, None, None),
            ffmod("separator", None, None, None, Some("─")),
            ffmod("command", Some("❯ "), None, Some("shuf -n1 ~/.config/fastfetch/quotes.txt 2>/dev/null || echo 'The obstacle is the way.'"), None),
            ffmod("custom", None, Some("❯ {#35} tmux{#}  {#36}cc{#} claude  {#36}cx{#} codex  {#36}gm{#} gemini  {#36}ai{#} workspace"), None, None),
            ffmod("break", None, None, None, None),
            ffmod("colors", None, None, None, None),
        ],
        quotes: vec![
            "The impediment to action advances action. What stands in the way becomes the way.".into(),
            "You have power over your mind, not outside events. Realize this and you will find strength.".into(),
            "Waste no more time arguing what a good man should be. Be one.".into(),
            "Loss is nothing else but change, and change is nature's delight.".into(),
            "The best revenge is to be unlike him who performed the injustice.".into(),
            "Man is not disturbed by events, but by the opinions he has of events.".into(),
            "Concentrate every minute on doing what is in front of you with precise elegance.".into(),
        ],
        show_color_circles: true,
    }
}

fn ffmod(t: &str, key: Option<&str>, format: Option<&str>, text: Option<&str>, string: Option<&str>) -> FastfetchModule {
    FastfetchModule {
        module_type: t.into(),
        key: key.map(|s| s.into()),
        format: format.map(|s| s.into()),
        text: text.map(|s| s.into()),
        string: string.map(|s| s.into()),
        enabled: true,
    }
}

fn default_zsh_plugins() -> ZshPluginsConfig {
    ZshPluginsConfig {
        enabled: true,
        plugins: vec![
            ZshPlugin { name: "zsh-autosuggestions".into(), repo: "https://github.com/zsh-users/zsh-autosuggestions".into(), enabled: true },
            ZshPlugin { name: "zsh-syntax-highlighting".into(), repo: "https://github.com/zsh-users/zsh-syntax-highlighting".into(), enabled: true },
            ZshPlugin { name: "fzf-tab".into(), repo: "https://github.com/Aloxaf/fzf-tab".into(), enabled: true },
            ZshPlugin { name: "zsh-vi-mode".into(), repo: "https://github.com/jeffreytse/zsh-vi-mode".into(), enabled: true },
        ],
        yazi: YaziConfig {
            ratio: [1, 4, 3],
            sort_by: "alphabetical".into(),
            sort_sensitive: false,
            sort_reverse: false,
            sort_dir_first: true,
            show_hidden: false,
            show_symlink: true,
            scrolloff: 5,
            mouse_events: vec!["click".into(), "scroll".into(), "touch".into()],
            linemode: "none".into(),
            title_format: "".into(),
            preview_tab_size: 2,
            preview_max_width: 800,
            preview_max_height: 600,
            preview_image_filter: "lanczos3".into(),
            preview_image_quality: 75,
        },
    }
}

fn default_tmux() -> TmuxConfig {
    TmuxConfig {
        enabled: true,
        prefix_key: "C-a".into(),
        default_terminal: "tmux-256color".into(),
        mouse: true,
        history_limit: 50000,
        base_index: 1,
        pane_base_index: 1,
        renumber_windows: true,
        set_clipboard: true,
        escape_time: 0,
        focus_events: true,
        status_position: "top".into(),
        status_left_length: 30,
        status_right_length: 60,
        mode_keys: "vi".into(),
        pane_border_lines: "heavy".into(),
        pane_border_indicators: "both".into(),
        plugins: vec![
            tplugin("tpm", "tmux-plugins/tpm"),
            tplugin("tmux-sensible", "tmux-plugins/tmux-sensible"),
            tplugin("vim-tmux-navigator", "christoomey/vim-tmux-navigator"),
            tplugin("tmux-resurrect", "tmux-plugins/tmux-resurrect"),
            tplugin("tmux-continuum", "tmux-plugins/tmux-continuum"),
            tplugin("tmux-yank", "tmux-plugins/tmux-yank"),
            tplugin("tmux-fzf", "sainnhe/tmux-fzf"),
            tplugin("tmux-sessionx", "omerxx/tmux-sessionx"),
            tplugin("tmux-floax", "omerxx/tmux-floax"),
        ],
        floax_width: "80%".into(),
        floax_height: "80%".into(),
        floax_border_color: "magenta".into(),
        floax_text_color: "blue".into(),
        floax_bind: "p".into(),
        floax_change_path: true,
        sessionx_bind: "o".into(),
        sessionx_window_height: "85%".into(),
        sessionx_window_width: "75%".into(),
        sessionx_zoxide_mode: true,
        sessionx_filter_current: false,
        continuum_restore: true,
        resurrect_strategy_nvim: "session".into(),
        resurrect_capture_pane_contents: true,
        popup_bindings: vec![
            popup("C-c", "claude", "80%", "80%"),
            popup("C-x", "codex", "80%", "80%"),
            popup("T", "zsh", "60%", "50%"),
            popup("g", "lazygit 2>/dev/null || git log --oneline --graph -20", "90%", "90%"),
            popup("f", "yazi", "90%", "90%"),
            popup("b", "btop 2>/dev/null || htop 2>/dev/null || top", "90%", "90%"),
        ],
    }
}

fn tplugin(name: &str, repo: &str) -> TmuxPlugin {
    TmuxPlugin { name: name.into(), repo: repo.into(), enabled: true }
}

fn popup(key: &str, cmd: &str, w: &str, h: &str) -> PopupBinding {
    PopupBinding { key: key.into(), command: cmd.into(), width: w.into(), height: h.into() }
}

fn default_neovim() -> NeovimConfig {
    NeovimConfig {
        enabled: true,
        theme: "ayu-dark".into(),
        theme_bg_override: "#0b0e14".into(),
        leader_key: " ".into(),
        number: true,
        relative_number: true,
        cursorline: true,
        scrolloff: 8,
        expandtab: true,
        shiftwidth: 2,
        tabstop: 2,
        smartindent: true,
        mouse: "a".into(),
        clipboard: "unnamedplus".into(),
        undofile: true,
        ignorecase: true,
        smartcase: true,
        signcolumn: "yes".into(),
        termguicolors: true,
        plugins: vec![
            nplugin("LazyVim", true, "Plugin framework"),
            nplugin("neovim-ayu", true, "Ayu colorscheme"),
            nplugin("indent-blankline", true, "Indent guides"),
            nplugin("mini.animate", true, "Smooth animations"),
            nplugin("vim-illuminate", true, "Highlight word under cursor"),
            nplugin("vim-tmux-navigator", true, "Seamless tmux/nvim nav"),
            nplugin("neo-tree", true, "File explorer"),
            nplugin("bufferline", true, "Tab line"),
            nplugin("noice", true, "UI for messages/cmdline"),
        ],
    }
}

fn nplugin(name: &str, enabled: bool, desc: &str) -> NvimPlugin {
    NvimPlugin { name: name.into(), enabled, description: desc.into() }
}

fn default_tool_configs() -> ToolConfigsConfig {
    ToolConfigsConfig {
        enabled: true,
        atuin: AtuinConfig {
            style: "full".into(),
            inline_height: 0,
            show_preview: true,
            filter_mode: "global".into(),
            filter_mode_shell_up_key_binding: "directory".into(),
            search_mode: "fuzzy".into(),
            show_tabs: true,
            timestamps_enabled: true,
            time_format: "%I:%M %p".into(),
            exit_mode: "return-original".into(),
            sync_records: true,
            store_failed: true,
            secrets_filter: true,
            enter_accept: false,
            keymap_mode: "emacs".into(),
            workspaces: false,
            invert: false,
            show_help: true,
            max_preview_height: 4,
            prefers_reduced_motion: false,
        },
        lazygit: LazygitConfig {
            nerd_fonts_version: "3".into(),
            show_file_icons: true,
            border: "rounded".into(),
            mouse_events: true,
            show_command_log: false,
            pager: "delta --dark --paging=never".into(),
            edit_preset: "nvim".into(),
        },
    }
}

fn default_git() -> GitConfig {
    GitConfig {
        enabled: true,
        delta: DeltaConfig {
            navigate: true,
            dark: true,
            line_numbers: true,
            side_by_side: true,
            syntax_theme: "ayu-dark".into(),
            file_style: "bold yellow".into(),
            file_decoration_style: "yellow ul".into(),
            hunk_header_style: "syntax bold".into(),
            hunk_header_decoration_style: "blue box".into(),
            minus_style: "syntax #2d1517".into(),
            plus_style: "syntax #152e1a".into(),
            minus_emph_style: "syntax #4a1c1f".into(),
            plus_emph_style: "syntax #1e4020".into(),
            zero_style: "syntax".into(),
            whitespace_error_style: "reverse red".into(),
            tabs: 8,
            max_line_length: 3000,
            wrap_max_lines: 2,
            true_color: "auto".into(),
            hyperlinks: false,
        },
        merge_conflictstyle: "diff3".into(),
        diff_algorithm: "histogram".into(),
        diff_color_moved: "default".into(),
        pull_rebase: true,
        rebase_autostash: true,
        push_auto_setup_remote: true,
        push_default: "current".into(),
        init_default_branch: "main".into(),
        core_editor: "nvim".into(),
        help_autocorrect: 20,
        aliases: vec![
            galias("s", "status -sb"),
            galias("co", "checkout"),
            galias("br", "branch"),
            galias("ci", "commit"),
            galias("lg", "log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"),
            galias("last", "log -1 HEAD --stat"),
            galias("unstage", "reset HEAD --"),
            galias("amend", "commit --amend --no-edit"),
            galias("undo", "reset --soft HEAD~1"),
            galias("wip", "!git add -A && git commit -m \"wip\""),
            galias("cleanup", "!git branch --merged | grep -v \"\\\\*\\\\|main\\\\|master\" | xargs -n 1 git branch -d"),
        ],
    }
}

fn galias(name: &str, cmd: &str) -> GitAlias {
    GitAlias { name: name.into(), command: cmd.into(), enabled: true }
}

fn default_themes() -> ThemesConfig {
    ThemesConfig {
        enabled: true,
        btop: BtopConfig {
            color_theme: "ayu".into(),
            theme_background: false,
            truecolor: true,
            shown_boxes: "cpu mem net proc".into(),
            update_ms: 1000,
            proc_sorting: "cpu lazy".into(),
            proc_tree: false,
            proc_per_core: false,
            rounded_corners: true,
        },
        bat_theme_enabled: true,
        yazi_theme_enabled: true,
        fzf: FzfConfig {
            border: "rounded".into(),
            prompt: "❯ ".into(),
            marker: "✓".into(),
            pointer: "▶".into(),
            separator: "─".into(),
            scrollbar: "│".into(),
            layout: "reverse".into(),
            height: "60%".into(),
        },
    }
}

fn default_tmux_scripts() -> TmuxScriptsConfig {
    TmuxScriptsConfig {
        enabled: true,
        tmux_ai_enabled: true,
        tmux_pair_enabled: true,
        tmux_review_enabled: true,
        tmux_dev_enabled: true,
        tmux_cheat_enabled: true,
    }
}

fn default_zshrc() -> ZshrcConfig {
    ZshrcConfig {
        enabled: true,
        aliases: vec![
            // Files
            alias("ls", "eza --color=always --icons --group-directories-first", "files"),
            alias("ll", "eza -la --color=always --icons --group-directories-first", "files"),
            alias("la", "eza -a --color=always --icons --group-directories-first", "files"),
            alias("lt", "eza --tree --level=2 --color=always --icons", "files"),
            alias("tree", "eza --tree --color=always --icons", "files"),
            // Tools
            alias("cat", "bat --paging=never", "tools"),
            alias("grep", "rg", "tools"),
            alias("find", "fd", "tools"),
            alias("du", "dust", "tools"),
            alias("ps", "procs", "tools"),
            alias("ping", "gping", "tools"),
            alias("curl", "curlie", "tools"),
            alias("tldr", "tlrc", "tools"),
            alias("loc", "tokei", "tools"),
            alias("bench", "hyperfine", "tools"),
            // Editor
            alias("vim", "nvim", "editor"),
            alias("vi", "nvim", "editor"),
            // Git
            alias("lg", "lazygit", "git"),
            alias("g", "git", "git"),
            alias("gs", "git status -sb", "git"),
            alias("ga", "git add", "git"),
            alias("gc", "git commit", "git"),
            alias("gp", "git push", "git"),
            alias("gl", "git log --oneline --graph -20", "git"),
            alias("gd", "git diff", "git"),
            alias("gds", "git diff --staged", "git"),
            alias("gco", "git checkout", "git"),
            alias("gb", "git branch", "git"),
            // Nav
            alias("..", "cd ..", "nav"),
            alias("...", "cd ../..", "nav"),
            alias("....", "cd ../../..", "nav"),
            // Tmux
            alias("ta", "tmux attach -t", "tmux"),
            alias("tl", "tmux list-sessions", "tmux"),
            alias("tk", "tmux kill-session -t", "tmux"),
            // Util
            alias("myip", "curl -s ifconfig.me", "util"),
            alias("localip", "ipconfig getifaddr en0 2>/dev/null || echo \"N/A\"", "util"),
            alias("ports", "lsof -i -P -n | grep LISTEN", "util"),
            alias("serve", "python3 -m http.server", "util"),
            alias("wt", "curl -s wttr.in/?format=3", "util"),
            alias("update", "brew update && brew upgrade && brew cleanup", "util"),
            alias("path", "echo $PATH | tr \":\" \"\\n\"", "util"),
            alias("reload", "source ~/.zshrc && echo \"  ✓ zshrc reloaded\"", "util"),
            alias("week", "date +%V", "util"),
            alias("timestamp", "date +%Y%m%d_%H%M%S", "util"),
            // Safety
            alias("rm", "trash", "safety"),
        ],
        functions: vec![
            func("cc", r#"if [ -n "$TMUX" ]; then command claude "$@"; else local s="claude"; tmux has-session -t "$s" 2>/dev/null && tmux attach -t "$s" || tmux new-session -s "$s" -c "${PWD}" "claude $*; zsh"; fi"#, "Claude CLI wrapper"),
            func("cx", r#"if [ -n "$TMUX" ]; then command codex "$@"; else local s="codex"; tmux has-session -t "$s" 2>/dev/null && tmux attach -t "$s" || tmux new-session -s "$s" -c "${PWD}" "codex $*; zsh"; fi"#, "Codex CLI wrapper"),
            func("gm", r#"if [ -n "$TMUX" ]; then command gemini "$@"; else local s="gemini"; tmux has-session -t "$s" 2>/dev/null && tmux attach -t "$s" || tmux new-session -s "$s" -c "${PWD}" "gemini $*; zsh"; fi"#, "Gemini CLI wrapper"),
            func("ai", r#"local d="${1:-$PWD}"; [ -n "$TMUX" ] && { echo "▲ Already in tmux."; return; }; tmux-ai "$d""#, "AI workspace launcher"),
            func("y", r#"local t="$(mktemp -t yazi-cwd.XXXXXX)"; yazi "$@" --cwd-file="$t"; if cwd="$(cat -- "$t")" && [ -n "$cwd" ] && [ "$cwd" != "$PWD" ]; then builtin cd -- "$cwd"; fi; rm -f -- "$t""#, "Yazi with cd on exit"),
            func("fcd", r#"local d; d=$(fd --type d --hidden --follow --exclude .git | fzf --preview 'eza --tree --level=2 --color=always --icons {}') && cd "$d""#, "Fuzzy cd"),
            func("fvim", r#"local f; f=$(fd --type f --hidden --follow --exclude .git | fzf --preview 'bat -n --color=always --line-range :200 {}') && nvim "$f""#, "Fuzzy open in nvim"),
            func("ff", r#"local r; r=$(rg --color=always --line-number --no-heading "${1:-.}" | fzf --ansi --delimiter ':') || return; nvim "+$(echo "$r"|cut -d: -f2)" "$(echo "$r"|cut -d: -f1)""#, "Fuzzy grep + open"),
            func("mkcd", r#"mkdir -p "$1" && cd "$1""#, "Make dir and cd"),
            func("toclaude", r#"local i; [ -t 0 ] && i="$*" || i=$(cat); echo "$i" | claude"#, "Pipe to Claude"),
            func("askc", r#"[ -f "$1" ] && cat "$1" | claude "${2:-explain this code}" || echo "✗ File not found: $1""#, "Ask Claude about file"),
            func("gcai", r#"local d=$(git diff --staged); [ -z "$d" ] && { echo "▲ No staged changes."; return 1; }; echo "$d" | claude "Write a concise git commit message. Output ONLY the message." | git commit -F -"#, "AI git commit message"),
            func("hx", r#"curl -sI "$1" | bat --language=http --style=plain"#, "HTTP headers"),
            func("extract", r#"case "$1" in *.tar.bz2) tar xjf "$1" ;; *.tar.gz) tar xzf "$1" ;; *.tar.xz) tar xJf "$1" ;; *.zip) unzip "$1" ;; *.7z) 7zz x "$1" ;; *.gz) gunzip "$1" ;; *) echo "✗ Unknown format: $1" ;; esac"#, "Extract archives"),
            func("tmpdir", r#"local t=$(mktemp -d); echo "  → $t"; cd "$t""#, "Create and cd to tempdir"),
        ],
        histsize: 50000,
        savehist: 50000,
        editor: "nvim".into(),
        vi_mode_enabled: true,
        vi_mode_escape_key: "jk".into(),
        autosuggest_style: "fg=#8b949e".into(),
        autosuggest_strategy: vec!["history".into(), "completion".into()],
        show_fastfetch_on_start: true,
        show_tmux_cheat_on_start: true,
        bat_theme: "Ayu Dark".into(),
        manpager_enabled: true,
    }
}

fn alias(name: &str, cmd: &str, cat: &str) -> AliasEntry {
    AliasEntry { name: name.into(), command: cmd.into(), category: cat.into(), enabled: true }
}

fn func(name: &str, body: &str, desc: &str) -> FunctionEntry {
    FunctionEntry { name: name.into(), body: body.into(), description: desc.into(), enabled: true }
}
