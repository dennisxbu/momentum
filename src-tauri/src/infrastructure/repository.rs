use crate::domain::*;
use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};
use uuid::Uuid;

use super::crypto::load_or_create_key;

const PROJECT_COLLECTION: &str = "11111111-1111-4111-8111-111111111111";
const CHAIR_COLLECTION: &str = "22222222-2222-4222-8222-222222222222";
const PROJECT_ITEM: &str = "41111111-1111-4111-8111-111111111111";
const CHAIR_ONE: &str = "42222222-2222-4222-8222-222222222221";
const CHAIR_TWO: &str = "42222222-2222-4222-8222-222222222222";
const CHAIR_MODEL: &str = "33333333-3333-4333-8333-333333333331";
const CHAIR_PRICE: &str = "33333333-3333-4333-8333-333333333332";
const CHAIR_DEADLINE: &str = "33333333-3333-4333-8333-333333333333";
const CHAIR_IMPRESSION: &str = "33333333-3333-4333-8333-333333333334";
const CHAIR_PROJECT: &str = "33333333-3333-4333-8333-333333333335";

pub struct Repository {
    connection: Connection,
    database_path: PathBuf,
}

impl Repository {
    pub fn open(data_dir: &Path) -> Result<Self> {
        fs::create_dir_all(data_dir)?;
        let key = load_or_create_key(&data_dir.join("momentum.key"))?;
        let database_path = data_dir.join("momentum.db");
        let connection = Self::open_encrypted_connection(&database_path, &key)?;
        let mut repository = Self {
            connection,
            database_path,
        };
        repository.migrate()?;
        repository.seed_if_empty()?;
        Ok(repository)
    }

