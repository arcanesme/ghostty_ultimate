<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";
  import Select from "$lib/components/shared/Select.svelte";

  let config = $derived(configState.config.zsh_plugins);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.zsh_plugins));
  $effect(() => {
    const curr = JSON.stringify(configState.config.zsh_plugins);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("zsh_plugins");
    }
  });

  function togglePlugin(index: number) {
    configState.config.zsh_plugins.plugins[index].enabled =
      !configState.config.zsh_plugins.plugins[index].enabled;
  }

  // Yazi ratio as individual inputs
  let ratioA = $derived(config.yazi.ratio[0]);
  let ratioB = $derived(config.yazi.ratio[1]);
  let ratioC = $derived(config.yazi.ratio[2]);

  function updateRatio(index: number, value: number) {
    const r: [number, number, number] = [...configState.config.zsh_plugins.yazi.ratio] as [number, number, number];
    r[index] = value;
    configState.config.zsh_plugins.yazi.ratio = r;
  }
</script>

<section class="section-panel">
  <SectionHeader icon="🧩" number={6} title="Zsh Plugins + Yazi" description="Zsh plugin management and Yazi file manager configuration" />

  <!-- Plugins -->
  <div class="sub-section">
    <h3 class="sub-heading">Zsh Plugins</h3>
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

  <!-- Yazi -->
  <div class="sub-section">
    <h3 class="sub-heading">Yazi File Manager</h3>

    <!-- Layout Ratio -->
    <div class="field-group">
      <span class="field-label">Layout Ratio</span>
      <span class="field-desc">Column ratio for parent : current : preview panes</span>
      <div class="ratio-row">
        <label class="ratio-input">
          <span class="ratio-label">Parent</span>
          <input
            type="number"
            class="field-input-sm"
            value={ratioA}
            min={1}
            max={10}
            oninput={(e) => updateRatio(0, parseInt(e.currentTarget.value) || 1)}
          />
        </label>
        <span class="ratio-sep">:</span>
        <label class="ratio-input">
          <span class="ratio-label">Current</span>
          <input
            type="number"
            class="field-input-sm"
            value={ratioB}
            min={1}
            max={10}
            oninput={(e) => updateRatio(1, parseInt(e.currentTarget.value) || 1)}
          />
        </label>
        <span class="ratio-sep">:</span>
        <label class="ratio-input">
          <span class="ratio-label">Preview</span>
          <input
            type="number"
            class="field-input-sm"
            value={ratioC}
            min={1}
            max={10}
            oninput={(e) => updateRatio(2, parseInt(e.currentTarget.value) || 1)}
          />
        </label>
      </div>
    </div>

    <!-- Sort Options -->
    <div class="field-group">
      <h4 class="sub-sub-heading">Sorting</h4>
      <Select
        label="Sort By"
        bind:value={configState.config.zsh_plugins.yazi.sort_by}
        options={[
          { value: "alphabetical", label: "Alphabetical" },
          { value: "natural", label: "Natural" },
          { value: "created", label: "Created" },
          { value: "modified", label: "Modified" },
          { value: "size", label: "Size" },
          { value: "extension", label: "Extension" },
        ]}
      />
      <Toggle
        label="Case Sensitive"
        bind:value={configState.config.zsh_plugins.yazi.sort_sensitive}
      />
      <Toggle
        label="Reverse Order"
        bind:value={configState.config.zsh_plugins.yazi.sort_reverse}
      />
      <Toggle
        label="Directories First"
        bind:value={configState.config.zsh_plugins.yazi.sort_dir_first}
      />
    </div>

    <!-- Display -->
    <div class="field-group">
      <h4 class="sub-sub-heading">Display</h4>
      <Toggle
        label="Show Hidden Files"
        bind:value={configState.config.zsh_plugins.yazi.show_hidden}
      />
      <Toggle
        label="Show Symlinks"
        bind:value={configState.config.zsh_plugins.yazi.show_symlink}
      />
      <NumberInput
        label="Scroll Offset"
        description="Lines to keep visible above/below cursor"
        bind:value={configState.config.zsh_plugins.yazi.scrolloff}
        min={0}
        max={20}
      />
    </div>

    <!-- Preview -->
    <div class="field-group">
      <h4 class="sub-sub-heading">Preview</h4>
      <NumberInput
        label="Tab Size"
        bind:value={configState.config.zsh_plugins.yazi.preview_tab_size}
        min={1}
        max={8}
      />
      <div class="field-row">
        <NumberInput
          label="Max Width"
          bind:value={configState.config.zsh_plugins.yazi.preview_max_width}
          min={100}
          max={3840}
          step={100}
        />
        <NumberInput
          label="Max Height"
          bind:value={configState.config.zsh_plugins.yazi.preview_max_height}
          min={100}
          max={2160}
          step={100}
        />
      </div>
      <Select
        label="Image Filter"
        bind:value={configState.config.zsh_plugins.yazi.preview_image_filter}
        options={[
          { value: "lanczos3", label: "Lanczos3" },
          { value: "nearest", label: "Nearest" },
          { value: "triangle", label: "Triangle" },
          { value: "catmullrom", label: "Catmull-Rom" },
          { value: "gaussian", label: "Gaussian" },
        ]}
      />
      <NumberInput
        label="Image Quality"
        description="JPEG quality for image previews (1-100)"
        bind:value={configState.config.zsh_plugins.yazi.preview_image_quality}
        min={1}
        max={100}
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

  .sub-sub-heading {
    font-size: 12px;
    font-weight: 600;
    color: var(--purple);
    margin: 12px 0 4px 0;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 12px;
  }

  .field-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    align-items: start;
  }

  .field-label {
    font-size: 12px;
    color: var(--fg);
    font-weight: 500;
  }

  .field-desc {
    font-size: 11px;
    color: var(--comment);
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

  .ratio-row {
    display: flex;
    align-items: flex-end;
    gap: 8px;
  }

  .ratio-input {
    display: flex;
    flex-direction: column;
    gap: 2px;
    align-items: center;
  }

  .ratio-label {
    font-size: 10px;
    color: var(--comment);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .ratio-sep {
    font-size: 16px;
    color: var(--comment);
    padding-bottom: 6px;
  }

  .field-input-sm {
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 6px;
    padding: 6px 8px;
    color: var(--fg);
    font-family: "JetBrains Mono", monospace;
    font-size: 13px;
    width: 60px;
    text-align: center;
    transition: border-color 0.15s;
  }

  .field-input-sm:focus {
    outline: none;
    border-color: var(--blue);
  }
</style>
