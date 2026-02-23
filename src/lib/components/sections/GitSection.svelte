<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import SectionHeader from "$lib/components/shared/SectionHeader.svelte";
  import Toggle from "$lib/components/shared/Toggle.svelte";
  import TextInput from "$lib/components/shared/TextInput.svelte";
  import Select from "$lib/components/shared/Select.svelte";

  let config = $derived(configState.config.git);

  // Dirty tracking
  let prev = $state(JSON.stringify(configState.config.git));
  $effect(() => {
    const curr = JSON.stringify(configState.config.git);
    if (curr !== prev) {
      prev = curr;
      configState.dirty = true;
      configState.debouncedApply("git");
    }
  });

  function toggleAlias(index: number) {
    configState.config.git.aliases[index].enabled =
      !configState.config.git.aliases[index].enabled;
  }

  const mergeConflictOptions = [
    { value: "merge", label: "Merge" },
    { value: "diff3", label: "Diff3" },
    { value: "zdiff3", label: "ZDiff3" },
  ];

  const diffAlgorithmOptions = [
    { value: "default", label: "Default" },
    { value: "patience", label: "Patience" },
    { value: "histogram", label: "Histogram" },
    { value: "minimal", label: "Minimal" },
    { value: "myers", label: "Myers" },
  ];

  const pushDefaultOptions = [
    { value: "current", label: "Current" },
    { value: "simple", label: "Simple" },
    { value: "matching", label: "Matching" },
  ];
</script>

<section class="section-panel">
  <SectionHeader icon="🔀" number={10} title="Git" description="Git configuration, delta pager, and aliases" />

  <!-- Delta -->
  <div class="sub-section">
    <h3 class="sub-heading">Delta</h3>
    <div class="field-group">
      <Toggle
        label="Navigate"
        description="Enable n/N navigation between diff sections"
        bind:value={configState.config.git.delta.navigate}
      />
      <Toggle
        label="Dark Mode"
        description="Use dark background theme"
        bind:value={configState.config.git.delta.dark}
      />
      <Toggle
        label="Line Numbers"
        description="Show line numbers in diffs"
        bind:value={configState.config.git.delta.line_numbers}
      />
      <Toggle
        label="Side by Side"
        description="Display diffs in side-by-side layout"
        bind:value={configState.config.git.delta.side_by_side}
      />
      <TextInput
        label="Syntax Theme"
        bind:value={configState.config.git.delta.syntax_theme}
        placeholder="ayu-dark"
        description="Syntax highlighting theme name"
      />
      <TextInput
        label="File Style"
        bind:value={configState.config.git.delta.file_style}
        placeholder="bold yellow"
        description="Style for file headers"
        monospace
      />
      <TextInput
        label="Hunk Header Style"
        bind:value={configState.config.git.delta.hunk_header_style}
        placeholder="syntax bold"
        description="Style for hunk headers"
        monospace
      />
      <TextInput
        label="Minus Style"
        bind:value={configState.config.git.delta.minus_style}
        placeholder="syntax #2d1517"
        description="Style for removed lines"
        monospace
      />
      <TextInput
        label="Plus Style"
        bind:value={configState.config.git.delta.plus_style}
        placeholder="syntax #152e1a"
        description="Style for added lines"
        monospace
      />
    </div>
  </div>

  <!-- Git Settings -->
  <div class="sub-section">
    <h3 class="sub-heading">Git Settings</h3>
    <div class="field-group">
      <Select
        label="Merge Conflict Style"
        bind:value={configState.config.git.merge_conflictstyle}
        options={mergeConflictOptions}
        description="Format for merge conflict markers"
      />
      <Select
        label="Diff Algorithm"
        bind:value={configState.config.git.diff_algorithm}
        options={diffAlgorithmOptions}
        description="Algorithm used for generating diffs"
      />
      <Toggle
        label="Pull Rebase"
        description="Rebase instead of merge on git pull"
        bind:value={configState.config.git.pull_rebase}
      />
      <Toggle
        label="Rebase Autostash"
        description="Automatically stash/unstash during rebase"
        bind:value={configState.config.git.rebase_autostash}
      />
      <Toggle
        label="Auto Setup Remote"
        description="Automatically set upstream on first push"
        bind:value={configState.config.git.push_auto_setup_remote}
      />
      <Select
        label="Push Default"
        bind:value={configState.config.git.push_default}
        options={pushDefaultOptions}
        description="Default push behavior"
      />
      <TextInput
        label="Default Branch"
        bind:value={configState.config.git.init_default_branch}
        placeholder="main"
        description="Default branch name for git init"
      />
      <TextInput
        label="Core Editor"
        bind:value={configState.config.git.core_editor}
        placeholder="nvim"
        description="Editor for commit messages and interactive rebase"
      />
    </div>
  </div>

  <!-- Aliases -->
  <div class="sub-section">
    <h3 class="sub-heading">Aliases</h3>
    <div class="field-group">
      {#if config.aliases.length === 0}
        <p class="empty-hint">No git aliases configured yet.</p>
      {:else}
        {#each config.aliases as alias, i}
          <div class="alias-row">
            <label class="alias-toggle">
              <input
                type="checkbox"
                checked={alias.enabled}
                onchange={() => toggleAlias(i)}
              />
              <span class="alias-name">{alias.name}</span>
              <span class="alias-arrow">&rarr;</span>
              <span class="alias-command">{alias.command}</span>
            </label>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</section>

<style>
  .section-panel {
    padding: 24px;
  }

  .sub-section {
    margin-bottom: 20px;
  }

  .sub-heading {
    font-size: 13px;
    font-weight: 600;
    color: var(--blue);
    margin: 16px 0 8px 0;
    padding-bottom: 4px;
    border-bottom: 1px solid color-mix(in srgb, var(--comment) 10%, transparent);
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .empty-hint {
    font-size: 12px;
    color: var(--comment);
    font-style: italic;
    margin: 8px 0;
  }

  .alias-row {
    padding: 4px 0;
  }

  .alias-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 13px;
    color: var(--fg);
  }

  .alias-toggle input[type="checkbox"] {
    accent-color: var(--blue);
    width: 14px;
    height: 14px;
  }

  .alias-name {
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
    font-weight: 600;
    color: var(--cyan);
    min-width: 60px;
  }

  .alias-arrow {
    color: var(--comment);
    font-size: 11px;
  }

  .alias-command {
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
    color: var(--comment);
  }
</style>
