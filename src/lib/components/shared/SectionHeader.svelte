<script lang="ts">
  let { icon, number, title, description = "" }: {
    icon: string;
    number: number;
    title: string;
    description?: string;
  } = $props();
</script>

<div class="section-header">
  <div class="section-title">
    <span class="section-icon">{icon}</span>
    <span class="section-num">{number}/13</span>
    <h2>{title}</h2>
  </div>
  {#if description}
    <p class="section-desc">{description}</p>
  {/if}
</div>

<style>
  .section-header {
    margin-bottom: 20px;
    padding-bottom: 12px;
    border-bottom: 1px solid color-mix(in srgb, var(--comment) 15%, transparent);
  }

  .section-title {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .section-icon {
    font-size: 20px;
  }

  .section-num {
    font-size: 12px;
    color: var(--comment);
    background: var(--surface);
    padding: 2px 8px;
    border-radius: 4px;
  }

  h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: var(--fg);
  }

  .section-desc {
    margin: 6px 0 0 0;
    font-size: 12px;
    color: var(--comment);
  }
</style>
