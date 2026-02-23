<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";

  let config = $derived(configState.config.starship);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.starship));
  $effect(() => {
    const curr = JSON.stringify(configState.config.starship);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("starship");
    }
  });

  function toggleModule(index: number) {
    configState.config.starship.modules[index].enabled =
      !configState.config.starship.modules[index].enabled;
  }
</script>

<section class="section-panel">
  <SectionHeader icon="🚀" number={4} title="Starship" description="Cross-shell prompt configuration" />

  <!-- General -->
  <div class="sub-section">
    <h3 class="sub-heading">General</h3>
    <div class="field-group">
      <Toggle
        label="Add Newline"
        description="Insert blank line between prompts"
        bind:value={configState.config.starship.add_newline}
      />
      <NumberInput
        label="Command Timeout"
        description="Timeout for commands in milliseconds"
        bind:value={configState.config.starship.command_timeout}
        min={100}
        max={10000}
        step={100}
      />
    </div>
  </div>

  <!-- Modules -->
  <div class="sub-section">
    <h3 class="sub-heading">Modules</h3>
    <div class="module-list">
      {#each config.modules as mod, i}
        <label class="module-item">
          <input
            type="checkbox"
            checked={mod.enabled}
            onchange={() => toggleModule(i)}
          />
          <span class="module-name">{mod.name}</span>
        </label>
      {/each}
      {#if config.modules.length === 0}
        <p class="empty-hint">No modules configured. Modules will appear once loaded from the backend.</p>
      {/if}
    </div>
  </div>

  <!-- Character Symbols -->
  <div class="sub-section">
    <h3 class="sub-heading">Character Symbols</h3>
    <div class="field-group">
      <TextInput
        label="Success Symbol"
        description="Prompt symbol on command success"
        bind:value={configState.config.starship.character_success_symbol}
        monospace
      />
      <TextInput
        label="Error Symbol"
        description="Prompt symbol on command failure"
        bind:value={configState.config.starship.character_error_symbol}
        monospace
      />
      <TextInput
        label="Vi Command Symbol"
        description="Prompt symbol in vi normal mode"
        bind:value={configState.config.starship.character_vicmd_symbol}
        monospace
      />
    </div>
  </div>

  <!-- Directory -->
  <div class="sub-section">
    <h3 class="sub-heading">Directory</h3>
    <div class="field-group">
      <TextInput
        label="Style"
        description="Prompt directory display style (bold, dimmed, etc.)"
        bind:value={configState.config.starship.directory_style}
        monospace
      />
      <NumberInput
        label="Truncation Length"
        description="Number of parent directories to show"
        bind:value={configState.config.starship.directory_truncation_length}
        min={1}
        max={20}
      />
      <TextInput
        label="Truncation Symbol"
        description="Character shown when path is truncated"
        bind:value={configState.config.starship.directory_truncation_symbol}
        monospace
      />
    </div>
  </div>

  <!-- Time -->
  <div class="sub-section">
    <h3 class="sub-heading">Time</h3>
    <div class="field-group">
      <Toggle
        label="Disabled"
        description="Hide the time module"
        bind:value={configState.config.starship.time_disabled}
      />
      <TextInput
        label="Time Format"
        description="Starship format string for time display"
        bind:value={configState.config.starship.time_format}
        monospace
      />
      <TextInput
        label="Time strftime Format"
        description="strftime format string (e.g. %I:%M %p)"
        bind:value={configState.config.starship.time_time_format}
        monospace
      />
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

  .module-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 4px 16px;
  }

  .module-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 0;
    cursor: pointer;
    font-size: 13px;
    color: var(--fg);
  }

  .module-item input[type="checkbox"] {
    accent-color: var(--blue);
    width: 14px;
    height: 14px;
  }

  .module-name {
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
  }

  .empty-hint {
    font-size: 12px;
    color: var(--comment);
    margin: 4px 0;
    grid-column: 1 / -1;
  }
</style>
