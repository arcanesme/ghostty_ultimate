<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";
  import Select from "$lib/components/shared/Select.svelte";
  import ColorPicker from "$lib/components/shared/ColorPicker.svelte";

  let config = $derived(configState.config.neovim);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.neovim));
  $effect(() => {
    const curr = JSON.stringify(configState.config.neovim);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("neovim");
    }
  });

  function togglePlugin(index: number) {
    configState.config.neovim.plugins[index].enabled =
      !configState.config.neovim.plugins[index].enabled;
  }

  const mouseOptions = [
    { value: "a", label: "All (a)" },
    { value: "n", label: "Normal (n)" },
    { value: "v", label: "Visual (v)" },
    { value: "i", label: "Insert (i)" },
    { value: "c", label: "Command (c)" },
    { value: "", label: "Disabled" },
  ];

  const clipboardOptions = [
    { value: "unnamedplus", label: "unnamedplus (system)" },
    { value: "unnamed", label: "unnamed" },
    { value: "", label: "Default" },
  ];

  const signcolumnOptions = [
    { value: "yes", label: "Yes (always)" },
    { value: "no", label: "No (never)" },
    { value: "auto", label: "Auto" },
    { value: "number", label: "Number" },
  ];
</script>

<section class="section-panel">
  <SectionHeader icon="✏️" number={8} title="Neovim" description="Neovim editor configuration and plugins" />

  <!-- Theme -->
  <div class="sub-section">
    <h3 class="sub-heading">Theme</h3>
    <div class="field-group">
      <TextInput
        label="Theme"
        bind:value={configState.config.neovim.theme}
        placeholder="ayu-dark"
        description="Colorscheme name"
      />
      <ColorPicker
        label="BG Override"
        bind:value={configState.config.neovim.theme_bg_override}
      />
    </div>
  </div>

  <!-- Editor -->
  <div class="sub-section">
    <h3 class="sub-heading">Editor</h3>
    <div class="field-group">
      <TextInput
        label="Leader Key"
        bind:value={configState.config.neovim.leader_key}
        placeholder="<Space>"
        description="Map leader key"
        monospace
      />
      <Toggle
        label="Line Numbers"
        description="Show absolute line numbers"
        bind:value={configState.config.neovim.number}
      />
      <Toggle
        label="Relative Numbers"
        description="Show relative line numbers"
        bind:value={configState.config.neovim.relative_number}
      />
      <Toggle
        label="Cursor Line"
        description="Highlight the current line"
        bind:value={configState.config.neovim.cursorline}
      />
      <div class="field-row">
        <NumberInput
          label="Scroll Offset"
          bind:value={configState.config.neovim.scrolloff}
          min={0}
          max={50}
          description="Minimum lines above/below cursor"
        />
        <NumberInput
          label="Shift Width"
          bind:value={configState.config.neovim.shiftwidth}
          min={1}
          max={16}
          description="Spaces per indent level"
        />
      </div>
      <NumberInput
        label="Tab Stop"
        bind:value={configState.config.neovim.tabstop}
        min={1}
        max={16}
        description="Width of a tab character"
      />
      <Toggle
        label="Expand Tab"
        description="Use spaces instead of tabs"
        bind:value={configState.config.neovim.expandtab}
      />
      <Toggle
        label="Smart Indent"
        description="Auto-indent new lines"
        bind:value={configState.config.neovim.smartindent}
      />
    </div>
  </div>

  <!-- General -->
  <div class="sub-section">
    <h3 class="sub-heading">General</h3>
    <div class="field-group">
      <Select
        label="Mouse Mode"
        bind:value={configState.config.neovim.mouse}
        options={mouseOptions}
        description="Mouse support mode"
      />
      <Select
        label="Clipboard"
        bind:value={configState.config.neovim.clipboard}
        options={clipboardOptions}
        description="Clipboard register to use"
      />
      <Toggle
        label="Undo File"
        description="Persist undo history across sessions"
        bind:value={configState.config.neovim.undofile}
      />
      <Toggle
        label="Ignore Case"
        description="Case-insensitive search by default"
        bind:value={configState.config.neovim.ignorecase}
      />
      <Toggle
        label="Smart Case"
        description="Override ignorecase when search has uppercase"
        bind:value={configState.config.neovim.smartcase}
      />
      <Toggle
        label="True Color"
        description="Enable 24-bit RGB color in the terminal"
        bind:value={configState.config.neovim.termguicolors}
      />
      <Select
        label="Sign Column"
        bind:value={configState.config.neovim.signcolumn}
        options={signcolumnOptions}
        description="When to show the sign column"
      />
    </div>
  </div>

  <!-- Plugins -->
  <div class="sub-section">
    <h3 class="sub-heading">Plugins</h3>
    <div class="field-group">
      {#if config.plugins.length === 0}
        <p class="empty-hint">No plugins configured yet.</p>
      {:else}
        {#each config.plugins as plugin, i}
          <Toggle
            label={plugin.name}
            description={plugin.description}
            bind:value={configState.config.neovim.plugins[i].enabled}
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

  .empty-hint {
    font-size: 12px;
    color: var(--comment);
    font-style: italic;
    margin: 8px 0;
  }
</style>
