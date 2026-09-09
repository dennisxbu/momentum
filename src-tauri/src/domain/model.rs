use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum MomentumError {
    #[error("{0}")]
    Validation(String),
    #[error("Lokaler Speicherfehler: {0}")]
    Storage(#[from] rusqlite::Error),
    #[error("Dateifehler: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ungültiger Export: {0}")]
    Export(#[from] serde_json::Error),
    #[error("{0}")]
    Security(String),
}

pub type Result<T> = std::result::Result<T, MomentumError>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PropertyKind {
    Text,
    Number,
    Date,
    Choice,
    Relation,
}

impl fmt::Display for PropertyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Date => "date",
            Self::Choice => "choice",
            Self::Relation => "relation",
        })
    }
}

impl FromStr for PropertyKind {
    type Err = MomentumError;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "text" => Ok(Self::Text),
            "number" => Ok(Self::Number),
            "date" => Ok(Self::Date),
            "choice" => Ok(Self::Choice),
            "relation" => Ok(Self::Relation),
            _ => Err(MomentumError::Validation(format!(
                "Unbekannte Eigenschaftsform: {value}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownFact {
    pub label: String,
    pub value: String,
    pub state: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionChoice {
    pub id: String,
    pub label: String,
    pub tone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BriefingView {
    pub phase: String,
    pub day_label: String,
    pub overline: String,
    pub title: String,
    pub lead: String,
    pub known: Vec<KnownFact>,
    pub meaning: String,
    pub recommendation: String,
    pub reason: String,
    pub alternative: String,
    pub alternative_cost: String,
    pub unknowns: Vec<String>,
    pub changed: Vec<String>,
    pub unchanged: Vec<String>,
    pub question: Option<String>,
    pub actions: Vec<ActionChoice>,
    pub progress: u8,
    pub status_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    pub id: String,
    pub logical_id: String,
    pub name: String,
    pub kind: PropertyKind,
    pub unit: Option<String>,
    pub options: Vec<String>,
    pub meaning: Option<String>,
    pub meaning_confirmed: bool,
    pub version: i64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemValue {
    pub property_id: String,
    pub logical_id: String,
    pub property_name: String,
    pub kind: PropertyKind,
    pub unit: Option<String>,
    pub property_version: i64,
    pub display: String,
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemView {
    pub id: String,
    pub title: String,
    pub values: Vec<ItemValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub properties: Vec<PropertyDefinition>,
    pub items: Vec<ItemView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioView {
    pub collections: Vec<CollectionView>,
    pub selected_collection_id: String,
    pub view_label: String,
    pub view_explanation: String,
    pub synthetic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceView {
    pub briefing: BriefingView,
    pub studio: StudioView,
    pub history_count: i64,
    pub storage_label: String,
    pub encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDraft {
    pub name: String,
    pub kind: PropertyKind,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub meaning: Option<String>,
    #[serde(default)]
    pub meaning_confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionDraft {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub properties: Vec<PropertyDraft>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemValueDraft {
    pub property_id: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioState {
    pub phase: String,
    pub home_after_uni: Option<bool>,
    pub plan_confirmed: bool,
    pub uni_extended: bool,
    pub replan_confirmed: bool,
    pub learning_result: Option<String>,
    pub irrelevant_color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RelevantDeadline {
    pub collection: String,
    pub item: String,
    pub date: String,
    pub meaning: String,
    pub related_intention: String,
}
