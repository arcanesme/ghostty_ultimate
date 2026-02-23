<script lang="ts">
  import { uiState } from "$lib/state/ui.svelte";
  import PaletteEditor from "$lib/components/preview/PaletteEditor.svelte";
  import ExecutionPanel from "$lib/components/execution/ExecutionPanel.svelte";
  import ScriptPreview from "$lib/components/preview/ScriptPreview.svelte";
  import CleanArtifactsSection from "$lib/components/sections/CleanArtifactsSection.svelte";
  import PackagesSection from "$lib/components/sections/PackagesSection.svelte";
  import GhosttySection from "$lib/components/sections/GhosttySection.svelte";
  import StarshipSection from "$lib/components/sections/StarshipSection.svelte";
  import FastfetchSection from "$lib/components/sections/FastfetchSection.svelte";
  import ZshPluginsSection from "$lib/components/sections/ZshPluginsSection.svelte";
  import TmuxSection from "$lib/components/sections/TmuxSection.svelte";
  import NeovimSection from "$lib/components/sections/NeovimSection.svelte";
  import ToolConfigsSection from "$lib/components/sections/ToolConfigsSection.svelte";
  import GitSection from "$lib/components/sections/GitSection.svelte";
  import ThemesSection from "$lib/components/sections/ThemesSection.svelte";
  import TmuxScriptsSection from "$lib/components/sections/TmuxScriptsSection.svelte";
  import ZshrcSection from "$lib/components/sections/ZshrcSection.svelte";

  const sectionMap: Record<string, any> = {
    clean_artifacts: CleanArtifactsSection,
    packages: PackagesSection,
    ghostty: GhosttySection,
    starship: StarshipSection,
    fastfetch: FastfetchSection,
    zsh_plugins: ZshPluginsSection,
    tmux: TmuxSection,
    neovim: NeovimSection,
    tool_configs: ToolConfigsSection,
    git: GitSection,
    themes: ThemesSection,
    tmux_scripts: TmuxScriptsSection,
    zshrc: ZshrcSection,
  };
</script>

<main class="main-panel">
  {#if uiState.showPaletteEditor}
    <PaletteEditor />
  {:else if uiState.showExecutionPanel}
    <ExecutionPanel />
  {:else if uiState.showPreview}
    <ScriptPreview />
  {:else}
    <div class="section-content">
      {#each Object.entries(sectionMap) as [id, Component]}
        {#if uiState.activeSection === id}
          <svelte:component this={Component} />
        {/if}
      {/each}
    </div>
  {/if}
</main>

<style>
  .main-panel {
    flex: 1;
    overflow-y: auto;
    padding: 24px 32px;
  }

  .section-content {
    max-width: 800px;
  }
</style>