    fn open_encrypted_connection(path: &Path, key: &[u8]) -> Result<Connection> {
        let connection = Connection::open(path)?;
        let key_hex: String = key.iter().map(|byte| format!("{byte:02x}")).collect();
        connection.execute_batch(&format!("PRAGMA key = \"x'{key_hex}'\";"))?;
        connection.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))?;
        connection.execute_batch(
            "PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL; PRAGMA synchronous = FULL;",
        )?;
        Ok(connection)
    }

    fn migrate(&mut self) -> Result<()> {
        let transaction = self.connection.transaction()?;
        transaction.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL CHECK(length(trim(name)) > 0),
                description TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS property_definitions (
                id TEXT PRIMARY KEY,
                logical_id TEXT NOT NULL,
                collection_id TEXT NOT NULL REFERENCES collections(id),
                name TEXT NOT NULL CHECK(length(trim(name)) > 0),
                kind TEXT NOT NULL CHECK(kind IN ('text','number','date','choice','relation')),
                unit TEXT,
                options_json TEXT NOT NULL DEFAULT '[]',
                meaning TEXT,
                meaning_confirmed INTEGER NOT NULL CHECK(meaning_confirmed IN (0,1)),
                version INTEGER NOT NULL CHECK(version > 0),
                active INTEGER NOT NULL CHECK(active IN (0,1)),
                supersedes_id TEXT REFERENCES property_definitions(id),
                created_at TEXT NOT NULL,
                UNIQUE(logical_id, version)
            );
            CREATE UNIQUE INDEX IF NOT EXISTS one_active_property_version
                ON property_definitions(logical_id) WHERE active = 1;
            CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                collection_id TEXT NOT NULL REFERENCES collections(id),
                title TEXT NOT NULL CHECK(length(trim(title)) > 0),
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS property_values (
                id TEXT PRIMARY KEY,
                item_id TEXT NOT NULL REFERENCES items(id),
                property_id TEXT NOT NULL REFERENCES property_definitions(id),
                text_value TEXT,
                number_value REAL,
                date_value TEXT,
                choice_value TEXT,
                relation_item_id TEXT REFERENCES items(id),
                created_at TEXT NOT NULL,
                CHECK (
                    (text_value IS NOT NULL) + (number_value IS NOT NULL) +
                    (date_value IS NOT NULL) + (choice_value IS NOT NULL) +
                    (relation_item_id IS NOT NULL) = 1
                ),
                UNIQUE(item_id, property_id)
            );
            CREATE TABLE IF NOT EXISTS scenario_state (
                id INTEGER PRIMARY KEY CHECK(id = 1),
                phase TEXT NOT NULL,
                home_after_uni INTEGER,
                plan_confirmed INTEGER NOT NULL CHECK(plan_confirmed IN (0,1)),
                uni_extended INTEGER NOT NULL CHECK(uni_extended IN (0,1)),
                replan_confirmed INTEGER NOT NULL CHECK(replan_confirmed IN (0,1)),
                learning_result TEXT,
                irrelevant_color TEXT
            );
            CREATE TABLE IF NOT EXISTS change_events (
                id TEXT PRIMARY KEY,
                occurred_at TEXT NOT NULL,
                kind TEXT NOT NULL,
                summary TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS tool_settings (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL CHECK(status IN ('enabled','paused','hidden')),
                config_json TEXT NOT NULL DEFAULT '{}'
            );
            INSERT OR IGNORE INTO schema_migrations(version, applied_at) VALUES (1, datetime('now'));"
        )?;
        transaction.commit()?;
        Ok(())
    }

    fn seed_if_empty(&mut self) -> Result<()> {
        let count: i64 =
            self.connection
                .query_row("SELECT count(*) FROM collections", [], |row| row.get(0))?;
        if count == 0 {
            let transaction = self.connection.transaction()?;
            Self::seed(&transaction)?;
            transaction.commit()?;
        }
        Ok(())
    }

    fn seed(transaction: &Transaction<'_>) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        transaction.execute(
            "INSERT INTO collections VALUES (?1, ?2, ?3, ?4)",
            params![
                PROJECT_COLLECTION,
                "Vorhaben",
                "Was über mehrere Schritte hinweg Bedeutung hat.",
                now
            ],
        )?;
        transaction.execute("INSERT INTO collections VALUES (?1, ?2, ?3, ?4)", params![CHAIR_COLLECTION, "Stuhl-Kandidaten", "Eine frei angelegte Sammlung für die synthetische Entscheidung „Arbeitsplatz verbessern“.", now])?;

        let properties = [
            (
                CHAIR_MODEL,
                "Modell",
                "text",
                None,
                "[]",
                Some("Bezeichnung des Kandidaten"),
                0,
            ),
            (
                CHAIR_PRICE,
                "Preis",
                "number",
                Some("EUR"),
                "[]",
                Some("Vergleich mit dem gesetzten Budget von 450 EUR"),
                1,
            ),
            (
                CHAIR_DEADLINE,
                "Rückgabefrist",
                "date",
                None,
                "[]",
                Some("Letzter Rückgabetag bei bestätigter Rückgabeabsicht"),
                1,
            ),
            (
                CHAIR_IMPRESSION,
                "Eindruck",
                "choice",
                None,
                "[\"offen\",\"passend\",\"ungeeignet\"]",
                Some("Persönlicher Eindruck, keine automatische Entscheidung"),
                1,
            ),
            (
                CHAIR_PROJECT,
                "Gehört zu",
                "relation",
                None,
                "[]",
                Some("Verbindung zu einem aktiven Vorhaben"),
                1,
            ),
        ];
        for (id, name, kind, unit, options, meaning, confirmed) in properties {
            transaction.execute(
                "INSERT INTO property_definitions(id, logical_id, collection_id, name, kind, unit, options_json, meaning, meaning_confirmed, version, active, supersedes_id, created_at)
                 VALUES (?1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1, 1, NULL, ?9)",
                params![id, CHAIR_COLLECTION, name, kind, unit, options, meaning, confirmed, now],
            )?;
        }
        transaction.execute(
            "INSERT INTO items VALUES (?1, ?2, ?3, ?4)",
            params![
                PROJECT_ITEM,
                PROJECT_COLLECTION,
                "Arbeitsplatz verbessern",
                now
            ],
        )?;
        transaction.execute(
            "INSERT INTO items VALUES (?1, ?2, ?3, ?4)",
            params![CHAIR_ONE, CHAIR_COLLECTION, "Nordic Ergo", now],
        )?;
        transaction.execute(
            "INSERT INTO items VALUES (?1, ?2, ?3, ?4)",
            params![CHAIR_TWO, CHAIR_COLLECTION, "Mira Work", now],
        )?;

        Self::insert_seed_value(
            transaction,
            CHAIR_ONE,
            CHAIR_MODEL,
            Some("Nordic Ergo"),
            None,
            None,
            None,
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_ONE,
            CHAIR_PRICE,
            None,
            Some(429.0),
            None,
            None,
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_ONE,
            CHAIR_DEADLINE,
            None,
            None,
            Some("2026-09-10"),
            None,
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_ONE,
            CHAIR_IMPRESSION,
            None,
            None,
            None,
            Some("offen"),
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_ONE,
            CHAIR_PROJECT,
            None,
            None,
            None,
            None,
            Some(PROJECT_ITEM),
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_TWO,
            CHAIR_MODEL,
            Some("Mira Work"),
            None,
            None,
            None,
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_TWO,
            CHAIR_PRICE,
            None,
            Some(379.0),
            None,
            None,
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_TWO,
            CHAIR_DEADLINE,
            None,
            None,
            Some("2026-09-18"),
            None,
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_TWO,
            CHAIR_IMPRESSION,
            None,
            None,
            None,
            Some("passend"),
            None,
            &now,
        )?;
        Self::insert_seed_value(
            transaction,
            CHAIR_TWO,
            CHAIR_PROJECT,
            None,
            None,
            None,
            None,
            Some(PROJECT_ITEM),
            &now,
        )?;

        transaction.execute(
            "INSERT INTO scenario_state VALUES (1, 'morning_unknown', NULL, 0, 0, 0, NULL, NULL)",
            [],
        )?;
        transaction.execute(
            "INSERT INTO tool_settings VALUES ('example-calendar', 'hidden', '{}')",
            [],
        )?;
        Self::event(
            transaction,
            "seed",
            "Synthetischer K5-Prüffall angelegt",
            json!({"synthetic": true}),
        )?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_seed_value(
        transaction: &Transaction<'_>,
        item: &str,
        property: &str,
        text: Option<&str>,
        number: Option<f64>,
        date: Option<&str>,
        choice: Option<&str>,
        relation: Option<&str>,
        now: &str,
    ) -> Result<()> {
        transaction.execute(
            "INSERT INTO property_values(id,item_id,property_id,text_value,number_value,date_value,choice_value,relation_item_id,created_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![Uuid::new_v4().to_string(), item, property, text, number, date, choice, relation, now],
        )?;
        Ok(())
    }

    fn event(
        transaction: &Transaction<'_>,
        kind: &str,
        summary: &str,
        payload: Value,
    ) -> Result<()> {
        transaction.execute(
            "INSERT INTO change_events VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                Uuid::new_v4().to_string(),
                Utc::now().to_rfc3339(),
                kind,
                summary,
                payload.to_string()
            ],
        )?;
        Ok(())
    }

    pub fn scenario(&self) -> Result<ScenarioState> {
        self.connection.query_row(
            "SELECT phase, home_after_uni, plan_confirmed, uni_extended, replan_confirmed, learning_result, irrelevant_color FROM scenario_state WHERE id = 1",
            [],
            |row| Ok(ScenarioState {
                phase: row.get(0)?,
                home_after_uni: row.get::<_, Option<i64>>(1)?.map(|value| value != 0),
                plan_confirmed: row.get::<_, i64>(2)? != 0,
                uni_extended: row.get::<_, i64>(3)? != 0,
                replan_confirmed: row.get::<_, i64>(4)? != 0,
                learning_result: row.get(5)?,
                irrelevant_color: row.get(6)?,
            }),
        ).map_err(Into::into)
    }

    pub fn apply_scenario_action(&mut self, action: &str) -> Result<()> {
        let transaction = self.connection.transaction()?;
        let (sql, summary, payload) = match action {
            "home_by_13" => (
                "UPDATE scenario_state SET phase='morning_answered', home_after_uni=1 WHERE id=1",
                "Rückkehr nach der Uni als Angabe übernommen",
                json!({"home_after_uni": true}),
            ),
            "away_after_uni" => (
                "UPDATE scenario_state SET phase='morning_answered', home_after_uni=0 WHERE id=1",
                "Späte Rückkehr nach der Uni als Angabe übernommen",
                json!({"home_after_uni": false}),
            ),
            "confirm_plan" => (
                "UPDATE scenario_state SET phase='plan_confirmed', plan_confirmed=1 WHERE id=1",
                "Morgenvorschlag konkret bestätigt",
                json!({"plan_confirmed": true}),
            ),
            "uni_extended" => (
                "UPDATE scenario_state SET phase='day_changed', uni_extended=1 WHERE id=1",
                "Uni-Verlängerung um 90 Minuten als Korrektur übernommen",
                json!({"uni_extended_minutes": 90}),
            ),
            "confirm_replan" => (
                "UPDATE scenario_state SET phase='replan_confirmed', replan_confirmed=1 WHERE id=1",
                "Begrenzte Neuplanung bestätigt",
                json!({"replan_confirmed": true}),
            ),
            "to_evening" => (
                "UPDATE scenario_state SET phase='evening' WHERE id=1",
                "Zum Abend übergegangen",
                json!({"phase": "evening"}),
            ),
            "learning_done" => (
                "UPDATE scenario_state SET phase='next_day', learning_result='confirmed_done' WHERE id=1",
                "Lernblock als eigene Beobachtung bestätigt",
                json!({"learning_result": "confirmed_done"}),
            ),
            "learning_unknown" => (
                "UPDATE scenario_state SET phase='next_day', learning_result='unknown' WHERE id=1",
                "Durchführung bewusst unbekannt gelassen",
                json!({"learning_result": "unknown"}),
            ),
            "irrelevant_color" => (
                "UPDATE scenario_state SET irrelevant_color='salbeigrün' WHERE id=1",
                "Belanglose Zusatzangabe ohne Planwirkung gespeichert",
                json!({"chair_color": "salbeigrün", "meaning_confirmed": false}),
            ),
            _ => {
                return Err(MomentumError::Validation(
                    "Diese Antwort gehört nicht zum aktuellen Assistenzschritt.".into(),
                ));
            }
        };
        let affected = transaction.execute(sql, [])?;
        if affected != 1 {
            return Err(MomentumError::Validation(
                "Der synthetische Assistenzzustand fehlt.".into(),
            ));
        }
        Self::event(&transaction, "assistant_confirmation", summary, payload)?;
        transaction.commit()?;
        Ok(())
    }

    pub fn create_collection(&mut self, draft: CollectionDraft) -> Result<String> {
        let name = draft.name.trim();
        if name.is_empty() {
            return Err(MomentumError::Validation(
                "Die Sammlung braucht einen Namen.".into(),
            ));
        }
        if draft.properties.is_empty() {
            return Err(MomentumError::Validation(
                "Lege mindestens eine Eigenschaft an.".into(),
            ));
        }
        let mut names = HashSet::new();
        for property in &draft.properties {
            if property.name.trim().is_empty() {
                return Err(MomentumError::Validation(
                    "Jede Eigenschaft braucht einen Namen.".into(),
                ));
            }
            if !names.insert(property.name.trim().to_lowercase()) {
                return Err(MomentumError::Validation(
                    "Eigenschaftsnamen dürfen sich nicht wiederholen.".into(),
                ));
            }
            Self::validate_definition(property)?;
        }
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO collections VALUES (?1,?2,?3,?4)",
            params![id, name, draft.description.trim(), now],
        )?;
        for property in &draft.properties {
            let property_id = Uuid::new_v4().to_string();
            let meaning = property
                .meaning
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty());
            transaction.execute(
                "INSERT INTO property_definitions(id,logical_id,collection_id,name,kind,unit,options_json,meaning,meaning_confirmed,version,active,supersedes_id,created_at)
                 VALUES (?1,?1,?2,?3,?4,?5,?6,?7,?8,1,1,NULL,?9)",
                params![property_id, id, property.name.trim(), property.kind.to_string(), property.unit.as_deref().map(str::trim), serde_json::to_string(&property.options)?, meaning, property.meaning_confirmed as i64, now],
            )?;
        }
        Self::event(
            &transaction,
            "collection_created",
            &format!("Sammlung „{name}“ angelegt"),
            json!({"collection_id": id, "property_count": draft.properties.len()}),
        )?;
        transaction.commit()?;
        Ok(id)
    }

    fn validate_definition(property: &PropertyDraft) -> Result<()> {
        if property.meaning_confirmed && property.meaning.as_deref().unwrap_or("").trim().is_empty()
        {
            return Err(MomentumError::Validation(format!(
                "Für „{}“ muss die bestätigte Bedeutung beschrieben sein.",
                property.name
            )));
        }
        if property.kind == PropertyKind::Number
            && property.unit.as_deref().unwrap_or("").trim().is_empty()
        {
            return Err(MomentumError::Validation(format!(
                "„{}“ braucht eine Einheit.",
                property.name
            )));
        }
        if property.kind == PropertyKind::Choice && property.options.is_empty() {
            return Err(MomentumError::Validation(format!(
                "„{}“ braucht mindestens eine Auswahlmöglichkeit.",
                property.name
            )));
        }
        Ok(())
    }

    pub fn create_item(
        &mut self,
        collection_id: &str,
        title: &str,
        values: Vec<ItemValueDraft>,
    ) -> Result<()> {
        if title.trim().is_empty() {
            return Err(MomentumError::Validation(
                "Der Eintrag braucht eine Bezeichnung.".into(),
            ));
        }
        let definitions: HashMap<String, DefinitionRow> = self
            .definition_rows(collection_id)?
            .into_iter()
            .filter(|row| row.active)
            .map(|row| (row.id.clone(), row))
            .collect();
        if definitions.is_empty() && !values.is_empty() {
            return Err(MomentumError::Validation(
                "Die Sammlung oder ihre Eigenschaften wurden nicht gefunden.".into(),
            ));
        }
        let mut seen = HashSet::new();
        let item_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO items VALUES (?1,?2,?3,?4)",
            params![item_id, collection_id, title.trim(), now],
        )?;
        for value in values {
            if !seen.insert(value.property_id.clone()) {
                return Err(MomentumError::Validation(
                    "Eine Eigenschaft wurde doppelt angegeben.".into(),
                ));
            }
            let definition = definitions.get(&value.property_id).ok_or_else(|| {
                MomentumError::Validation(
                    "Eine Eigenschaft gehört nicht zu dieser Sammlung.".into(),
                )
            })?;
            let kind = PropertyKind::from_str(&definition.kind)?;
            let (text, number, date, choice, relation) = match kind {
                PropertyKind::Text => {
                    (Some(value.value.trim().to_string()), None, None, None, None)
                }
                PropertyKind::Number => (
                    None,
                    Some(value.value.parse::<f64>().map_err(|_| {
                        MomentumError::Validation(format!(
                            "„{}“ ist keine gültige Zahl.",
                            definition.name
                        ))
                    })?),
                    None,
                    None,
                    None,
                ),
                PropertyKind::Date => {
                    chrono::NaiveDate::parse_from_str(&value.value, "%Y-%m-%d").map_err(|_| {
                        MomentumError::Validation(format!(
                            "„{}“ ist kein gültiges Datum.",
                            definition.name
                        ))
                    })?;
                    (None, None, Some(value.value), None, None)
                }
                PropertyKind::Choice => {
                    let options: Vec<String> = serde_json::from_str(&definition.options_json)?;
                    if !options.contains(&value.value) {
                        return Err(MomentumError::Validation(format!(
                            "„{}“ ist keine erlaubte Auswahl.",
                            value.value
                        )));
                    }
                    (None, None, None, Some(value.value), None)
                }
                PropertyKind::Relation => {
                    let exists: bool = transaction.query_row(
                        "SELECT EXISTS(SELECT 1 FROM items WHERE id=?1)",
                        [&value.value],
                        |row| row.get(0),
                    )?;
                    if !exists {
                        return Err(MomentumError::Validation(
                            "Das Ziel der Beziehung wurde nicht gefunden.".into(),
                        ));
                    }
                    (None, None, None, None, Some(value.value))
                }
            };
            transaction.execute(
                "INSERT INTO property_values(id,item_id,property_id,text_value,number_value,date_value,choice_value,relation_item_id,created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![Uuid::new_v4().to_string(), item_id, value.property_id, text, number, date, choice, relation, now],
            )?;
        }
        Self::event(
            &transaction,
            "item_created",
            &format!("Eintrag „{}“ angelegt", title.trim()),
            json!({"collection_id": collection_id, "item_id": item_id}),
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn rename_property(&mut self, property_id: &str, new_name: &str) -> Result<()> {
        if new_name.trim().is_empty() {
            return Err(MomentumError::Validation(
                "Der neue Name darf nicht leer sein.".into(),
            ));
        }
        let old: DefinitionRow = self.connection.query_row(
            "SELECT id,logical_id,collection_id,name,kind,unit,options_json,meaning,meaning_confirmed,version,active,supersedes_id,created_at FROM property_definitions WHERE id=?1 AND active=1",
            [property_id], DefinitionRow::from_row,
        ).optional()?.ok_or_else(|| MomentumError::Validation("Die aktive Eigenschaft wurde nicht gefunden.".into()))?;
        let new_id = Uuid::new_v4().to_string();
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "UPDATE property_definitions SET active=0 WHERE id=?1",
            [property_id],
        )?;
        transaction.execute(
            "INSERT INTO property_definitions(id,logical_id,collection_id,name,kind,unit,options_json,meaning,meaning_confirmed,version,active,supersedes_id,created_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,?11,?12)",
            params![new_id, old.logical_id, old.collection_id, new_name.trim(), old.kind, old.unit, old.options_json, old.meaning, old.meaning_confirmed, old.version + 1, old.id, Utc::now().to_rfc3339()],
        )?;
        Self::event(
            &transaction,
            "property_versioned",
            &format!(
                "Eigenschaft „{}“ in „{}“ umbenannt",
                old.name,
                new_name.trim()
            ),
            json!({"logical_id": old.logical_id, "old_definition_id": old.id, "new_definition_id": new_id, "old_name": old.name, "new_name": new_name.trim()}),
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn studio(&self, selected: Option<&str>) -> Result<StudioView> {
        let mut statement = self.connection.prepare(
            "SELECT id,name,description,created_at FROM collections ORDER BY created_at,id",
        )?;
        let collection_rows = statement
            .query_map([], CollectionRow::from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut collections = Vec::new();
        for collection in collection_rows {
            let definitions = self.definition_rows(&collection.id)?;
            let active: Vec<PropertyDefinition> = definitions
                .iter()
                .filter(|row| row.active)
                .map(DefinitionRow::to_view)
                .collect::<Result<_>>()?;
            let items = self.items_for_collection(&collection.id)?;
            collections.push(CollectionView {
                id: collection.id,
                name: collection.name,
                description: collection.description,
                properties: active,
                items,
            });
        }
        let selected_id = selected
            .filter(|id| collections.iter().any(|entry| entry.id == *id))
            .map(str::to_string)
            .or_else(|| {
                collections
                    .iter()
                    .find(|entry| entry.id == CHAIR_COLLECTION)
                    .map(|entry| entry.id.clone())
            })
            .or_else(|| collections.first().map(|entry| entry.id.clone()))
            .unwrap_or_default();
        let is_chair = selected_id == CHAIR_COLLECTION;
        Ok(StudioView {
            collections,
            selected_collection_id: selected_id,
            view_label: if is_chair {
                "Rückgabefrist zuerst"
            } else {
                "Alle Einträge"
            }
            .into(),
            view_explanation: if is_chair {
                "Zeigt die bestätigte Fristbedeutung; Preis bleibt mit seiner Einheit vergleichbar."
            } else {
                "Vollständige Sicht ohne versteckte Bewertung."
            }
            .into(),
            synthetic: true,
        })
    }

    fn definition_rows(&self, collection_id: &str) -> Result<Vec<DefinitionRow>> {
        let mut statement = self.connection.prepare(
            "SELECT id,logical_id,collection_id,name,kind,unit,options_json,meaning,meaning_confirmed,version,active,supersedes_id,created_at FROM property_definitions WHERE collection_id=?1 ORDER BY created_at,id"
        )?;
        Ok(statement
            .query_map([collection_id], DefinitionRow::from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?)
    }

    fn items_for_collection(&self, collection_id: &str) -> Result<Vec<ItemView>> {
        let order = if collection_id == CHAIR_COLLECTION {
            format!(
                "ORDER BY (SELECT date_value FROM property_values WHERE item_id=i.id AND property_id='{CHAIR_DEADLINE}') IS NULL, (SELECT date_value FROM property_values WHERE item_id=i.id AND property_id='{CHAIR_DEADLINE}'), i.title"
            )
        } else {
            "ORDER BY i.created_at, i.title".to_string()
        };
        let mut statement = self.connection.prepare(&format!(
            "SELECT i.id,i.title FROM items i WHERE i.collection_id=?1 {order}"
        ))?;
        let item_rows = statement
            .query_map([collection_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        item_rows
            .into_iter()
            .map(|(id, title)| {
                Ok(ItemView {
                    values: self.values_for_item(&id)?,
                    id,
                    title,
                })
            })
            .collect()
    }

    fn values_for_item(&self, item_id: &str) -> Result<Vec<ItemValue>> {
        let mut statement = self.connection.prepare(
            "SELECT d.id,d.logical_id,d.name,d.kind,d.unit,d.version,v.text_value,v.number_value,v.date_value,v.choice_value,v.relation_item_id,ri.title
             FROM property_values v JOIN property_definitions d ON d.id=v.property_id LEFT JOIN items ri ON ri.id=v.relation_item_id WHERE v.item_id=?1 ORDER BY d.created_at,d.id"
        )?;
        let rows = statement
            .query_map([item_id], |row| {
                let kind_raw: String = row.get(3)?;
                let unit: Option<String> = row.get(4)?;
                let text_value: Option<String> = row.get(6)?;
                let number_value: Option<f64> = row.get(7)?;
                let date_value: Option<String> = row.get(8)?;
                let choice_value: Option<String> = row.get(9)?;
                let relation_id: Option<String> = row.get(10)?;
                let relation_title: Option<String> = row.get(11)?;
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    kind_raw,
                    unit,
                    row.get::<_, i64>(5)?,
                    text_value,
                    number_value,
                    date_value,
                    choice_value,
                    relation_id,
                    relation_title,
                ))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        rows.into_iter()
            .map(
                |(
                    property_id,
                    logical_id,
                    property_name,
                    kind_raw,
                    unit,
                    property_version,
                    text_value,
                    number_value,
                    date_value,
                    choice_value,
                    relation_id,
                    relation_title,
                )| {
                    let kind = PropertyKind::from_str(&kind_raw)?;
                    let (display, raw) = match kind {
                        PropertyKind::Text => {
                            let value = text_value.unwrap_or_default();
                            (value.clone(), json!(value))
                        }
                        PropertyKind::Number => {
                            let value = number_value.unwrap_or_default();
                            (format_number(value, unit.as_deref()), json!(value))
                        }
                        PropertyKind::Date => {
                            let value = date_value.unwrap_or_default();
                            (format_date(&value), json!(value))
                        }
                        PropertyKind::Choice => {
                            let value = choice_value.unwrap_or_default();
                            (value.clone(), json!(value))
                        }
                        PropertyKind::Relation => (
                            relation_title.unwrap_or_else(|| "Unbekannter Bezug".into()),
                            json!(relation_id),
                        ),
                    };
                    Ok(ItemValue {
                        property_id,
                        logical_id,
                        property_name,
                        kind,
                        unit,
                        property_version,
                        display,
                        raw,
                    })
                },
            )
            .collect()
    }

    pub fn history_count(&self) -> Result<i64> {
        Ok(self
            .connection
            .query_row("SELECT count(*) FROM change_events", [], |row| row.get(0))?)
    }

    pub fn relevant_deadline(&self) -> Result<Option<RelevantDeadline>> {
        self.connection.query_row(
            "SELECT c.name,i.title,v.date_value,d.meaning,related.title
             FROM property_values v
             JOIN property_definitions d ON d.id=v.property_id
             JOIN items i ON i.id=v.item_id
             JOIN collections c ON c.id=i.collection_id
             JOIN property_values rv ON rv.item_id=i.id
             JOIN property_definitions rd ON rd.id=rv.property_id AND rd.kind='relation' AND rd.meaning_confirmed=1
             JOIN items related ON related.id=rv.relation_item_id
             WHERE d.kind='date' AND d.meaning_confirmed=1 AND d.active=1 AND v.date_value IS NOT NULL
             ORDER BY v.date_value LIMIT 1",
            [],
            |row| Ok(RelevantDeadline {
                collection: row.get(0)?, item: row.get(1)?, date: row.get(2)?,
                meaning: row.get::<_, Option<String>>(3)?.unwrap_or_else(|| "Bestätigte Datumsbedeutung".into()),
                related_intention: row.get(4)?,
            }),
        ).optional().map_err(Into::into)
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }

    pub fn reset_demo(&mut self) -> Result<()> {
        let transaction = self.connection.transaction()?;
        transaction.execute_batch("DELETE FROM property_values; DELETE FROM property_definitions; DELETE FROM items; DELETE FROM collections; DELETE FROM scenario_state; DELETE FROM change_events; DELETE FROM tool_settings;")?;
        Self::seed(&transaction)?;
        transaction.commit()?;
        Ok(())
    }

    pub fn export_json(&self) -> Result<String> {
        let bundle = ExportBundle::read(&self.connection)?;
        Ok(serde_json::to_string_pretty(&bundle)?)
    }

    pub fn restore_json(&mut self, source: &str) -> Result<()> {
        let bundle: ExportBundle = serde_json::from_str(source)?;
        bundle.validate()?;
        let mut temporary = Connection::open_in_memory()?;
        Self::create_import_schema(&mut temporary)?;
        bundle.insert_into(&mut temporary)?;
        let transaction = self.connection.transaction()?;
        transaction.execute_batch("DELETE FROM property_values; DELETE FROM property_definitions; DELETE FROM items; DELETE FROM collections; DELETE FROM scenario_state; DELETE FROM change_events; DELETE FROM tool_settings;")?;
        bundle.insert_transaction(&transaction)?;
        transaction.commit()?;
        Ok(())
    }

    fn create_import_schema(connection: &mut Connection) -> Result<()> {
        connection.execute_batch(
            "PRAGMA foreign_keys=ON;
             CREATE TABLE collections(id TEXT PRIMARY KEY,name TEXT NOT NULL,description TEXT NOT NULL,created_at TEXT NOT NULL);
             CREATE TABLE property_definitions(id TEXT PRIMARY KEY,logical_id TEXT NOT NULL,collection_id TEXT NOT NULL REFERENCES collections(id),name TEXT NOT NULL,kind TEXT NOT NULL CHECK(kind IN ('text','number','date','choice','relation')),unit TEXT,options_json TEXT NOT NULL,meaning TEXT,meaning_confirmed INTEGER NOT NULL,version INTEGER NOT NULL,active INTEGER NOT NULL,supersedes_id TEXT REFERENCES property_definitions(id),created_at TEXT NOT NULL,UNIQUE(logical_id,version));
             CREATE TABLE items(id TEXT PRIMARY KEY,collection_id TEXT NOT NULL REFERENCES collections(id),title TEXT NOT NULL,created_at TEXT NOT NULL);
             CREATE TABLE property_values(id TEXT PRIMARY KEY,item_id TEXT NOT NULL REFERENCES items(id),property_id TEXT NOT NULL REFERENCES property_definitions(id),text_value TEXT,number_value REAL,date_value TEXT,choice_value TEXT,relation_item_id TEXT REFERENCES items(id),created_at TEXT NOT NULL,CHECK ((text_value IS NOT NULL)+(number_value IS NOT NULL)+(date_value IS NOT NULL)+(choice_value IS NOT NULL)+(relation_item_id IS NOT NULL)=1));
             CREATE TABLE scenario_state(id INTEGER PRIMARY KEY,phase TEXT NOT NULL,home_after_uni INTEGER,plan_confirmed INTEGER NOT NULL,uni_extended INTEGER NOT NULL,replan_confirmed INTEGER NOT NULL,learning_result TEXT,irrelevant_color TEXT);
             CREATE TABLE change_events(id TEXT PRIMARY KEY,occurred_at TEXT NOT NULL,kind TEXT NOT NULL,summary TEXT NOT NULL,payload_json TEXT NOT NULL);
             CREATE TABLE tool_settings(id TEXT PRIMARY KEY,status TEXT NOT NULL,config_json TEXT NOT NULL);"
        )?;
        Ok(())
    }
}

fn format_number(value: f64, unit: Option<&str>) -> String {
    let number = if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.2}").trim_end_matches('0').to_string()
    };
    match unit {
        Some("EUR") => format!("{number} €"),
        Some(unit) if !unit.is_empty() => format!("{number} {unit}"),
        _ => number,
    }
}

fn format_date(value: &str) -> String {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map(|date| date.format("%d.%m.%Y").to_string())
        .unwrap_or_else(|_| value.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CollectionRow {
    id: String,
    name: String,
    description: String,
    created_at: String,
}
impl CollectionRow {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DefinitionRow {
    id: String,
    logical_id: String,
    collection_id: String,
    name: String,
    kind: String,
    unit: Option<String>,
    options_json: String,
    meaning: Option<String>,
    meaning_confirmed: i64,
    version: i64,
    active: bool,
    supersedes_id: Option<String>,
    created_at: String,
}
impl DefinitionRow {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            logical_id: row.get(1)?,
            collection_id: row.get(2)?,
            name: row.get(3)?,
            kind: row.get(4)?,
            unit: row.get(5)?,
            options_json: row.get(6)?,
            meaning: row.get(7)?,
            meaning_confirmed: row.get(8)?,
            version: row.get(9)?,
            active: row.get::<_, i64>(10)? != 0,
            supersedes_id: row.get(11)?,
            created_at: row.get(12)?,
        })
    }
    fn to_view(&self) -> Result<PropertyDefinition> {
        Ok(PropertyDefinition {
            id: self.id.clone(),
            logical_id: self.logical_id.clone(),
            name: self.name.clone(),
            kind: PropertyKind::from_str(&self.kind)?,
            unit: self.unit.clone(),
            options: serde_json::from_str(&self.options_json)?,
            meaning: self.meaning.clone(),
            meaning_confirmed: self.meaning_confirmed != 0,
            version: self.version,
            active: self.active,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueRow {
    id: String,
    item_id: String,
    property_id: String,
    text_value: Option<String>,
    number_value: Option<f64>,
    date_value: Option<String>,
    choice_value: Option<String>,
    relation_item_id: Option<String>,
    created_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ItemRow {
    id: String,
    collection_id: String,
    title: String,
    created_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventRow {
    id: String,
    occurred_at: String,
    kind: String,
    summary: String,
    payload_json: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolRow {
    id: String,
    status: String,
    config_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExportBundle {
    schema_version: u32,
    exported_at: String,
    notice: String,
    collections: Vec<CollectionRow>,
    property_definitions: Vec<DefinitionRow>,
    items: Vec<ItemRow>,
    property_values: Vec<ValueRow>,
    scenario: ScenarioState,
    change_events: Vec<EventRow>,
    tool_settings: Vec<ToolRow>,
}

impl ExportBundle {
    fn read(connection: &Connection) -> Result<Self> {
        Ok(Self {
            schema_version: 1,
            exported_at: Utc::now().to_rfc3339(),
            notice: "Lesbarer vollständiger Momentum-Export. Synthetischer K5-Entwicklungsstand.".into(),
            collections: query_all(connection, "SELECT id,name,description,created_at FROM collections", CollectionRow::from_row)?,
            property_definitions: query_all(connection, "SELECT id,logical_id,collection_id,name,kind,unit,options_json,meaning,meaning_confirmed,version,active,supersedes_id,created_at FROM property_definitions", DefinitionRow::from_row)?,
            items: query_all(connection, "SELECT id,collection_id,title,created_at FROM items", |row| Ok(ItemRow { id: row.get(0)?, collection_id: row.get(1)?, title: row.get(2)?, created_at: row.get(3)? }))?,
            property_values: query_all(connection, "SELECT id,item_id,property_id,text_value,number_value,date_value,choice_value,relation_item_id,created_at FROM property_values", |row| Ok(ValueRow { id: row.get(0)?, item_id: row.get(1)?, property_id: row.get(2)?, text_value: row.get(3)?, number_value: row.get(4)?, date_value: row.get(5)?, choice_value: row.get(6)?, relation_item_id: row.get(7)?, created_at: row.get(8)? }))?,
            scenario: connection.query_row("SELECT phase,home_after_uni,plan_confirmed,uni_extended,replan_confirmed,learning_result,irrelevant_color FROM scenario_state WHERE id=1", [], |row| Ok(ScenarioState { phase: row.get(0)?, home_after_uni: row.get::<_, Option<i64>>(1)?.map(|value| value != 0), plan_confirmed: row.get::<_, i64>(2)? != 0, uni_extended: row.get::<_, i64>(3)? != 0, replan_confirmed: row.get::<_, i64>(4)? != 0, learning_result: row.get(5)?, irrelevant_color: row.get(6)? }))?,
            change_events: query_all(connection, "SELECT id,occurred_at,kind,summary,payload_json FROM change_events ORDER BY occurred_at,id", |row| Ok(EventRow { id: row.get(0)?, occurred_at: row.get(1)?, kind: row.get(2)?, summary: row.get(3)?, payload_json: row.get(4)? }))?,
            tool_settings: query_all(connection, "SELECT id,status,config_json FROM tool_settings", |row| Ok(ToolRow { id: row.get(0)?, status: row.get(1)?, config_json: row.get(2)? }))?,
        })
    }

    fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(MomentumError::Validation(format!(
                "Exportversion {} wird nicht unterstützt.",
                self.schema_version
            )));
        }
        if self.collections.is_empty() {
            return Err(MomentumError::Validation(
                "Der Export enthält keine Sammlung.".into(),
            ));
        }
        if self.scenario.phase.trim().is_empty() {
            return Err(MomentumError::Validation(
                "Der Assistenzzustand fehlt.".into(),
            ));
        }
        for definition in &self.property_definitions {
            PropertyKind::from_str(&definition.kind)?;
            serde_json::from_str::<Vec<String>>(&definition.options_json)?;
        }
        for event in &self.change_events {
            serde_json::from_str::<Value>(&event.payload_json)?;
        }
        for tool in &self.tool_settings {
            serde_json::from_str::<Value>(&tool.config_json)?;
        }
        Ok(())
    }

    fn insert_into(&self, connection: &mut Connection) -> Result<()> {
        let transaction = connection.transaction()?;
        self.insert_transaction(&transaction)?;
        transaction.commit()?;
        Ok(())
    }

    fn insert_transaction(&self, transaction: &Transaction<'_>) -> Result<()> {
        for row in &self.collections {
            transaction.execute(
                "INSERT INTO collections VALUES (?1,?2,?3,?4)",
                params![row.id, row.name, row.description, row.created_at],
            )?;
        }
        let mut pending = self.property_definitions.clone();
        pending.sort_by_key(|row| row.version);
        for row in &pending {
            transaction.execute("INSERT INTO property_definitions VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)", params![row.id,row.logical_id,row.collection_id,row.name,row.kind,row.unit,row.options_json,row.meaning,row.meaning_confirmed,row.version,row.active as i64,row.supersedes_id,row.created_at])?;
        }
        for row in &self.items {
            transaction.execute(
                "INSERT INTO items VALUES (?1,?2,?3,?4)",
                params![row.id, row.collection_id, row.title, row.created_at],
            )?;
        }
        for row in &self.property_values {
            transaction.execute(
                "INSERT INTO property_values VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![
                    row.id,
                    row.item_id,
                    row.property_id,
                    row.text_value,
                    row.number_value,
                    row.date_value,
                    row.choice_value,
                    row.relation_item_id,
                    row.created_at
                ],
            )?;
        }
        transaction.execute(
            "INSERT INTO scenario_state VALUES (1,?1,?2,?3,?4,?5,?6,?7)",
            params![
                self.scenario.phase,
                self.scenario.home_after_uni.map(i64::from),
                self.scenario.plan_confirmed as i64,
                self.scenario.uni_extended as i64,
                self.scenario.replan_confirmed as i64,
                self.scenario.learning_result,
                self.scenario.irrelevant_color
            ],
        )?;
        for row in &self.change_events {
            transaction.execute(
                "INSERT INTO change_events VALUES (?1,?2,?3,?4,?5)",
                params![
                    row.id,
                    row.occurred_at,
                    row.kind,
                    row.summary,
                    row.payload_json
                ],
            )?;
        }
        for row in &self.tool_settings {
            transaction.execute(
                "INSERT INTO tool_settings VALUES (?1,?2,?3)",
                params![row.id, row.status, row.config_json],
            )?;
        }
        Ok(())
    }
}

fn query_all<T, F>(connection: &Connection, sql: &str, mapper: F) -> Result<Vec<T>>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
{
    let mut statement = connection.prepare(sql)?;
    Ok(statement
        .query_map([], mapper)?
        .collect::<std::result::Result<Vec<_>, _>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository() -> (tempfile::TempDir, Repository) {
        let directory = tempfile::tempdir().unwrap();
        let repository = Repository::open(directory.path()).unwrap();
        (directory, repository)
    }

    #[test]
    fn database_file_is_not_plain_sqlite() {
        let (_directory, repository) = repository();
        let bytes = fs::read(repository.database_path()).unwrap();
        assert!(!bytes.starts_with(b"SQLite format 3"));
    }

    #[test]
    fn seed_contains_all_five_types_and_relation() {
        let (_directory, repository) = repository();
        let studio = repository.studio(None).unwrap();
        let chairs = studio
            .collections
            .iter()
            .find(|entry| entry.id == CHAIR_COLLECTION)
            .unwrap();
        let kinds: HashSet<_> = chairs
            .properties
            .iter()
            .map(|entry| entry.kind.to_string())
            .collect();
        assert_eq!(
            kinds,
            HashSet::from([
                "text".into(),
                "number".into(),
                "date".into(),
                "choice".into(),
                "relation".into()
            ])
        );
        assert!(chairs.items.iter().any(|item| {
            item.values
                .iter()
                .any(|value| value.kind == PropertyKind::Relation)
        }));
    }

    #[test]
    fn unconfirmed_date_is_not_assistant_context() {
        let (_directory, repository) = repository();
        repository
            .connection
            .execute(
                "UPDATE property_definitions SET meaning_confirmed=0 WHERE id=?1",
                [CHAIR_DEADLINE],
            )
            .unwrap();
        assert!(repository.relevant_deadline().unwrap().is_none());
    }

    #[test]
    fn export_restore_roundtrip_preserves_semantics() {
        let (_directory, mut repository) = repository();
        repository.apply_scenario_action("home_by_13").unwrap();
        let before = repository.export_json().unwrap();
        repository.reset_demo().unwrap();
        repository.restore_json(&before).unwrap();
        let restored = repository.export_json().unwrap();
        let mut before_value: Value = serde_json::from_str(&before).unwrap();
        let mut restored_value: Value = serde_json::from_str(&restored).unwrap();
        before_value["exported_at"] = Value::Null;
        restored_value["exported_at"] = Value::Null;
        assert_eq!(before_value, restored_value);
    }

    #[test]
    fn invalid_item_rolls_back_with_its_event() {
        let (_directory, mut repository) = repository();
        let events = repository.history_count().unwrap();
        let result = repository.create_item(
            CHAIR_COLLECTION,
            "Fehler",
            vec![ItemValueDraft {
                property_id: CHAIR_PRICE.into(),
                value: "keine Zahl".into(),
            }],
        );
        assert!(result.is_err());
        let studio = repository.studio(Some(CHAIR_COLLECTION)).unwrap();
        let chairs = studio
            .collections
            .iter()
            .find(|entry| entry.id == CHAIR_COLLECTION)
            .unwrap();
        assert!(!chairs.items.iter().any(|item| item.title == "Fehler"));
        assert_eq!(repository.history_count().unwrap(), events);
    }

    #[test]
    fn rename_versions_definition_without_rewriting_old_value() {
        let (_directory, mut repository) = repository();
        repository.rename_property(CHAIR_PRICE, "Kosten").unwrap();
        let studio = repository.studio(Some(CHAIR_COLLECTION)).unwrap();
        let chairs = studio
            .collections
            .iter()
            .find(|entry| entry.id == CHAIR_COLLECTION)
            .unwrap();
        let current = chairs
            .properties
            .iter()
            .find(|entry| entry.logical_id == CHAIR_PRICE)
            .unwrap();
        assert_eq!(current.name, "Kosten");
        assert_eq!(current.version, 2);
        let historical = chairs.items[0]
            .values
            .iter()
            .find(|entry| entry.logical_id == CHAIR_PRICE)
            .unwrap();
        assert_eq!(historical.property_name, "Preis");
        assert_eq!(historical.property_version, 1);
    }
}
