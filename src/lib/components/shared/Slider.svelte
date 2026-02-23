<script lang="ts">
  let { label, value = $bindable(), min = 0, max = 1, step = 0.01, description = "", suffix = "" }: {
    label: string;
    value: number;
    min?: number;
    max?: number;
    step?: number;
    description?: string;
    suffix?: string;
  } = $props();
</script>

<label class="field">
  <div class="field-header">
    <span class="field-label">{label}</span>
    <span class="field-value">{value}{suffix}</span>
  </div>
  {#if description}
    <span class="field-desc">{description}</span>
  {/if}
  <input
    type="range"
    class="slider"
    bind:value
    {min}
    {max}
    {step}
  />
</label>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 6px 0;
  }

  .field-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .field-label {
    font-size: 12px;
    color: var(--fg);
    font-weight: 500;
  }

  .field-value {
    font-size: 12px;
    color: var(--blue);
    font-family: "JetBrains Mono", monospace;
  }

  .field-desc {
    font-size: 11px;
    color: var(--comment);
  }

  .slider {
    -webkit-appearance: none;
    width: 100%;
    height: 4px;
    border-radius: 2px;
    background: color-mix(in srgb, var(--comment) 30%, var(--bg));
    outline: none;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: var(--blue);
    cursor: pointer;
  }
</style>
