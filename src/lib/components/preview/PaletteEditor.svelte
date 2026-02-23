<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import { uiState } from "$lib/state/ui.svelte";
  import { getBuiltinPalettes } from "$lib/api/invoke";
  import ColorPicker from "$lib/components/shared/ColorPicker.svelte";
  import type { NamedPalette, AyuPalette } from "$lib/api/types";

  let palettes = $state<NamedPalette[]>([]);
  let selectedPreset = $state("Ayu Dark");

  const baseColors: { key: keyof AyuPalette; label: string }[] = [
    { key: "bg", label: "Background" },
    { key: "fg", label: "Foreground" },
    { key: "surface", label: "Surface" },
    { key: "selection", label: "Selection" },
    { key: "comment", label: "Comment" },
    { key: "gutter", label: "Gutter" },
  ];

  const accentColors: { key: keyof AyuPalette; label: string }[] = [
    { key: "red", label: "Red" },
    { key: "green", label: "Green" },
    { key: "yellow", label: "Yellow" },
    { key: "blue", label: "Blue" },
    { key: "purple", label: "Purple" },
    { key: "cyan", label: "Cyan" },
    { key: "orange", label: "Orange" },
  ];

  const brightColors: { key: keyof AyuPalette; label: string }[] = [
    { key: "br_red", label: "Bright Red" },
    { key: "br_green", label: "Bright Green" },
    { key: "br_blue", label: "Bright Blue" },
    { key: "br_purple", label: "Bright Purple" },
    { key: "br_cyan", label: "Bright Cyan" },
  ];

  $effect(() => {
    getBuiltinPalettes()
      .then((p) => (palettes = p))
      .catch(() => {
        // Fallback presets for dev mode
        palettes = [{ name: "Ayu Dark", palette: configState.config.palette }];
      });
  });

  function applyPreset(name: string) {
    const preset = palettes.find((p) => p.name === name);
    if (preset) {
      configState.updatePalette({ ...preset.palette });
      selectedPreset = name;
    }
  }

  function updateColor(key: keyof AyuPalette, value: string) {
    const p = { ...configState.config.palette };
    p[key] = value;
    configState.updatePalette(p);
  }
</script>

