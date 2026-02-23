<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";
  import Select from "$lib/components/shared/Select.svelte";

  let config = $derived(configState.config.themes);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.themes));
  $effect(() => {
    const curr = JSON.stringify(configState.config.themes);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("themes");
    }
  });

  const fzfLayoutOptions = [
    { value: "reverse", label: "Reverse" },
    { value: "default", label: "Default" },
  ];
</script>

<section class="section-panel">
  <SectionHeader icon="🎨" number={11} title="Themes" description="Theme settings for bat, yazi, btop, and fzf" />

  <!-- Global -->
  <div class="sub-section">
    <h3 class="sub-heading">Global</h3>
    <div class="field-group">
      <Toggle
        label="bat Theme"
        description="Apply Ayu theme to bat syntax highlighter"
        bind:value={configState.config.themes.bat_theme_enabled}
      />
      <Toggle
        label="yazi Theme"
        description="Apply Ayu theme to yazi file manager"
        bind:value={configState.config.themes.yazi_theme_enabled}
      />
    </div>
  </div>

  <!-- btop -->
  <div class="sub-section">
    <h3 class="sub-heading">btop</h3>
    <div class="field-group">
      <TextInput
        label="Color Theme"
        bind:value={configState.config.themes.btop.color_theme}
        placeholder="ayu"
        description="btop color theme name"
      />
      <Toggle
        label="Theme Background"
        description="Use theme-defined background color"
        bind:value={configState.config.themes.btop.theme_background}
      />
      <Toggle
        label="True Color"
        description="Enable 24-bit color output"
        bind:value={configState.config.themes.btop.truecolor}
      />
      <TextInput
        label="Shown Boxes"
        bind:value={configState.config.themes.btop.shown_boxes}
        placeholder="cpu mem net proc"
        description="Space-separated list of visible panels"
        monospace
      />
      <NumberInput
        label="Update Interval (ms)"
        bind:value={configState.config.themes.btop.update_ms}
        min={100}
        max={10000}
        step={100}
        description="Refresh rate in milliseconds"
      />
      <TextInput
        label="Process Sorting"
        bind:value={configState.config.themes.btop.proc_sorting}
        placeholder="cpu lazy"
        description="Default process sort order"
        monospace
      />
    </div>
  </div>

  <!-- fzf -->
  <div class="sub-section">
    <h3 class="sub-heading">fzf</h3>
    <div class="field-group">
      <TextInput
        label="Border"
        bind:value={configState.config.themes.fzf.border}
        placeholder="rounded"
        description="Border style for fzf window"
      />
      <div class="field-row">
        <TextInput
          label="Prompt"
          bind:value={configState.config.themes.fzf.prompt}
          placeholder="❯ "
          description="Prompt string"
          monospace
        />
        <TextInput
          label="Marker"
          bind:value={configState.config.themes.fzf.marker}
          placeholder="✓"
          description="Multi-select marker character"
          monospace
        />
      </div>
      <div class="field-row">
        <TextInput
          label="Pointer"
          bind:value={configState.config.themes.fzf.pointer}
          placeholder="▶"
          description="Pointer character for current item"
          monospace
        />
        <TextInput
          label="Separator"
          bind:value={configState.config.themes.fzf.separator}
          placeholder="─"
          description="Separator line character"
          monospace
        />
      </div>
      <TextInput
        label="Scrollbar"
        bind:value={configState.config.themes.fzf.scrollbar}
        placeholder="│"
        description="Scrollbar character"
        monospace
      />
      <Select
        label="Layout"
        bind:value={configState.config.themes.fzf.layout}
        options={fzfLayoutOptions}
        description="Result list direction"
      />
      <TextInput
        label="Height"
        bind:value={configState.config.themes.fzf.height}
        placeholder="60%"
        description="Height of the fzf window"
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

  .field-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    align-items: start;
  }
</style>
