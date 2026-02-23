<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";

  let config = $derived(configState.config.zshrc);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.zshrc));
  $effect(() => {
    const curr = JSON.stringify(configState.config.zshrc);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("zshrc");
    }
  });

  function toggleAlias(index: number) {
    configState.config.zshrc.aliases[index].enabled =
      !configState.config.zshrc.aliases[index].enabled;
  }

  function toggleFunction(index: number) {
    configState.config.zshrc.functions[index].enabled =
      !configState.config.zshrc.functions[index].enabled;
  }

  // Group aliases by category
  let aliasGroups = $derived.by(() => {
    const groups: Record<string, { alias: typeof config.aliases[number]; index: number }[]> = {};
    config.aliases.forEach((alias, index) => {
      const cat = alias.category || "other";
      if (!groups[cat]) groups[cat] = [];
      groups[cat].push({ alias, index });
    });
    return groups;
  });

  const categoryLabels: Record<string, string> = {
    files: "Files",
    tools: "Tools",
    editor: "Editor",
    git: "Git",
    nav: "Navigation",
    tmux: "tmux",
    util: "Utilities",
    safety: "Safety",
    other: "Other",
  };

  const categoryOrder = ["files", "tools", "editor", "git", "nav", "tmux", "util", "safety", "other"];

  let orderedCategories = $derived(
    categoryOrder.filter((c) => aliasGroups[c]?.length)
  );
</script>

<section class="section-panel">
  <SectionHeader icon="🏠" number={13} title="Zshrc" description="Zsh shell configuration, aliases, and functions" />

  <!-- Settings -->
  <div class="sub-section">
    <h3 class="sub-heading">Settings</h3>
    <div class="field-group">
      <div class="field-row">
        <NumberInput
          label="History Size"
          bind:value={configState.config.zshrc.histsize}
          min={1000}
          max={500000}
          step={1000}
          description="Number of commands kept in memory"
        />
        <NumberInput
          label="Save History"
          bind:value={configState.config.zshrc.savehist}
          min={1000}
          max={500000}
          step={1000}
          description="Number of commands saved to history file"
        />
      </div>
      <TextInput
        label="Editor"
        bind:value={configState.config.zshrc.editor}
        placeholder="nvim"
        description="Default EDITOR and VISUAL value"
      />
      <Toggle
        label="Vi Mode"
        description="Enable vi-style key bindings in zsh"
        bind:value={configState.config.zshrc.vi_mode_enabled}
      />
      <TextInput
        label="Vi Mode Escape Key"
        bind:value={configState.config.zshrc.vi_mode_escape_key}
        placeholder="jk"
        description="Key sequence to exit insert mode"
        monospace
      />
      <TextInput
        label="bat Theme"
        bind:value={configState.config.zshrc.bat_theme}
        placeholder="Ayu Dark"
        description="Syntax theme for bat preview"
      />
      <Toggle
        label="Man Pager"
        description="Use bat as the pager for man pages"
        bind:value={configState.config.zshrc.manpager_enabled}
      />
      <Toggle
        label="Show Fastfetch on Start"
        description="Display system info when opening a new shell"
        bind:value={configState.config.zshrc.show_fastfetch_on_start}
      />
      <Toggle
        label="Show tmux Cheat on Start"
        description="Display tmux keybinding cheat sheet on shell start"
        bind:value={configState.config.zshrc.show_tmux_cheat_on_start}
      />
    </div>
  </div>

  <!-- Aliases -->
  <div class="sub-section">
    <h3 class="sub-heading">Aliases</h3>
    <div class="field-group">
      {#if config.aliases.length === 0}
        <p class="empty-hint">No aliases configured yet.</p>
      {:else}
        {#each orderedCategories as cat}
          <h4 class="category-title">{categoryLabels[cat] || cat}</h4>
          {#each aliasGroups[cat] as { alias, index }}
            <div class="alias-row">
              <label class="alias-toggle">
                <input
                  type="checkbox"
                  checked={alias.enabled}
                  onchange={() => toggleAlias(index)}
                />
                <span class="alias-name">{alias.name}</span>
                <span class="alias-arrow">&rarr;</span>
                <span class="alias-command">{alias.command}</span>
              </label>
            </div>
          {/each}
        {/each}
      {/if}
    </div>
  </div>

  <!-- Functions -->
  <div class="sub-section">
    <h3 class="sub-heading">Functions</h3>
    <div class="field-group">
      {#if config.functions.length === 0}
        <p class="empty-hint">No functions configured yet.</p>
      {:else}
        {#each config.functions as func, i}
          <Toggle
            label={func.name}
            description={func.description}
            bind:value={configState.config.zshrc.functions[i].enabled}
          />
        {/each}
      {/if}
    </div>
  </div>
</section>

<style>
  .section-panel {
    padding: 24px;
  }

  .sub-section {
    margin-bottom: 20px;
  }

  .sub-heading {
    font-size: 13px;
    font-weight: 600;
    color: var(--blue);
    margin: 16px 0 8px 0;
    padding-bottom: 4px;
    border-bottom: 1px solid color-mix(in srgb, var(--comment) 10%, transparent);
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .field-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    align-items: start;
  }

  .category-title {
    font-size: 11px;
    font-weight: 600;
    color: var(--cyan);
    margin: 14px 0 4px 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .category-title:first-child {
    margin-top: 0;
  }

  .empty-hint {
    font-size: 12px;
    color: var(--comment);
    font-style: italic;
    margin: 8px 0;
  }

  .alias-row {
    padding: 4px 0;
  }

  .alias-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 13px;
    color: var(--fg);
  }

  .alias-toggle input[type="checkbox"] {
    accent-color: var(--blue);
    width: 14px;
    height: 14px;
  }

  .alias-name {
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
    font-weight: 600;
    color: var(--cyan);
    min-width: 60px;
  }

  .alias-arrow {
    color: var(--comment);
    font-size: 11px;
  }

  .alias-command {
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
    color: var(--comment);
  }
</style>
