<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";
  import Select from "$lib/components/shared/Select.svelte";

  let config = $derived(configState.config.fastfetch);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.fastfetch));
  $effect(() => {
    const curr = JSON.stringify(configState.config.fastfetch);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("fastfetch");
    }
  });

  // Quotes as editable textarea text
  let quotesText = $derived(config.quotes.join("\n"));
  let quotesInput = $state("");
  $effect(() => {
    quotesInput = config.quotes.join("\n");
  });

  function updateQuotes(text: string) {
    configState.config.fastfetch.quotes = text
      .split("\n")
      .filter((line) => line.trim().length > 0);
  }

  function toggleModule(index: number) {
    configState.config.fastfetch.modules[index].enabled =
      !configState.config.fastfetch.modules[index].enabled;
  }
</script>

<section class="section-panel">
  <SectionHeader icon="📊" number={5} title="Fastfetch" description="System information display on terminal startup" />

  <!-- Logo & Display -->
  <div class="sub-section">
    <h3 class="sub-heading">Logo & Display</h3>
    <div class="field-group">
      <Select
        label="Logo Type"
        description="Size of the ASCII logo displayed"
        bind:value={configState.config.fastfetch.logo_type}
        options={[
          { value: "small", label: "Small" },
          { value: "default", label: "Default" },
          { value: "none", label: "None" },
        ]}
      />
      <div class="field-row">
        <TextInput
          label="Logo Color 1"
          description="ANSI color code for primary logo color"
          bind:value={configState.config.fastfetch.logo_color_1}
          monospace
        />
        <TextInput
          label="Logo Color 2"
          description="ANSI color code for secondary logo color"
          bind:value={configState.config.fastfetch.logo_color_2}
          monospace
        />
      </div>
      <div class="field-row">
        <TextInput
          label="Separator"
          description="Character between label and value columns"
          bind:value={configState.config.fastfetch.separator}
          monospace
        />
        <NumberInput
          label="Key Width"
          description="Width of the label column"
          bind:value={configState.config.fastfetch.key_width}
          min={0}
          max={40}
        />
      </div>
      <Toggle
        label="Show Color Circles"
        description="Display ANSI color palette circles"
        bind:value={configState.config.fastfetch.show_color_circles}
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
          <span class="module-name">{mod.type}</span>
          {#if mod.key}
            <span class="module-key">({mod.key})</span>
          {/if}
        </label>
      {/each}
      {#if config.modules.length === 0}
        <p class="empty-hint">No modules configured. Modules will appear once loaded from the backend.</p>
      {/if}
    </div>
  </div>

  <!-- Quotes -->
  <div class="sub-section">
    <h3 class="sub-heading">Quotes</h3>
    <div class="field-group">
      <span class="field-label">Startup Quotes</span>
      <span class="field-desc">One quote per line. A random quote is displayed on each terminal launch.</span>
      <!-- svelte-ignore a11y_label_has_associated_control -->
      <textarea
        class="quotes-textarea"
        value={quotesInput}
        oninput={(e) => {
          quotesInput = e.currentTarget.value;
          updateQuotes(e.currentTarget.value);
        }}
        rows={6}
        placeholder="Enter quotes, one per line..."
      ></textarea>
      <span class="quote-count">{config.quotes.length} quote{config.quotes.length !== 1 ? "s" : ""}</span>
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

  .field-label {
    font-size: 12px;
    color: var(--fg);
    font-weight: 500;
  }

  .field-desc {
    font-size: 11px;
    color: var(--comment);
  }

  .module-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
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

  .module-key {
    font-size: 11px;
    color: var(--comment);
  }

  .empty-hint {
    font-size: 12px;
    color: var(--comment);
    margin: 4px 0;
    grid-column: 1 / -1;
  }

  .quotes-textarea {
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 6px;
    padding: 10px 12px;
    color: var(--fg);
    font-family: inherit;
    font-size: 13px;
    resize: vertical;
    line-height: 1.5;
    transition: border-color 0.15s;
    min-height: 100px;
  }

  .quotes-textarea:focus {
    outline: none;
    border-color: var(--blue);
  }

  .quote-count {
    font-size: 11px;
    color: var(--comment);
    text-align: right;
  }
</style>
