<script lang="ts">
  import { uiState } from "$lib/state/ui.svelte";
</script>

<div class="script-preview">
  <div class="preview-header">
    <h2>Generated Script</h2>
    <div class="preview-actions">
      <button
        class="btn"
        onclick={() => navigator.clipboard.writeText(uiState.previewContent)}
      >
        Copy
      </button>
      <button class="btn" onclick={() => (uiState.showPreview = false)}>
        Close
      </button>
    </div>
  </div>
  <pre class="preview-code"><code>{uiState.previewContent}</code></pre>
</div>

<style>
  .script-preview {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
    flex-shrink: 0;
  }

  .preview-header h2 {
    margin: 0;
    font-size: 16px;
    color: var(--purple);
  }

  .preview-actions {
    display: flex;
    gap: 8px;
  }

  .btn {
    padding: 6px 14px;
    background: var(--surface);
    border: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
    border-radius: 6px;
    color: var(--fg);
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
  }
  .btn:hover {
    border-color: var(--blue);
  }

  .preview-code {
    flex: 1;
    margin: 0;
    padding: 16px;
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 15%, transparent);
    border-radius: 8px;
    overflow: auto;
    font-size: 12px;
    line-height: 1.6;
    color: var(--fg);
    white-space: pre;
    tab-size: 2;
  }
</style>
