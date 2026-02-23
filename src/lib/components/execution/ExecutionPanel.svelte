<script lang="ts">
  import { configState } from "$lib/state/config.svelte";
  import { executionState } from "$lib/state/execution.svelte";
  import { uiState } from "$lib/state/ui.svelte";
  import { generateScript } from "$lib/api/invoke";

  let scriptContent = $state("");
  let generating = $state(false);

  async function handleGenerate() {
    generating = true;
    try {
      scriptContent = await generateScript(configState.config);
    } catch (e) {
      scriptContent = `Error generating script: ${e}`;
    }
    generating = false;
  }

  async function handleRun() {
    if (!scriptContent) {
      await handleGenerate();
    }
    executionState.start();

    // Write script to temp file and execute via Tauri shell
    try {
      const { writeTextFile, tempDir } = await import("@tauri-apps/plugin-fs");
      const dir = await tempDir();
      const scriptPath = `${dir}ghostty-ultimate-${Date.now()}.sh`;
      await writeTextFile(scriptPath, scriptContent);

      const { Command } = await import("@tauri-apps/plugin-shell");
      const cmd = Command.create("run-script", [scriptPath]);

      cmd.on("close", (data) => {
        executionState.addEvent({
          event_type: data.code === 0 ? "complete" : "error",
          section: 13,
          total_sections: 13,
          message: data.code === 0 ? "Script completed successfully" : `Exit code: ${data.code}`,
          timestamp: Date.now(),
        });
      });

      cmd.on("error", (error) => {
        executionState.addEvent({
          event_type: "error",
          section: null,
          total_sections: null,
          message: `Error: ${error}`,
          timestamp: Date.now(),
        });
      });

      cmd.stdout.on("data", (line) => {
        // Parse section progress from output
        const match = line.match(/(\d+)\/13\s*[—–-]/);
        if (match) {
          executionState.addEvent({
            event_type: "progress",
            section: parseInt(match[1]),
            total_sections: 13,
            message: line,
            timestamp: Date.now(),
          });
        }
        executionState.addEvent({
          event_type: "log",
          section: null,
          total_sections: null,
          message: line,
          timestamp: Date.now(),
        });
      });

      cmd.stderr.on("data", (line) => {
        executionState.addEvent({
          event_type: "log",
          section: null,
          total_sections: null,
          message: line,
          timestamp: Date.now(),
        });
      });

      await cmd.spawn();
    } catch (e) {
      executionState.addEvent({
        event_type: "error",
        section: null,
        total_sections: null,
        message: `Failed to execute: ${e}`,
        timestamp: Date.now(),
      });
    }
  }

  let progressPercent = $derived(
    executionState.totalSections > 0
      ? (executionState.progress / executionState.totalSections) * 100
      : 0,
  );
</script>

<div class="execution-panel">
  <div class="exec-header">
    <h2>Script Execution</h2>
    <div class="exec-actions">
      {#if !executionState.running}
        <button class="btn btn-generate" onclick={handleGenerate} disabled={generating}>
          {generating ? "Generating..." : "Generate"}
        </button>
        <button class="btn btn-run" onclick={handleRun} disabled={!scriptContent && !generating}>
          Run Script
        </button>
      {/if}
      <button class="btn" onclick={() => { uiState.showExecutionPanel = false; executionState.reset(); }}>
        Close
      </button>
    </div>
  </div>

  {#if executionState.running || executionState.completed}
    <div class="progress-section">
      <div class="progress-info">
        <span class="progress-label">
          {#if executionState.completed}
            Complete
          {:else}
            Section {executionState.progress}/{executionState.totalSections}
          {/if}
        </span>
        <span class="progress-pct">{Math.round(progressPercent)}%</span>
      </div>
      <div class="progress-bar">
        <div
          class="progress-fill"
          class:complete={executionState.completed}
          style="width:{progressPercent}%"
        ></div>
      </div>
    </div>
  {/if}

  <div class="log-stream">
    {#each executionState.logs as event}
      <div class="log-line" class:error={event.event_type === "error"} class:progress={event.event_type === "progress"}>
        {event.message}
      </div>
    {/each}
    {#if executionState.logs.length === 0 && scriptContent}
      <div class="log-placeholder">Script generated. Click "Run Script" to execute.</div>
    {:else if executionState.logs.length === 0}
      <div class="log-placeholder">Click "Generate" to create the script, then "Run" to execute.</div>
    {/if}
  </div>
</div>

<style>
  .execution-panel {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .exec-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
    flex-shrink: 0;
  }

  .exec-header h2 {
    margin: 0;
    font-size: 16px;
    color: var(--purple);
  }

  .exec-actions {
    display: flex;
    gap: 8px;
  }

  .btn {
    padding: 6px 14px;
    border: 1px solid color-mix(in srgb, var(--comment) 20%, transparent);
    border-radius: 6px;
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
    background: var(--surface);
    color: var(--fg);
  }

  .btn-generate {
    background: var(--surface);
    color: var(--blue);
    border-color: color-mix(in srgb, var(--blue) 30%, transparent);
  }

  .btn-run {
    background: var(--green);
    color: var(--bg);
    border: none;
    font-weight: 600;
  }
  .btn-run:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .progress-section {
    margin-bottom: 16px;
    flex-shrink: 0;
  }

  .progress-info {
    display: flex;
    justify-content: space-between;
    margin-bottom: 6px;
  }

  .progress-label {
    font-size: 12px;
    color: var(--fg);
  }

  .progress-pct {
    font-size: 12px;
    color: var(--blue);
    font-family: "JetBrains Mono", monospace;
  }

  .progress-bar {
    height: 6px;
    background: color-mix(in srgb, var(--comment) 20%, var(--bg));
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--blue);
    border-radius: 3px;
    transition: width 0.3s ease;
  }
  .progress-fill.complete {
    background: var(--green);
  }

  .log-stream {
    flex: 1;
    background: var(--bg);
    border: 1px solid color-mix(in srgb, var(--comment) 15%, transparent);
    border-radius: 8px;
    padding: 12px;
    overflow-y: auto;
    font-family: "JetBrains Mono", monospace;
    font-size: 12px;
    line-height: 1.6;
  }

  .log-line {
    color: var(--fg);
    white-space: pre-wrap;
    word-break: break-all;
  }
  .log-line.error {
    color: var(--red);
  }
  .log-line.progress {
    color: var(--blue);
    font-weight: 600;
  }

  .log-placeholder {
    color: var(--comment);
    text-align: center;
    padding: 40px 20px;
    font-family: inherit;
  }
</style>
