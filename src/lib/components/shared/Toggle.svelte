<script lang="ts">
  let { label, value = $bindable(), description = "" }: {
    label: string;
    value: boolean;
    description?: string;
  } = $props();
</script>

<label class="toggle-row">
  <div class="toggle-info">
    <span class="toggle-label">{label}</span>
    {#if description}
      <span class="toggle-desc">{description}</span>
    {/if}
  </div>
  <button
    class="toggle-switch"
    class:active={value}
    onclick={() => (value = !value)}
    role="switch"
    aria-checked={value}
    aria-label={label}
  >
    <span class="toggle-thumb"></span>
  </button>
</label>

<style>
  .toggle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0;
    cursor: pointer;
    gap: 16px;
  }

  .toggle-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toggle-label {
    font-size: 13px;
    color: var(--fg);
  }

  .toggle-desc {
    font-size: 11px;
    color: var(--comment);
  }

  .toggle-switch {
    width: 36px;
    height: 20px;
    border-radius: 10px;
    border: none;
    background: color-mix(in srgb, var(--comment) 40%, var(--bg));
    position: relative;
    cursor: pointer;
    transition: background 0.15s;
    flex-shrink: 0;
    padding: 0;
  }

  .toggle-switch.active {
    background: var(--blue);
  }

  .toggle-thumb {
    position: absolute;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: white;
    top: 2px;
    left: 2px;
    transition: transform 0.15s;
  }

  .toggle-switch.active .toggle-thumb {
    transform: translateX(16px);
  }
</style>
