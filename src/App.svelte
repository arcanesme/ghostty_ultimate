<script lang="ts">
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import MainPanel from "$lib/components/layout/MainPanel.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import StatusBar from "$lib/components/layout/StatusBar.svelte";
  import Toast from "$lib/components/shared/Toast.svelte";
  import { configState } from "$lib/state/config.svelte";
  import { uiState } from "$lib/state/ui.svelte";
  import { SECTIONS } from "$lib/api/types";

  // Section IDs in order for Cmd+1-9,0 shortcuts
  const sectionIds = SECTIONS.map((s) => s.id);

  function handleKeydown(e: KeyboardEvent) {
    const meta = e.metaKey || e.ctrlKey;
    if (!meta) return;

    // Cmd+S — Apply current section
    if (e.key === "s") {
      e.preventDefault();
      configState.applySection(uiState.activeSection);
      return;
    }

    // Cmd+E — Export config
    if (e.key === "e" && !e.shiftKey) {
      e.preventDefault();
      window.dispatchEvent(new CustomEvent("trigger-export"));
      return;
    }

    // Cmd+Shift+P — Toggle profile menu
    if (e.key === "P" && e.shiftKey) {
      e.preventDefault();
      window.dispatchEvent(new CustomEvent("toggle-profile-menu"));
      return;
    }

    // Cmd+G — Generate script preview
    if (e.key === "g") {
      e.preventDefault();
      window.dispatchEvent(new CustomEvent("trigger-generate"));
      return;
    }

    // Cmd+R — Run
    if (e.key === "r") {
      e.preventDefault();
      uiState.showExecutionPanel = true;
      return;
    }

    // Cmd+1 through Cmd+9 — Jump to section 1-9
    if (e.key >= "1" && e.key <= "9") {
      e.preventDefault();
      const idx = parseInt(e.key) - 1;
      if (idx < sectionIds.length) {
        uiState.activeSection = sectionIds[idx];
      }
      return;
    }

    // Cmd+0 — Jump to section 10
    if (e.key === "0") {
      e.preventDefault();
      if (sectionIds.length > 9) {
        uiState.activeSection = sectionIds[9];
      }
      return;
    }

    // Cmd+- — Jump to section 11
    if (e.key === "-") {
      e.preventDefault();
      if (sectionIds.length > 10) {
        uiState.activeSection = sectionIds[10];
      }
      return;
    }

    // Cmd+= — Jump to section 12
    if (e.key === "=") {
      e.preventDefault();
      if (sectionIds.length > 11) {
        uiState.activeSection = sectionIds[11];
      }
      return;
    }

    // Cmd+\ — Jump to section 13
    if (e.key === "\\") {
      e.preventDefault();
      if (sectionIds.length > 12) {
        uiState.activeSection = sectionIds[12];
      }
      return;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="app"
  style="--bg:{configState.config.palette.bg};--fg:{configState.config.palette.fg};--surface:{configState.config.palette.surface};--selection:{configState.config.palette.selection};--comment:{configState.config.palette.comment};--red:{configState.config.palette.red};--green:{configState.config.palette.green};--yellow:{configState.config.palette.yellow};--blue:{configState.config.palette.blue};--purple:{configState.config.palette.purple};--cyan:{configState.config.palette.cyan};--orange:{configState.config.palette.orange};--ui-opacity:{uiState.uiTransparency};--ui-blur:{uiState.uiBlur}px"
>
  <Header />
  <div class="content">
    <Sidebar />
    <MainPanel />
  </div>
  <StatusBar />
  <Toast />
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: "JetBrains Mono", "SF Mono", "Fira Code", monospace;
    background: transparent;
    color: var(--fg);
    overflow: hidden;
    height: 100vh;
  }

  :global(*) {
    box-sizing: border-box;
  }

  :global(::-webkit-scrollbar) {
    width: 6px;
  }

  :global(::-webkit-scrollbar-track) {
    background: var(--bg);
  }

  :global(::-webkit-scrollbar-thumb) {
    background: var(--surface);
    border-radius: 3px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: var(--comment);
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: color-mix(in srgb, var(--bg) calc(var(--ui-opacity) * 100%), transparent);
    backdrop-filter: blur(var(--ui-blur));
    -webkit-backdrop-filter: blur(var(--ui-blur));
    color: var(--fg);
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
</style>
