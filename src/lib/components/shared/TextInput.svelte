<script lang="ts">
  let { label, value = $bindable(), placeholder = "", description = "", monospace = false }: {
    label: string;
    value: string;
    placeholder?: string;
    description?: string;
    monospace?: boolean;
  } = $props();
</script>

<label class="field">
  <span class="field-label">{label}</span>
  {#if description}
    <span class="field-desc">{description}</span>
  {/if}
  <input
    type="text"
    class="field-input"
    class:mono={monospace}
    bind:value
    {placeholder}
  />
</label>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 6px 0;
  }

  .field-label {
    font-size: 12px;
    color: var(--fg);
    font-weight: 500;
  }

  .field-desc {
    font-size: 11px;
    color: var(--comment);
  }

  .field-input {
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 25%, transparent);
    border-radius: 6px;
    padding: 7px 10px;
    color: var(--fg);
    font-family: inherit;
    font-size: 13px;
    transition: border-color 0.15s;
  }
  .field-input:focus {
    outline: none;
    border-color: var(--blue);
  }
  .field-input.mono {
    font-family: "JetBrains Mono", monospace;
  }
</style>
