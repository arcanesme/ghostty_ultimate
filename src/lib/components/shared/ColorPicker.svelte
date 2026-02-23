<script lang="ts">
  let { label, value = $bindable() }: {
    label: string;
    value: string;
  } = $props();
</script>

<label class="color-field">
  <span class="color-label">{label}</span>
  <span class="color-input-group">
    <input type="color" class="color-picker" bind:value />
    <input
      type="text"
      class="color-text"
      bind:value
      maxlength="7"
      pattern="#[0-9a-fA-F]{6}"
    />
  </span>
</label>

<style>
  .color-field {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 0;
    gap: 8px;
  }

  .color-label {
    font-size: 12px;
    color: var(--fg);
    min-width: 70px;
  }

  .color-input-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .color-picker {
    -webkit-appearance: none;
    width: 28px;
    height: 28px;
    border: 2px solid color-mix(in srgb, var(--comment) 30%, transparent);
    border-radius: 6px;
    cursor: pointer;
    padding: 0;
    background: none;
  }
  .color-picker::-webkit-color-swatch-wrapper {
    padding: 0;
  }
  .color-picker::-webkit-color-swatch {
    border: none;
    border-radius: 4px;
  }

  .color-text {
    width: 80px;
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 4px;
    padding: 4px 8px;
    color: var(--fg);
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
  }
  .color-text:focus {
    outline: none;
    border-color: var(--blue);
  }
</style>
