import { invoke } from "@tauri-apps/api/core";
import type {
  GhosttyConfig,
  NamedPalette,
  ExecutionEvent,
} from "./types";

export async function getDefaultConfig(): Promise<GhosttyConfig> {
  return invoke<GhosttyConfig>("get_default_config");
}

export async function validateConfig(
  config: GhosttyConfig,
): Promise<string[]> {
  return invoke<string[]>("validate_config", { config });
}

export async function generateScript(
  config: GhosttyConfig,
): Promise<string> {
  return invoke<string>("generate_script", { config });
}

export async function previewSection(
  config: GhosttyConfig,
  section: string,
): Promise<string> {
  return invoke<string>("preview_section", { config, section });
}

export async function getBuiltinPalettes(): Promise<NamedPalette[]> {
  return invoke<NamedPalette[]>("get_builtin_palettes");
}

export async function saveProfile(
  name: string,
  config: GhosttyConfig,
): Promise<void> {
  return invoke("save_profile", { name, config });
}

export async function loadProfile(name: string): Promise<GhosttyConfig> {
  return invoke<GhosttyConfig>("load_profile", { name });
}

export async function listProfiles(): Promise<string[]> {
  return invoke<string[]>("list_profiles");
}

export async function deleteProfile(name: string): Promise<void> {
  return invoke("delete_profile", { name });
}

export async function exportProfile(
  config: GhosttyConfig,
  path: string,
): Promise<void> {
  return invoke("export_profile", { config, path });
}

export async function importProfile(path: string): Promise<GhosttyConfig> {
  return invoke<GhosttyConfig>("import_profile", { path });
}

export async function detectInstalledPackages(): Promise<string[]> {
  return invoke<string[]>("detect_installed_packages");
}

export async function detectInstalledFonts(): Promise<string[]> {
  return invoke<string[]>("detect_installed_fonts");
}

export async function applyConfig(
  config: GhosttyConfig,
  section: string,
): Promise<string> {
  return invoke<string>("apply_config", { config, section });
}
