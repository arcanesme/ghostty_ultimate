<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import NumberInput from "$lib/components/shared/NumberInput.svelte";
  import Slider from "$lib/components/shared/Slider.svelte";
  import Select from "$lib/components/shared/Select.svelte";

  let config = $derived(configState.config.ghostty);

  // Dirty tracking + auto-apply
  let prev = $state(JSON.stringify(configState.config.ghostty));
  $effect(() => {
    const curr = JSON.stringify(configState.config.ghostty);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("ghostty");
    }
  });

  // Font features as editable comma-separated string
  let fontFeaturesText = $derived(config.font_features.join(", "));

  function updateFontFeatures(text: string) {
    configState.config.ghostty.font_features = text
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);
  }

  // Local state for font features input
  let fontFeaturesInput = $state("");
  $effect(() => {
    fontFeaturesInput = config.font_features.join(", ");
  });

  // cursor_style_blink can be boolean | null; use local $state for Toggle binding
  let cursorBlink = $state(configState.config.ghostty.cursor_style_blink ?? true);
  $effect(() => {
    cursorBlink = configState.config.ghostty.cursor_style_blink ?? true;
  });
  $effect(() => {
    if (cursorBlink !== (configState.config.ghostty.cursor_style_blink ?? true)) {
      configState.config.ghostty.cursor_style_blink = cursorBlink;
    }
  });
</script>