<div class="palette-editor">
  <div class="palette-header">
    <h2>Palette Editor</h2>
    <button class="close-btn" onclick={() => (uiState.showPaletteEditor = false)}>
      Close
    </button>
  </div>

  <label class="preset-bar">
    <span class="preset-label">Preset</span>
    <select
      class="preset-select"
      value={selectedPreset}
      onchange={(e) => applyPreset((e.target as HTMLSelectElement).value)}
    >
      {#each palettes as p}
        <option value={p.name}>{p.name}</option>
      {/each}
    </select>
  </label>

  <div class="color-groups">
    <div class="color-group">
      <h3>Base</h3>
      {#each baseColors as c}
        <ColorPicker
          label={c.label}
          value={configState.config.palette[c.key]}
          onchange={(e: Event) => updateColor(c.key, (e.target as HTMLInputElement).value)}
        />
      {/each}
    </div>

    <div class="color-group">
      <h3>Accents</h3>
      {#each accentColors as c}
        <ColorPicker
          label={c.label}
          value={configState.config.palette[c.key]}
          onchange={(e: Event) => updateColor(c.key, (e.target as HTMLInputElement).value)}
        />
      {/each}
    </div>

    <div class="color-group">
      <h3>Bright</h3>
      {#each brightColors as c}
        <ColorPicker
          label={c.label}
          value={configState.config.palette[c.key]}
          onchange={(e: Event) => updateColor(c.key, (e.target as HTMLInputElement).value)}
        />
      {/each}
    </div>
  </div>

  <div class="preview-section">
    <h3>Terminal Preview</h3>
    <div
      class="terminal-preview"
      style="background:{configState.config.palette.bg};color:{configState.config.palette.fg}"
    >
      <div class="preview-titlebar" style="background:{configState.config.palette.surface}">
        <span class="preview-dot" style="background:#ff5f57"></span>
        <span class="preview-dot" style="background:#febc2e"></span>
        <span class="preview-dot" style="background:#28c840"></span>
        <span class="preview-title" style="color:{configState.config.palette.comment}">ghostty</span>
      </div>
      <div class="preview-content">
        <div>
          <span style="color:{configState.config.palette.blue}">~/projects</span>
          <span style="color:{configState.config.palette.comment}"> main</span>
          <span style="color:{configState.config.palette.blue}">❯</span>
          <span style="color:{configState.config.palette.fg}"> git status</span>
        </div>
        <div>
          <span style="color:{configState.config.palette.green}">  modified:</span>
          <span style="color:{configState.config.palette.fg}"> src/main.rs</span>
        </div>
        <div>
          <span style="color:{configState.config.palette.red}">  deleted:</span>
          <span style="color:{configState.config.palette.fg}">  old_file.txt</span>
        </div>
        <div>
          <span style="color:{configState.config.palette.yellow}">  new file:</span>
          <span style="color:{configState.config.palette.fg}"> config.toml</span>
        </div>
        <div>&nbsp;</div>
        <div>
          <span style="color:{configState.config.palette.blue}">~/projects</span>
          <span style="color:{configState.config.palette.comment}"> main</span>
          <span style="color:{configState.config.palette.blue}">❯</span>
          <span style="color:{configState.config.palette.purple}"> cargo</span>
          <span style="color:{configState.config.palette.fg}"> build</span>
        </div>
        <div>
          <span style="color:{configState.config.palette.green}">   Compiling</span>
          <span style="color:{configState.config.palette.fg}"> ghostty-ui v0.1.0</span>
        </div>
        <div>
          <span style="color:{configState.config.palette.cyan}">    Finished</span>
          <span style="color:{configState.config.palette.comment}"> dev [unoptimized] in 2.34s</span>
        </div>
        <div>&nbsp;</div>
        <div>
          <span style="color:{configState.config.palette.blue}">~/projects</span>
          <span style="color:{configState.config.palette.comment}"> main</span>
          <span style="color:{configState.config.palette.blue}">❯</span>
          <span
            class="cursor-blink"
            style="background:{configState.config.palette.fg};color:{configState.config.palette.bg}"
          >&nbsp;</span>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .palette-editor {
    max-width: 900px;
  }

  .palette-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
  }

  .palette-header h2 {
    margin: 0;
    font-size: 18px;
    color: var(--purple);
  }

  .close-btn {
    background: var(--surface);
    border: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
    border-radius: 6px;
    padding: 6px 14px;
    color: var(--fg);
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
  }

  .preset-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 20px;
    padding: 12px;
    background: var(--surface);
    border-radius: 8px;
  }

  .preset-label {
    font-size: 12px;
    color: var(--comment);
  }

  .preset-select {
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 6px;
    padding: 6px 12px;
    color: var(--fg);
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
  }

  .color-groups {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
    margin-bottom: 24px;
  }

  .color-group h3 {
    margin: 0 0 10px 0;
    font-size: 13px;
    color: var(--comment);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .preview-section h3 {
    margin: 0 0 10px 0;
    font-size: 13px;
    color: var(--comment);
  }

  .terminal-preview {
    border-radius: 10px;
    overflow: hidden;
    font-family: "JetBrains Mono", monospace;
    font-size: 13px;
    line-height: 1.6;
    border: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
  }

  .preview-titlebar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 14px;
  }

  .preview-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }

  .preview-title {
    margin-left: auto;
    font-size: 12px;
  }

  .preview-content {
    padding: 16px 20px;
  }

  .cursor-blink {
    display: inline-block;
    width: 8px;
    animation: blink 1s step-end infinite;
  }

  @keyframes blink {
    50% { opacity: 0; }
  }
</style>
