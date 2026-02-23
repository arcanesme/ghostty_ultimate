<script lang="ts">
  let { show = $bindable() }: {
    show: boolean;
  } = $props();

  function close() {
    show = false;
  }

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) close();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (show && e.key === "Escape") close();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="backdrop" onclick={handleBackdrop} onkeydown={handleKeydown} role="presentation">
    <div class="modal" role="dialog" aria-modal="true" aria-label="About Ghostty Ultimate">
      <div class="modal-header">
        <span class="modal-logo">👻</span>
        <h2>Ghostty Ultimate</h2>
        <span class="version">v0.1.0</span>
      </div>
      <p class="tagline">Ayu Dark Terminal Architecture</p>
      <div class="details">
        <div class="detail-row">
          <span class="detail-label">Built with</span>
          <span class="detail-value">Tauri 2, Svelte 5, Tera</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Source</span>
          <a
            class="detail-link"
            href="https://github.com/arcanesme/ghostty_ultimate"
            target="_blank"
            rel="noopener noreferrer"
          >
            github.com/arcanesme/ghostty_ultimate
          </a>
        </div>
      </div>
      <button class="close-btn" onclick={close}>Close</button>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 500;
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
  }

  .modal {
    background: var(--surface);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 12px;
    padding: 32px;
    min-width: 340px;
    max-width: 420px;
    text-align: center;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    margin-bottom: 4px;
  }

  .modal-logo {
    font-size: 28px;
  }

  h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 700;
    color: var(--purple);
  }

  .version {
    font-size: 11px;
    color: var(--comment);
    background: color-mix(in srgb, var(--comment) 15%, transparent);
    padding: 2px 8px;
    border-radius: 10px;
  }

  .tagline {
    font-size: 12px;
    color: var(--comment);
    margin: 8px 0 20px 0;
  }

  .details {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 24px;
    text-align: left;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 12px;
    background: color-mix(in srgb, var(--bg) 60%, var(--surface));
    border-radius: 6px;
  }

  .detail-label {
    font-size: 11px;
    color: var(--comment);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .detail-value {
    font-size: 12px;
    color: var(--fg);
  }

  .detail-link {
    font-size: 12px;
    color: var(--blue);
    text-decoration: none;
  }

  .detail-link:hover {
    text-decoration: underline;
  }

  .close-btn {
    background: var(--blue);
    color: var(--bg);
    border: none;
    border-radius: 6px;
    padding: 8px 24px;
    font-size: 12px;
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    transition: filter 0.15s;
  }

  .close-btn:hover {
    filter: brightness(1.1);
  }
</style>