<section class="section-panel">
  <SectionHeader icon="👻" number={3} title="Ghostty" description="Terminal emulator appearance and behavior" />

  <!-- Font -->
  <div class="sub-section">
    <h3 class="sub-heading">Font</h3>
    <div class="field-group">
      <TextInput
        label="Font Family"
        bind:value={configState.config.ghostty.font_family}
        placeholder="JetBrains Mono"
        monospace
      />
      <TextInput
        label="Font Family (Bold)"
        bind:value={configState.config.ghostty.font_family_bold}
        placeholder="JetBrains Mono ExtraBold"
        monospace
      />
      <div class="field-row">
        <NumberInput
          label="Font Size"
          bind:value={configState.config.ghostty.font_size}
          min={8}
          max={72}
          step={0.5}
        />
        <Toggle
          label="Font Thicken"
          description="Make fonts appear bolder on macOS"
          bind:value={configState.config.ghostty.font_thicken}
        />
      </div>
      <label class="field">
        <span class="field-label">Font Features</span>
        <span class="field-desc">Comma-separated OpenType features (e.g. liga, calt, ss01)</span>
        <input
          type="text"
          class="field-input mono"
          value={fontFeaturesInput}
          oninput={(e) => {
            fontFeaturesInput = e.currentTarget.value;
            updateFontFeatures(e.currentTarget.value);
          }}
          placeholder="liga, calt, dlig, ss01"
        />
      </label>
    </div>
  </div>

  <!-- Window -->
  <div class="sub-section">
    <h3 class="sub-heading">Window</h3>
    <div class="field-group">
      <div class="field-row">
        <NumberInput
          label="Padding X"
          bind:value={configState.config.ghostty.window_padding_x}
          min={0}
          max={200}
        />
        <NumberInput
          label="Padding Y"
          bind:value={configState.config.ghostty.window_padding_y}
          min={0}
          max={200}
        />
      </div>
      <Toggle
        label="Window Padding Balance"
        description="Distribute extra padding evenly on both sides"
        bind:value={configState.config.ghostty.window_padding_balance}
      />
      <div class="field-row">
        <Select
          label="Window Decoration"
          bind:value={configState.config.ghostty.window_decoration}
          options={[
            { value: "auto", label: "Auto" },
            { value: "none", label: "None" },
          ]}
        />
        <Select
          label="Window Colorspace"
          bind:value={configState.config.ghostty.window_colorspace}
          options={[
            { value: "srgb", label: "sRGB" },
            { value: "display-p3", label: "Display P3" },
          ]}
        />
      </div>
    </div>
  </div>

  <!-- macOS -->
  <div class="sub-section">
    <h3 class="sub-heading">macOS</h3>
    <div class="field-group">
      <Select
        label="Titlebar Style"
        bind:value={configState.config.ghostty.macos_titlebar_style}
        options={[
          { value: "transparent", label: "Transparent" },
          { value: "native", label: "Native" },
        ]}
      />
      <Toggle
        label="Option as Alt"
        description="Treat macOS Option key as Alt"
        bind:value={configState.config.ghostty.macos_option_as_alt}
      />
      <Toggle
        label="Window Shadow"
        description="Show shadow around the window"
        bind:value={configState.config.ghostty.macos_window_shadow}
      />
    </div>
  </div>

  <!-- Transparency & Background -->
  <div class="sub-section">
    <h3 class="sub-heading">Transparency & Background</h3>
    <div class="field-group">
      <Slider
        label="Background Opacity"
        description="Transparency of the terminal background"
        bind:value={configState.config.ghostty.background_opacity}
        min={0}
        max={1}
        step={0.01}
      />
      <Slider
        label="Window Opacity"
        description="Opacity of the entire window (0 = invisible, 1 = opaque)"
        bind:value={configState.config.ghostty.window_opacity}
        min={0}
        max={1}
        step={0.01}
      />
      <NumberInput
        label="Background Blur"
        description="Blur radius for transparent backgrounds"
        bind:value={configState.config.ghostty.background_blur}
        min={0}
        max={100}
      />
      <Slider
        label="Unfocused Split Opacity"
        description="Opacity of inactive split panes"
        bind:value={configState.config.ghostty.unfocused_split_opacity}
        min={0}
        max={1}
        step={0.01}
      />
      <Slider
        label="Cursor Opacity"
        description="Cursor transparency"
        bind:value={configState.config.ghostty.cursor_opacity}
        min={0}
        max={1}
        step={0.01}
      />
      <Slider
        label="Minimum Contrast"
        description="Minimum contrast ratio (1 = no adjustment, 21 = max)"
        bind:value={configState.config.ghostty.minimum_contrast}
        min={1}
        max={21}
        step={0.1}
      />
      <Toggle
        label="Bold is Bright"
        description="Render bold text with bright colors"
        bind:value={configState.config.ghostty.bold_is_bright}
      />
    </div>
  </div>

  <!-- Cursor -->
  <div class="sub-section">
    <h3 class="sub-heading">Cursor</h3>
    <div class="field-group">
      <Select
        label="Cursor Style"
        bind:value={configState.config.ghostty.cursor_style}
        options={[
          { value: "bar", label: "Bar" },
          { value: "block", label: "Block" },
          { value: "underline", label: "Underline" },
        ]}
      />
      <Toggle
        label="Cursor Blink"
        description="Enable cursor blinking"
        bind:value={cursorBlink}
      />
      <Toggle
        label="Click to Move Cursor"
        description="Allow clicking to reposition cursor"
        bind:value={configState.config.ghostty.cursor_click_to_move}
      />
    </div>
  </div>

  <!-- Terminal -->
  <div class="sub-section">
    <h3 class="sub-heading">Terminal</h3>
    <div class="field-group">
      <NumberInput
        label="Scrollback Limit"
        description="Maximum number of scrollback lines"
        bind:value={configState.config.ghostty.scrollback_limit}
        min={0}
        max={1000000}
        step={1000}
      />
      <Select
        label="Shell Integration"
        bind:value={configState.config.ghostty.shell_integration}
        options={[
          { value: "zsh", label: "zsh" },
          { value: "bash", label: "bash" },
          { value: "fish", label: "fish" },
          { value: "none", label: "None" },
        ]}
      />
      <div class="field-row">
        <Select
          label="Clipboard Read"
          bind:value={configState.config.ghostty.clipboard_read}
          options={[
            { value: "allow", label: "Allow" },
            { value: "deny", label: "Deny" },
            { value: "ask", label: "Ask" },
          ]}
        />
        <Select
          label="Clipboard Write"
          bind:value={configState.config.ghostty.clipboard_write}
          options={[
            { value: "allow", label: "Allow" },
            { value: "deny", label: "Deny" },
            { value: "ask", label: "Ask" },
          ]}
        />
      </div>
      <Toggle
        label="Hide Mouse While Typing"
        bind:value={configState.config.ghostty.mouse_hide_while_typing}
      />
      <Select
        label="Confirm Close Surface"
        bind:value={configState.config.ghostty.confirm_close_surface}
        options={[
          { value: "true", label: "True" },
          { value: "false", label: "False" },
        ]}
      />
    </div>
  </div>

  <!-- Shaders -->
  <div class="sub-section">
    <h3 class="sub-heading">Shaders</h3>
    <div class="field-group">
      <Toggle
        label="Custom Shader"
        description="Enable custom GLSL shader"
        bind:value={configState.config.ghostty.custom_shader_enabled}
      />
      <TextInput
        label="Shader Path"
        description="Path to custom .glsl shader file"
        bind:value={configState.config.ghostty.custom_shader_path}
        placeholder="~/.config/ghostty/shaders/vignette-bloom.glsl"
        monospace
      />
      <Toggle
        label="Install Community Shaders"
        description="Download popular community shader pack"
        bind:value={configState.config.ghostty.install_community_shaders}
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

  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 6px 0;
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

  .field-input {
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 6px;
    padding: 7px 10px;
    color: var(--fg);
    font-family: inherit;
    font-size: 13px;
    transition: border-color 0.15s;
  }

  .field-input:focus {
    outline: none;
    border-color: var(--blue);
  }

  .field-input.mono {
    font-family: "JetBrains Mono", monospace;
  }
</style>
