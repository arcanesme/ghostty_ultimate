<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import Select from "$lib/components/shared/Select.svelte";

  let config = $derived(configState.config.tool_configs);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.tool_configs));
  $effect(() => {
    const curr = JSON.stringify(configState.config.tool_configs);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("tool_configs");
    }
  });

  const atuinStyleOptions = [
    { value: "full", label: "Full" },
    { value: "compact", label: "Compact" },
    { value: "auto", label: "Auto" },
  ];

  const atuinSearchModeOptions = [
    { value: "fuzzy", label: "Fuzzy" },
    { value: "prefix", label: "Prefix" },
    { value: "fulltext", label: "Full Text" },
    { value: "skim", label: "Skim" },
  ];

  const atuinFilterModeOptions = [
    { value: "global", label: "Global" },
    { value: "host", label: "Host" },
    { value: "session", label: "Session" },
    { value: "directory", label: "Directory" },
  ];

  const atuinKeymapOptions = [
    { value: "emacs", label: "Emacs" },
    { value: "vim-normal", label: "Vim Normal" },
    { value: "vim-insert", label: "Vim Insert" },
    { value: "auto", label: "Auto" },
  ];

  const lazygitNerdFontOptions = [
    { value: "3", label: "v3" },
    { value: "2", label: "v2" },
    { value: "", label: "None" },
  ];

  const lazygitBorderOptions = [
    { value: "single", label: "Single" },
    { value: "double", label: "Double" },
    { value: "rounded", label: "Rounded" },
    { value: "hidden", label: "Hidden" },
    { value: "none", label: "None" },
  ];

  const lazygitEditPresetOptions = [
    { value: "nvim", label: "Neovim" },
    { value: "vim", label: "Vim" },
    { value: "nano", label: "Nano" },
    { value: "vscode", label: "VS Code" },
  ];
</script>

<section class="section-panel">
  <SectionHeader icon="⚙️" number={9} title="Tool Configs" description="Configuration for CLI tools" />

  <!-- Atuin -->
  <div class="sub-section">
    <h3 class="sub-heading">Atuin</h3>
    <div class="field-group">
      <Select
        label="Style"
        bind:value={configState.config.tool_configs.atuin.style}
        options={atuinStyleOptions}
        description="UI display style"
      />
      <Select
        label="Search Mode"
        bind:value={configState.config.tool_configs.atuin.search_mode}
        options={atuinSearchModeOptions}
        description="How search queries are matched"
      />
      <Select
        label="Filter Mode"
        bind:value={configState.config.tool_configs.atuin.filter_mode}
        options={atuinFilterModeOptions}
        description="Scope of history shown"
      />
      <Toggle
        label="Show Preview"
        description="Display command preview pane"
        bind:value={configState.config.tool_configs.atuin.show_preview}
      />
      <Toggle
        label="Show Tabs"
        description="Display tab bar in UI"
        bind:value={configState.config.tool_configs.atuin.show_tabs}
      />
      <Toggle
        label="Timestamps"
        description="Show timestamps on history entries"
        bind:value={configState.config.tool_configs.atuin.timestamps_enabled}
      />
      <TextInput
        label="Time Format"
        bind:value={configState.config.tool_configs.atuin.time_format}
        placeholder="%I:%M %p"
        description="strftime format for timestamps"
        monospace
      />
      <Toggle
        label="Sync Records"
        description="Synchronize history records"
        bind:value={configState.config.tool_configs.atuin.sync_records}
      />
      <Select
        label="Keymap Mode"
        bind:value={configState.config.tool_configs.atuin.keymap_mode}
        options={atuinKeymapOptions}
        description="Key binding style"
      />
    </div>
  </div>

  <!-- Lazygit -->
  <div class="sub-section">
    <h3 class="sub-heading">Lazygit</h3>
    <div class="field-group">
      <Select
        label="Nerd Fonts Version"
        bind:value={configState.config.tool_configs.lazygit.nerd_fonts_version}
        options={lazygitNerdFontOptions}
        description="Nerd Fonts version for icon rendering"
      />
      <Toggle
        label="Show File Icons"
        description="Display file type icons"
        bind:value={configState.config.tool_configs.lazygit.show_file_icons}
      />
      <Select
        label="Border"
        bind:value={configState.config.tool_configs.lazygit.border}
        options={lazygitBorderOptions}
        description="Border style for panels"
      />
      <Toggle
        label="Mouse Events"
        description="Enable mouse interaction"
        bind:value={configState.config.tool_configs.lazygit.mouse_events}
      />
      <Toggle
        label="Show Command Log"
        description="Display the git command log panel"
        bind:value={configState.config.tool_configs.lazygit.show_command_log}
      />
      <TextInput
        label="Pager"
        bind:value={configState.config.tool_configs.lazygit.pager}
        placeholder="delta --dark --paging=never"
        description="External pager command for diffs"
        monospace
      />
      <Select
        label="Edit Preset"
        bind:value={configState.config.tool_configs.lazygit.edit_preset}
        options={lazygitEditPresetOptions}
        description="Editor launched for editing"
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
</style>
