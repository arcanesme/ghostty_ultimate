class UiState {
  activeSection = $state("ghostty");
  showPaletteEditor = $state(false);
  showExecutionPanel = $state(false);
  showPreview = $state(false);
  previewContent = $state("");
  sidebarCollapsed = $state(false);

  // Toast notifications
  showToast = $state(false);
  toastMessage = $state("");
  toastType = $state<"success" | "error">("success");

  // UI app transparency
  uiTransparency = $state(1.0);
  uiBlur = $state(10);

  private toastTimer: ReturnType<typeof setTimeout> | null = null;

  toast(message: string, type: "success" | "error" = "success", duration = 3000) {
    if (this.toastTimer) clearTimeout(this.toastTimer);
    this.toastMessage = message;
    this.toastType = type;
    this.showToast = true;
    this.toastTimer = setTimeout(() => {
      this.showToast = false;
      this.toastTimer = null;
    }, duration);
  }
}

export const uiState = new UiState();
