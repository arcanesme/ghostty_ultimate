<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";

  let config = $derived(configState.config.packages);

  // Group packages by category
  let grouped = $derived.by(() => {
    const groups: Record<string, typeof config.packages> = {};
    for (const pkg of config.packages) {
      if (!groups[pkg.category]) groups[pkg.category] = [];
      groups[pkg.category].push(pkg);
    }
    return groups;
  });

  const categoryLabels: Record<string, string> = {
    core: "Core",
    file: "File Management",
    monitor: "Monitoring",
    dev: "Development",
    util: "Utilities",
  };

  const categoryOrder = ["core", "file", "monitor", "dev", "util"];

  let orderedCategories = $derived(
    categoryOrder.filter((c) => grouped[c]?.length)
  );

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.packages));
  $effect(() => {
    const curr = JSON.stringify(configState.config.packages);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
    }
  });

  function togglePackage(index: number) {
    configState.config.packages.packages[index].enabled =
      !configState.config.packages.packages[index].enabled;
  }
</script>

<section class="section-panel">
  <SectionHeader icon="📦" number={2} title="Packages" description="Homebrew packages and font installation" />

  <div class="field-group">
    <h3 class="sub-heading">Fonts</h3>
    <Toggle
      label="Install JetBrains Mono"
      description="Install JetBrains Mono font via Homebrew cask"
      bind:value={configState.config.packages.install_jetbrains_font}
    />
    <Toggle
      label="Install Nerd Font"
      description="Install Symbols Nerd Font for icon support"
      bind:value={configState.config.packages.install_nerd_font}
    />
  </div>

  {#each orderedCategories as category}
    <div class="field-group">
      <h3 class="sub-heading">{categoryLabels[category] ?? category}</h3>
      <div class="package-list">
        {#each config.packages as pkg, i}
          {#if pkg.category === category}
            <label class="package-item">
              <input
                type="checkbox"
                checked={pkg.enabled}
                onchange={() => togglePackage(i)}
              />
              <span class="package-name">{pkg.name}</span>
            </label>
          {/if}
        {/each}
      </div>
    </div>
  {/each}

  {#if config.packages.length === 0}
    <div class="empty-state">
      <p>No packages configured. Packages will appear here once loaded from the backend.</p>
    </div>
  {/if}
</section>

<style>
  .section-panel {
    padding: 24px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 16px;
  }

  .sub-heading {
    font-size: 13px;
    font-weight: 600;
    color: var(--blue);
    margin: 12px 0 4px 0;
    padding-bottom: 4px;
    border-bottom: 1px solid color-mix(in srgb, var(--comment) 10%, transparent);
  }

  .package-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 4px 16px;
  }

  .package-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 0;
    cursor: pointer;
    font-size: 13px;
    color: var(--fg);
  }

  .package-item input[type="checkbox"] {
    accent-color: var(--blue);
    width: 14px;
    height: 14px;
  }

  .package-name {
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
  }

  .empty-state {
    padding: 16px;
    text-align: center;
    color: var(--comment);
    font-size: 12px;
  }

  .empty-state p {
    margin: 0;
  }
</style>
