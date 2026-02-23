<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";

  // Bind directly to configState's $state proxy.
  // Toggle's $bindable write-back mutates the reactive proxy.
  // Mark dirty on any change via $effect.
  let prev = $state(JSON.stringify(configState.config.clean_artifacts));
  $effect(() => {
    const curr = JSON.stringify(configState.config.clean_artifacts);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
    }
  });
</script>

<section class="section-panel">
  <SectionHeader icon="🧹" number={1} title="Clean Artifacts" description="Remove old configs and cached state before setup" />

  <div class="field-group">
    <Toggle
      label="Backup Configs"
      description="Back up existing config files before overwriting"
      bind:value={configState.config.clean_artifacts.backup_configs}
    />
    <Toggle
      label="Clean Neovim State"
      description="Remove Neovim share/state/cache directories"
      bind:value={configState.config.clean_artifacts.clean_nvim_state}
    />
    <Toggle
      label="Remove Catppuccin"
      description="Delete leftover Catppuccin theme files"
      bind:value={configState.config.clean_artifacts.remove_catppuccin}
    />
    <Toggle
      label="Clean .zshrc"
      description="Strip previous customization blocks from .zshrc"
      bind:value={configState.config.clean_artifacts.clean_zshrc}
    />
    <Toggle
      label="Remove Legacy Scripts"
      description="Delete old shell scripts from previous setups"
      bind:value={configState.config.clean_artifacts.remove_legacy_scripts}
    />
  </div>
</section>

<style>
  .section-panel {
    padding: 24px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
</style>
