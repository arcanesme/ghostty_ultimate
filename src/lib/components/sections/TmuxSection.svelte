<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import type { PopupBinding } from "$lib/api/types";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";
  import Select from "$lib/components/shared/Select.svelte";

  let config = $derived(configState.config.tmux);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.tmux));
  $effect(() => {
    const curr = JSON.stringify(configState.config.tmux);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("tmux");
    }
  });

  function togglePlugin(index: number) {
    configState.config.tmux.plugins[index].enabled =
      !configState.config.tmux.plugins[index].enabled;
  }

  function addPopupBinding() {
    configState.config.tmux.popup_bindings = [
      ...configState.config.tmux.popup_bindings,
      { key: "", command: "", width: "80%", height: "80%" },
    ];
  }

  function removePopupBinding(index: number) {
    configState.config.tmux.popup_bindings =
      configState.config.tmux.popup_bindings.filter((_, i) => i !== index);
  }

  function updatePopupField(index: number, field: keyof PopupBinding, value: string) {
    configState.config.tmux.popup_bindings[index][field] = value;
  }
</script>

<section class="section-panel">
  <SectionHeader icon="🖥️" number={7} title="tmux" description="Terminal multiplexer configuration" />

  <!-- General -->
  <div class="sub-section">
    <h3 class="sub-heading">General</h3>
    <div class="field-group">
      <TextInput
        label="Prefix Key"
        description="tmux prefix key binding (e.g. C-a, C-b)"
        bind:value={configState.config.tmux.prefix_key}
        monospace
      />
      <Toggle
        label="Mouse"
        description="Enable mouse support"
        bind:value={configState.config.tmux.mouse}
      />
      <NumberInput
        label="History Limit"
        description="Maximum scrollback lines per pane"
        bind:value={configState.config.tmux.history_limit}
        min={1000}
        max={500000}
        step={1000}
      />
      <div class="field-row">
        <NumberInput
          label="Base Index"
          description="Starting index for windows"
          bind:value={configState.config.tmux.base_index}
          min={0}
          max={10}
        />
        <NumberInput
          label="Escape Time"
          description="Delay for escape key (ms)"
          bind:value={configState.config.tmux.escape_time}
          min={0}
          max={1000}
        />
      </div>
      <div class="field-row">
        <Select
          label="Status Position"
          description="Position of the tmux status bar"
          bind:value={configState.config.tmux.status_position}
          options={[
            { value: "top", label: "Top" },
            { value: "bottom", label: "Bottom" },
          ]}
        />
        <Select
          label="Mode Keys"
          description="Key binding style for copy mode"
          bind:value={configState.config.tmux.mode_keys}
          options={[
            { value: "vi", label: "vi" },
            { value: "emacs", label: "emacs" },
          ]}
        />
      </div>
    </div>
  </div>

  <!-- Plugins -->
  <div class="sub-section">
    <h3 class="sub-heading">Plugins</h3>
    <div class="plugin-list">
      {#each config.plugins as plugin, i}
        <div class="plugin-item">
          <label class="plugin-toggle">
            <input
              type="checkbox"
              checked={plugin.enabled}
              onchange={() => togglePlugin(i)}
            />
            <span class="plugin-name">{plugin.name}</span>
          </label>
          <span class="plugin-repo">{plugin.repo}</span>
        </div>
      {/each}
      {#if config.plugins.length === 0}
        <p class="empty-hint">No plugins configured. Plugins will appear once loaded from the backend.</p>
      {/if}
    </div>
  </div>

  <!-- Floax -->
  <div class="sub-section">
    <h3 class="sub-heading">Floax (Floating Panes)</h3>
    <div class="field-group">
      <div class="field-row">
        <TextInput
          label="Width"
          description="Floating pane width (percentage or columns)"
          bind:value={configState.config.tmux.floax_width}
          monospace
          placeholder="80%"
        />
        <TextInput
          label="Height"
          description="Floating pane height (percentage or rows)"
          bind:value={configState.config.tmux.floax_height}
          monospace
          placeholder="80%"
        />
      </div>
      <TextInput
        label="Border Color"
        description="Border color of the floating pane"
        bind:value={configState.config.tmux.floax_border_color}
        monospace
      />
      <TextInput
        label="Bind Key"
        description="Key to toggle floating pane (after prefix)"
        bind:value={configState.config.tmux.floax_bind}
        monospace
      />
    </div>
  </div>

  <!-- SessionX -->
  <div class="sub-section">
    <h3 class="sub-heading">SessionX</h3>
    <div class="field-group">
      <TextInput
        label="Bind Key"
        description="Key to open session picker (after prefix)"
        bind:value={configState.config.tmux.sessionx_bind}
        monospace
      />
      <div class="field-row">
        <TextInput
          label="Window Width"
          description="Session picker window width"
          bind:value={configState.config.tmux.sessionx_window_width}
          monospace
          placeholder="75%"
        />
        <TextInput
          label="Window Height"
          description="Session picker window height"
          bind:value={configState.config.tmux.sessionx_window_height}
          monospace
          placeholder="85%"
        />
      </div>
      <Toggle
        label="Zoxide Mode"
        description="Use zoxide for directory-based session switching"
        bind:value={configState.config.tmux.sessionx_zoxide_mode}
      />
    </div>
  </div>

  <!-- Continuum & Resurrect -->
  <div class="sub-section">
    <h3 class="sub-heading">Continuum & Resurrect</h3>
    <div class="field-group">
      <Toggle
        label="Continuum Restore"
        description="Auto-restore last saved environment on tmux start"
        bind:value={configState.config.tmux.continuum_restore}
      />
      <Toggle
        label="Capture Pane Contents"
        description="Save pane text content when persisting sessions"
        bind:value={configState.config.tmux.resurrect_capture_pane_contents}
      />
      <TextInput
        label="Resurrect Neovim Strategy"
        description="Strategy for restoring Neovim sessions (e.g. session)"
        bind:value={configState.config.tmux.resurrect_strategy_nvim}
        monospace
      />
    </div>
  </div>

  <!-- Popup Bindings -->
  <div class="sub-section">
    <h3 class="sub-heading">Popup Bindings</h3>
    <div class="popup-list">
      {#each config.popup_bindings as binding, i}
        <div class="popup-item">
          <div class="popup-fields">
            <label class="popup-field">
              <span class="popup-field-label">Key</span>
              <input
                type="text"
                class="popup-input"
                value={binding.key}
                oninput={(e) => updatePopupField(i, "key", e.currentTarget.value)}
                placeholder="g"
              />
            </label>
            <label class="popup-field popup-field-wide">
              <span class="popup-field-label">Command</span>
              <input
                type="text"
                class="popup-input"
                value={binding.command}
                oninput={(e) => updatePopupField(i, "command", e.currentTarget.value)}
                placeholder="lazygit"
              />
            </label>
            <label class="popup-field">
              <span class="popup-field-label">Width</span>
              <input
                type="text"
                class="popup-input"
                value={binding.width}
                oninput={(e) => updatePopupField(i, "width", e.currentTarget.value)}
                placeholder="80%"
              />
            </label>
            <label class="popup-field">
              <span class="popup-field-label">Height</span>
              <input
                type="text"
                class="popup-input"
                value={binding.height}
                oninput={(e) => updatePopupField(i, "height", e.currentTarget.value)}
                placeholder="80%"
              />
            </label>
          </div>
          <button class="remove-btn" onclick={() => removePopupBinding(i)} title="Remove binding">
            &times;
          </button>
        </div>
      {/each}
    </div>
    <button class="add-btn" onclick={addPopupBinding}>+ Add Popup Binding</button>
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

  .plugin-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .plugin-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 0;
    gap: 12px;
  }

  .plugin-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .plugin-toggle input[type="checkbox"] {
    accent-color: var(--blue);
    width: 14px;
    height: 14px;
  }

  .plugin-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--fg);
  }

  .plugin-repo {
    font-size: 11px;
    font-family: "JetBrains Mono", monospace;
    color: var(--comment);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 300px;
  }

  .empty-hint {
    font-size: 12px;
    color: var(--comment);
    margin: 4px 0;
  }

  .popup-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .popup-item {
    display: flex;
    align-items: flex-end;
    gap: 8px;
    padding: 10px 12px;
    background: color-mix(in srgb, var(--surface) 50%, transparent);
    border-radius: 8px;
    border: 1px solid color-mix(in srgb, var(--comment) 10%, transparent);
  }

  .popup-fields {
    display: flex;
    gap: 8px;
    flex: 1;
    flex-wrap: wrap;
  }

  .popup-field {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 70px;
  }

  .popup-field-wide {
    flex: 1;
    min-width: 150px;
  }

  .popup-field-label {
    font-size: 10px;
    color: var(--comment);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .popup-input {
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 4px;
    padding: 5px 8px;
    color: var(--fg);
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
    transition: border-color 0.15s;
  }

  .popup-input:focus {
    outline: none;
    border-color: var(--blue);
  }

  .remove-btn {
    background: none;
    border: none;
    color: var(--red);
    font-size: 18px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    line-height: 1;
    opacity: 0.6;
    transition: opacity 0.15s;
  }

  .remove-btn:hover {
    opacity: 1;
    background: color-mix(in srgb, var(--red) 10%, transparent);
  }

  .add-btn {
    background: none;
    border: 1px dashed color-mix(in srgb, var(--comment) 30%, transparent);
    border-radius: 6px;
    padding: 8px 16px;
    color: var(--blue);
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s;
    margin-top: 8px;
  }

  .add-btn:hover {
    border-color: var(--blue);
    background: color-mix(in srgb, var(--blue) 5%, transparent);
  }
</style>
