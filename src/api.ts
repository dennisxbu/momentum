import { invoke } from "@tauri-apps/api/core";
import type { CollectionDraft, ItemDraftValue, WorkspaceView } from "./types";

const inTauri = () => "__TAURI_INTERNALS__" in window;

export async function loadWorkspace(): Promise<WorkspaceView> {
  if (!inTauri()) {
    throw new Error("Momentum muss als Desktop-App gestartet werden.");
  }
  return invoke<WorkspaceView>("load_workspace");
}

export function respondToBriefing(action: string): Promise<WorkspaceView> {
  return invoke("respond_to_briefing", { action });
}

export function createCollection(draft: CollectionDraft): Promise<WorkspaceView> {
  return invoke("create_collection", { draft });
}

export function createItem(
  collectionId: string,
  title: string,
  values: ItemDraftValue[],
): Promise<WorkspaceView> {
  return invoke("create_item", { collectionId, title, values });
}

export function renameProperty(propertyId: string, newName: string): Promise<WorkspaceView> {
  return invoke("rename_property", { propertyId, newName });
}

export function setStudioCollection(collectionId: string): Promise<WorkspaceView> {
  return invoke("select_studio_collection", { collectionId });
}

export function exportData(): Promise<string | null> {
  return invoke("export_data");
}

export function restoreData(): Promise<WorkspaceView | null> {
  return invoke("restore_data");
}

export function resetDemo(): Promise<WorkspaceView> {
  return invoke("reset_demo");
}
