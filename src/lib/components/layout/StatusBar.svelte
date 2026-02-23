<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import { executionState } from "$lib/state/execution.svelte";
</script>

<footer class="statusbar">
  <div class="left">
    {#if configState.dirty}
      <span class="dirty-indicator">Modified</span>
    {:else}
      <span class="clean-indicator">Clean</span>
    {/if}
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
  .palette-name {
    color: var(--comment);
  }
</style>
