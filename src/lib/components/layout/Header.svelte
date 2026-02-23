<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import { uiState } from "$lib/state/ui.svelte";
  import { generateScript, saveProfile, listProfiles, loadProfile } from "$lib/api/invoke";

  let showProfileMenu = $state(false);
  let profileName = $state("");
  let profiles = $state<string[]>([]);

  async function handleGenerate() {
    try {
      const script = await generateScript(configState.config);
      uiState.previewContent = script;
      uiState.showPreview = true;
    } catch (e) {
      uiState.previewContent = `Error: ${e}`;
      uiState.showPreview = true;
    }
  }

  async function handleRun() {
    uiState.showExecutionPanel = true;
  }

  async function handleSave() {
    if (!profileName) return;
    try {
      await saveProfile(profileName, configState.config);
      profileName = "";
      showProfileMenu = false;
    } catch (e) {
      console.error("Save failed:", e);
    }
  }

  async function toggleProfileMenu() {
    showProfileMenu = !showProfileMenu;
    if (showProfileMenu) {
      try {
        profiles = await listProfiles();
      } catch {
        profiles = [];
      }
    }
  }

  async function handleLoadProfile(name: string) {
    try {
      const config = await loadProfile(name);
      configState.loadConfig(config);
      showProfileMenu = false;
    } catch (e) {
      console.error("Load failed:", e);
    }
  }

  function handleApplyNow() {
    configState.applySection(uiState.activeSection);
  }

  function toggleAutoApply() {
    configState.autoApply = !configState.autoApply;
  }
</script>

<header class="header">
  <div class="left">
    <span class="logo">👻</span>
    <h1>Ghostty Ultimate</h1>
    <span class="subtitle">Ayu Dark Terminal Architecture</span>
  </div>
  <div class="right">
    <!-- Live Apply controls -->
    <div class="apply-controls">
      <button
        class="btn btn-apply-toggle"
        class:active={configState.autoApply}
        onclick={toggleAutoApply}
        title="Auto-apply changes to disk when settings change"
      >
        <span class="apply-dot" class:on={configState.autoApply}></span>
        Live Apply
      </button>
      <button
        class="btn btn-apply-now"
        onclick={handleApplyNow}
        disabled={configState.applyStatus === "applying"}
        title="Apply current section config to disk now"
      >
        {#if configState.applyStatus === "applying"}
          Applying...
        {:else}
          Apply Now
        {/if}
      </button>
    </div>

    <div class="separator"></div>

    <!-- UI transparency slider -->
    <div class="transparency-control" title="UI app transparency">
      <span class="transparency-label">UI</span>
      <input
        type="range"
        class="transparency-slider"
        min="0.3"
        max="1"
        step="0.05"
        bind:value={uiState.uiTransparency}
      />
    </div>

    <div class="separator"></div>

    <div class="profile-area">
      <button class="btn btn-ghost" onclick={toggleProfileMenu}>
        Profiles
      </button>
      {#if showProfileMenu}
        <div class="profile-dropdown">
          <div class="profile-save">
            <input
              type="text"
              placeholder="Profile name..."
              bind:value={profileName}
              onkeydown={(e) => e.key === "Enter" && handleSave()}
            />
            <button class="btn btn-sm" onclick={handleSave}>Save</button>
          </div>
          {#if profiles.length > 0}
            <div class="profile-list">
              {#each profiles as p}
                <button class="profile-item" onclick={() => handleLoadProfile(p)}>
                  {p}
                </button>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    </div>
    <button class="btn btn-ghost" onclick={() => configState.resetToDefaults()}>
      Reset
    </button>
    <button class="btn btn-secondary" onclick={handleGenerate}>
      Preview Script
    </button>
    <button class="btn btn-primary" onclick={handleRun}>
      Run
    </button>
  </div>
</header>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    background: var(--surface);
    border-bottom: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
    flex-shrink: 0;
    z-index: 100;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .logo {
    font-size: 24px;
  }

  h1 {
    font-size: 16px;
    font-weight: 700;
    margin: 0;
    color: var(--purple);
  }

  .subtitle {
    font-size: 11px;
    color: var(--comment);
  }

  .right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .separator {
    width: 1px;
    height: 20px;
    background: color-mix(in srgb, var(--comment) 20%, transparent);
    margin: 0 4px;
  }

  .btn {
    padding: 6px 14px;
    border: none;
    border-radius: 6px;
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.15s;
  }

  .btn-ghost {
    background: transparent;
    color: var(--comment);
  }
  .btn-ghost:hover {
    background: color-mix(in srgb, var(--surface) 80%, var(--fg));
    color: var(--fg);
  }

  .btn-secondary {
    background: var(--surface);
    color: var(--blue);
    border: 1px solid color-mix(in srgb, var(--blue) 30%, transparent);
  }
  .btn-secondary:hover {
    background: color-mix(in srgb, var(--blue) 15%, var(--surface));
  }

  .btn-primary {
    background: var(--blue);
    color: var(--bg);
    font-weight: 600;
  }
  .btn-primary:hover {
    filter: brightness(1.1);
  }

  .btn-sm {
    padding: 4px 10px;
    font-size: 11px;
    background: var(--blue);
    color: var(--bg);
    font-weight: 600;
  }

  /* Apply controls */
  .apply-controls {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .btn-apply-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    color: var(--comment);
    font-size: 11px;
    padding: 5px 10px;
    border: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
  }

  .btn-apply-toggle.active {
    color: var(--green);
    border-color: color-mix(in srgb, var(--green) 40%, transparent);
    background: color-mix(in srgb, var(--green) 8%, transparent);
  }

  .apply-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--comment);
    transition: background 0.15s;
  }

  .apply-dot.on {
    background: var(--green);
    box-shadow: 0 0 6px color-mix(in srgb, var(--green) 50%, transparent);
  }

  .btn-apply-now {
    background: color-mix(in srgb, var(--purple) 15%, var(--surface));
    color: var(--purple);
    border: 1px solid color-mix(in srgb, var(--purple) 30%, transparent);
    font-size: 11px;
    padding: 5px 10px;
  }
  .btn-apply-now:hover:not(:disabled) {
    background: color-mix(in srgb, var(--purple) 25%, var(--surface));
  }
  .btn-apply-now:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* UI Transparency */
  .transparency-control {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .transparency-label {
    font-size: 10px;
    color: var(--comment);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .transparency-slider {
    -webkit-appearance: none;
    width: 60px;
    height: 3px;
    border-radius: 2px;
    background: color-mix(in srgb, var(--comment) 30%, var(--bg));
    outline: none;
  }

  .transparency-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--blue);
    cursor: pointer;
  }

  /* Profile area */
  .profile-area {
    position: relative;
  }

  .profile-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    background: var(--surface);
    border: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
    border-radius: 8px;
    padding: 8px;
    min-width: 200px;
    z-index: 200;
  }

  .profile-save {
    display: flex;
    gap: 4px;
    margin-bottom: 8px;
  }

  .profile-save input {
    flex: 1;
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 30%, transparent);
    border-radius: 4px;
    padding: 4px 8px;
    color: var(--fg);
    font-family: inherit;
    font-size: 11px;
  }

  .profile-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .profile-item {
    background: none;
    border: none;
    color: var(--fg);
    padding: 6px 8px;
    text-align: left;
    border-radius: 4px;
    cursor: pointer;
    font-family: inherit;
    font-size: 12px;
  }
  .profile-item:hover {
    background: var(--selection);
  }
</style>
