<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import { uiState } from "$lib/state/ui.svelte";
  import { SECTIONS } from "$lib/api/types";

  function getSectionEnabled(id: string): boolean {
    const c = configState.config;
    const map: Record<string, boolean> = {
      clean_artifacts: c.clean_artifacts.enabled,
      packages: c.packages.enabled,
      ghostty: c.ghostty.enabled,
      starship: c.starship.enabled,
      fastfetch: c.fastfetch.enabled,
      zsh_plugins: c.zsh_plugins.enabled,
      tmux: c.tmux.enabled,
      neovim: c.neovim.enabled,
      tool_configs: c.tool_configs.enabled,
      git: c.git.enabled,
      themes: c.themes.enabled,
      tmux_scripts: c.tmux_scripts.enabled,
      zshrc: c.zshrc.enabled,
    };
    return map[id] ?? true;
  }

  function toggleSection(id: string) {
    const key = id as keyof typeof configState.config;
    const section = configState.config[key] as { enabled: boolean };
    configState.updateField(key, "enabled", !section.enabled);
  }
</script>

<nav class="sidebar" class:collapsed={uiState.sidebarCollapsed}>
  <div class="sidebar-header">
    <button
      class="palette-btn"
      onclick={() => (uiState.showPaletteEditor = !uiState.showPaletteEditor)}
    >
      <span class="palette-dots">
        <span class="dot" style="background:{configState.config.palette.red}"></span>
        <span class="dot" style="background:{configState.config.palette.green}"></span>
        <span class="dot" style="background:{configState.config.palette.blue}"></span>
        <span class="dot" style="background:{configState.config.palette.purple}"></span>
      </span>
      Palette
    </button>
  </div>
  <div class="sections">
    {#each SECTIONS as section}
      <button
        class="section-item"
        class:active={uiState.activeSection === section.id}
        onclick={() => (uiState.activeSection = section.id)}
      >
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <span class="section-toggle" role="switch" tabindex={0} aria-checked={getSectionEnabled(section.id)} aria-label="Toggle {section.label}" onclick={(e: MouseEvent) => { e.stopPropagation(); toggleSection(section.id); }} onkeydown={(e: KeyboardEvent) => { if (e.key === 'Enter' || e.key === ' ') { e.stopPropagation(); e.preventDefault(); toggleSection(section.id); } }}>
          <span
            class="toggle-dot"
            class:enabled={getSectionEnabled(section.id)}
          ></span>
        </span>
        <span class="section-icon">{section.icon}</span>
        <span class="section-label">
          <span class="section-num">{section.number}/13</span>
          {section.label}
        </span>
      </button>
    {/each}
  </div>
</nav>

<style>
  .sidebar {
    width: 220px;
    background: color-mix(in srgb, var(--surface) 60%, var(--bg));
    border-right: 1px solid color-mix(in srgb, var(--comment) 15%, transparent);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    flex-shrink: 0;
  }

  .sidebar.collapsed {
    width: 48px;
  }

  .sidebar-header {
    padding: 12px;
    border-bottom: 1px solid color-mix(in srgb, var(--comment) 15%, transparent);
  }

  .palette-btn {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
    border-radius: 8px;
    padding: 8px 10px;
    color: var(--fg);
    cursor: pointer;
    font-family: inherit;
    font-size: 12px;
    transition: border-color 0.15s;
  }
  .palette-btn:hover {
    border-color: var(--purple);
  }

  .palette-dots {
    display: flex;
    gap: 3px;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .sections {
    flex: 1;
    padding: 8px 6px;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .section-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border: none;
    background: transparent;
    color: var(--comment);
    border-radius: 6px;
    cursor: pointer;
    font-family: inherit;
    font-size: 12px;
    text-align: left;
    transition: all 0.12s;
  }

  .section-item:hover {
    background: color-mix(in srgb, var(--selection) 50%, transparent);
    color: var(--fg);
  }

  .section-item.active {
    background: var(--selection);
    color: var(--fg);
  }

  .section-toggle {
    flex-shrink: 0;
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toggle-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: color-mix(in srgb, var(--comment) 40%, transparent);
    transition: background 0.15s;
  }
  .toggle-dot.enabled {
    background: var(--green);
  }

  .section-icon {
    font-size: 14px;
    flex-shrink: 0;
  }

  .section-label {
    display: flex;
    align-items: center;
    gap: 6px;
    overflow: hidden;
  }

  .section-num {
    font-size: 10px;
    color: var(--comment);
    opacity: 0.6;
  }
</style>
