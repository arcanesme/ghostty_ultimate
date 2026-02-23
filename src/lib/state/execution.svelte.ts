import type { ExecutionEvent } from "$lib/api/types";

class ExecutionState {
  running = $state(false);
  progress = $state(0);
  totalSections = $state(13);
  currentSection = $state("");
  logs = $state<ExecutionEvent[]>([]);
  errors = $state<string[]>([]);
  completed = $state(false);

  addEvent(event: ExecutionEvent) {
    this.logs = [...this.logs, event];
    if (event.event_type === "progress" && event.section !== null) {
      this.progress = event.section;
      this.currentSection = event.message;
    }
    if (event.event_type === "error") {
      this.errors = [...this.errors, event.message];
    }
    if (event.event_type === "complete") {
      this.completed = true;
      this.running = false;
      this.progress = this.totalSections;
    }
  }

  start() {
    this.running = true;
    this.progress = 0;
    this.logs = [];
    this.errors = [];
    this.completed = false;
    this.currentSection = "";
  }

  reset() {
    this.running = false;
    this.progress = 0;
    this.logs = [];
    this.errors = [];
    this.completed = false;
    this.currentSection = "";
  }
}

export const executionState = new ExecutionState();
