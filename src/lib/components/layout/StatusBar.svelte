<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import { executionState } from "$lib/state/execution.svelte";
  import { uiState } from "$lib/state/ui.svelte";
  import { SECTIONS } from "$lib/api/types";

  let sectionLabel = $derived(
    SECTIONS.find((s) => s.id === uiState.activeSection)?.label ?? uiState.activeSection
  );
</script>

<footer class="statusbar">
  <div class="left">
    {#if configState.dirty}
      <span class="dirty-indicator">Modified</span>
    {:else}
      <span class="clean-indicator">Clean</span>
    {/if}
    <span class="separator">|</span>
    <span class="section-name">{sectionLabel}</span>
  </div>
  <div class="center">
    {#if executionState.running}
      <span class="running">
        Running {executionState.progress}/{executionState.totalSections}
      </span>
    {:else if executionState.completed}
      <span class="completed">Complete</span>
    {/if}
  </div>
  <div class="right">
    <span class="shortcut-hint">Cmd+S to apply</span>
    <span class="separator">|</span>
    <span class="palette-name">
      {configState.config.palette.bg}
    </span>
  </div>
</footer>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 16px;
    background: var(--surface);
    border-top: 1px solid color-mix(in srgb, var(--comment) 15%, transparent);
    font-size: 11px;
    flex-shrink: 0;
  }

  .left, .center, .right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dirty-indicator {
    color: var(--yellow);
  }
  .clean-indicator {
    color: var(--comment);
  }
  .running {
    color: var(--blue);
  }
  .completed {
    color: var(--green);
  }
  .section-name {
    color: var(--blue);
    font-weight: 500;
  }
  .separator {
    color: color-mix(in srgb, var(--comment) 40%, transparent);
  }
  .shortcut-hint {
    color: var(--comment);
    opacity: 0.7;
    font-size: 10px;
  }
  .palette-name {
    color: var(--comment);
  }
</style>
