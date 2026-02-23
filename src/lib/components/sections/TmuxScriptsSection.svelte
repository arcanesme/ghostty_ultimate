<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";

  let config = $derived(configState.config.tmux_scripts);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.tmux_scripts));
  $effect(() => {
    const curr = JSON.stringify(configState.config.tmux_scripts);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
    }
  });
</script>

<section class="section-panel">
  <SectionHeader icon="📜" number={12} title="tmux Scripts" description="Toggle helper scripts installed into tmux" />

  <div class="field-group">
    <Toggle
      label="tmux AI"
      description="Open an AI assistant panel inside a tmux popup for quick questions and code generation"
      bind:value={configState.config.tmux_scripts.tmux_ai_enabled}
    />
    <Toggle
      label="tmux Pair"
      description="Launch a pair-programming session with shared tmux panes for collaboration"
      bind:value={configState.config.tmux_scripts.tmux_pair_enabled}
    />
    <Toggle
      label="tmux Review"
      description="Start a code review workflow with diff views and annotation support in tmux"
      bind:value={configState.config.tmux_scripts.tmux_review_enabled}
    />
    <Toggle
      label="tmux Dev"
      description="Spin up a pre-configured development layout with editor, terminal, and server panes"
      bind:value={configState.config.tmux_scripts.tmux_dev_enabled}
    />
    <Toggle
      label="tmux Cheat"
      description="Display a cheat sheet popup with common tmux keybindings and commands"
      bind:value={configState.config.tmux_scripts.tmux_cheat_enabled}
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
    margin-top: 12px;
  }
</style>
