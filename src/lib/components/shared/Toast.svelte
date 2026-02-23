<script lang="ts">
  import { uiState } from "$lib/state/ui.svelte";
</script>

{#if uiState.showToast}
  <div class="toast" class:success={uiState.toastType === "success"} class:error={uiState.toastType === "error"}>
    <span class="toast-icon">{uiState.toastType === "success" ? "✓" : "✗"}</span>
    <span class="toast-message">{uiState.toastMessage}</span>
    <button class="toast-close" onclick={() => (uiState.showToast = false)}>×</button>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    bottom: 48px;
    right: 20px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    border-radius: 8px;
    font-size: 12px;
    font-family: inherit;
    z-index: 1000;
    animation: slideIn 0.2s ease-out;
    max-width: 400px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .toast.success {
    background: color-mix(in srgb, var(--green) 15%, var(--surface));
    border: 1px solid color-mix(in srgb, var(--green) 40%, transparent);
    color: var(--green);
  }

  .toast.error {
    background: color-mix(in srgb, var(--red) 15%, var(--surface));
    border: 1px solid color-mix(in srgb, var(--red) 40%, transparent);
    color: var(--red);
  }

  .toast-icon {
    font-weight: 700;
    font-size: 14px;
    flex-shrink: 0;
  }

  .toast-message {
    flex: 1;
    color: var(--fg);
  }

  .toast-close {
    background: none;
    border: none;
    color: var(--comment);
    cursor: pointer;
    font-size: 16px;
    padding: 0 2px;
    line-height: 1;
    flex-shrink: 0;
  }

  .toast-close:hover {
    color: var(--fg);
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
