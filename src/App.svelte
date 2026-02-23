<script lang="ts">
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import MainPanel from "$lib/components/layout/MainPanel.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import StatusBar from "$lib/components/layout/StatusBar.svelte";
  import Toast from "$lib/components/shared/Toast.svelte";
  import { configState } from "$lib/state/config.svelte";
  import { uiState } from "$lib/state/ui.svelte";
</script>

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
