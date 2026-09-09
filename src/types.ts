export type Confidence = "bestätigt" | "angegeben" | "berechnet" | "unbekannt";

export interface KnownFact {
  label: string;
  value: string;
  state: Confidence;
  source: string;
}

export interface ActionChoice {
  id: string;
  label: string;
  tone: "primary" | "secondary" | "quiet";
  preview?: string[];
}

export interface BriefingView {
  phase: "morning" | "changed" | "evening" | "next_day";
  day_label: string;
  overline: string;
  title: string;
  lead: string;
  known: KnownFact[];
  meaning: string;
  recommendation: string;
  reason: string;
  alternative: string;
  alternative_cost: string;
  unknowns: string[];
  changed: string[];
  unchanged: string[];
  question?: string;
  actions: ActionChoice[];
  progress: number;
  status_note: string;
}

export type PropertyKind = "text" | "number" | "date" | "choice" | "relation";

export interface PropertyDefinition {
  id: string;
  logical_id: string;
  name: string;
  kind: PropertyKind;
  unit?: string;
  options: string[];
  meaning?: string;
  meaning_confirmed: boolean;
  version: number;
  active: boolean;
}

export interface ItemValue {
  property_id: string;
  logical_id: string;
  property_name: string;
  property_version: number;
  kind: PropertyKind;
  display: string;
  raw: string | number;
}

export interface CollectionItem {
  id: string;
  title: string;
  values: ItemValue[];
}

export interface CollectionView {
  id: string;
  name: string;
  description: string;
  properties: PropertyDefinition[];
  items: CollectionItem[];
}

export interface StudioView {
  collections: CollectionView[];
  selected_collection_id: string;
  view_label: string;
  view_explanation: string;
  synthetic: boolean;
}

export interface WorkspaceView {
  briefing: BriefingView;
  studio: StudioView;
  history_count: number;
  storage_label: string;
  encrypted: boolean;
}

export interface PropertyDraft {
  name: string;
  kind: PropertyKind;
  unit?: string;
  options?: string[];
  meaning?: string;
  meaning_confirmed: boolean;
}

export interface CollectionDraft {
  name: string;
  description: string;
  properties: PropertyDraft[];
}

export interface ItemDraftValue {
  property_id: string;
  value: string;
}
