use super::briefing::{LocalReasoner, Reasoner};
use crate::{domain::*, infrastructure::Repository};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct AppService {
    repository: Repository,
    reasoner: Box<dyn Reasoner>,
    selected_collection: Option<String>,
}

impl AppService {
    pub fn open(data_dir: &Path) -> Result<Self> {
        Ok(Self {
            repository: Repository::open(data_dir)?,
            reasoner: Box::<LocalReasoner>::default(),
            selected_collection: None,
        })
    }

    pub fn briefing(&self) -> Result<BriefingView> {
        Ok(self.reasoner.prepare(
            &self.repository.scenario()?,
            self.repository.relevant_deadline()?.as_ref(),
        ))
    }

    pub fn workspace(&self) -> Result<WorkspaceView> {
        Ok(WorkspaceView {
            briefing: self.briefing()?,
            studio: self
                .repository
                .studio(self.selected_collection.as_deref())?,
            history_count: self.repository.history_count()?,
            storage_label: self.repository.database_path().display().to_string(),
            encrypted: true,
        })
    }

    pub fn respond(&mut self, action: &str) -> Result<WorkspaceView> {
        if action == "restart_walkthrough" {
            self.repository.reset_demo()?;
            self.selected_collection = None;
            return self.workspace();
        }
        let phase = self.repository.scenario()?.phase;
        let allowed = match phase.as_str() {
            "morning_unknown" => ["home_by_13", "away_after_uni"].as_slice(),
            "morning_answered" => ["confirm_plan", "irrelevant_color"].as_slice(),
            "plan_confirmed" => ["uni_extended", "to_evening", "irrelevant_color"].as_slice(),
            "day_changed" => ["confirm_replan"].as_slice(),
            "replan_confirmed" => ["to_evening"].as_slice(),
            "evening" => ["learning_done", "learning_unknown"].as_slice(),
            _ => [].as_slice(),
        };
        if !allowed.contains(&action) {
            return Err(MomentumError::Validation("Diese Antwort passt nicht mehr zum aktuellen Stand. Das Briefing wurde nicht verändert.".into()));
        }
        self.repository.apply_scenario_action(action)?;
        self.workspace()
    }

    pub fn create_collection(&mut self, draft: CollectionDraft) -> Result<WorkspaceView> {
        let id = self.repository.create_collection(draft)?;
        self.selected_collection = Some(id);
        self.workspace()
    }

    pub fn create_item(
        &mut self,
        collection_id: &str,
        title: &str,
        values: Vec<ItemValueDraft>,
    ) -> Result<WorkspaceView> {
        self.repository.create_item(collection_id, title, values)?;
        self.selected_collection = Some(collection_id.into());
        self.workspace()
    }

    pub fn rename_property(&mut self, property_id: &str, new_name: &str) -> Result<WorkspaceView> {
        self.repository.rename_property(property_id, new_name)?;
        self.workspace()
    }

    pub fn select_collection(&mut self, collection_id: &str) -> Result<WorkspaceView> {
        self.selected_collection = Some(collection_id.into());
        self.workspace()
    }

    pub fn export_to(&self, path: &Path) -> Result<PathBuf> {
        let path = if path.extension().is_none() {
            path.with_extension("json")
        } else {
            path.to_path_buf()
        };
        fs::write(&path, self.repository.export_json()?)?;
        Ok(path)
    }

    pub fn restore_from(&mut self, path: &Path) -> Result<WorkspaceView> {
        let source = fs::read_to_string(path)?;
        self.repository.restore_json(&source)?;
        self.selected_collection = None;
        self.workspace()
    }

    pub fn reset_demo(&mut self) -> Result<WorkspaceView> {
        self.repository.reset_demo()?;
        self.selected_collection = None;
        self.workspace()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposal_does_not_write_before_confirmation() {
        let directory = tempfile::tempdir().unwrap();
        let mut service = AppService::open(directory.path()).unwrap();
        let before = service.repository.history_count().unwrap();
        let initial = service.briefing().unwrap();
        assert!(
            initial
                .actions
                .iter()
                .all(|action| action.id != "confirm_plan")
        );
        assert_eq!(service.repository.history_count().unwrap(), before);
        service.respond("home_by_13").unwrap();
        let proposed = service.briefing().unwrap();
        assert!(
            proposed
                .actions
                .iter()
                .any(|action| action.id == "confirm_plan")
        );
        assert!(!service.repository.scenario().unwrap().plan_confirmed);
        service.respond("confirm_plan").unwrap();
        assert!(service.repository.scenario().unwrap().plan_confirmed);
    }

    #[test]
    fn full_flow_reaches_next_day_and_keeps_unknown_honest() {
        let directory = tempfile::tempdir().unwrap();
        let mut service = AppService::open(directory.path()).unwrap();
        for action in [
            "home_by_13",
            "confirm_plan",
            "uni_extended",
            "confirm_replan",
            "to_evening",
            "learning_unknown",
        ] {
            service.respond(action).unwrap();
        }
        let next = service.briefing().unwrap();
        assert_eq!(next.phase, "next_day");
        assert!(next.meaning.contains("keine Lernwirkung behaupten"));
        assert!(
            next.known
                .iter()
                .any(|fact| fact.label == "Lernblock" && fact.state == "unbekannt")
        );
    }

    #[test]
    fn restart_persists_confirmed_state() {
        let directory = tempfile::tempdir().unwrap();
        {
            let mut service = AppService::open(directory.path()).unwrap();
            service.respond("home_by_13").unwrap();
            service.respond("confirm_plan").unwrap();
        }
        let service = AppService::open(directory.path()).unwrap();
        assert!(service.repository.scenario().unwrap().plan_confirmed);
    }

    #[test]
    fn file_export_and_restore_preserve_the_confirmed_state() {
        let directory = tempfile::tempdir().unwrap();
        let export_path = directory.path().join("momentum-k6-export.json");
        let mut service = AppService::open(directory.path()).unwrap();
        service.respond("home_by_13").unwrap();
        service.respond("confirm_plan").unwrap();

        service.export_to(&export_path).unwrap();
        service.reset_demo().unwrap();
        assert!(!service.repository.scenario().unwrap().plan_confirmed);

        service.restore_from(&export_path).unwrap();
        assert!(service.repository.scenario().unwrap().plan_confirmed);
        assert!(export_path.is_file());
    }

    #[test]
    fn invalid_restore_file_does_not_replace_local_data() {
        let directory = tempfile::tempdir().unwrap();
        let invalid_path = directory.path().join("invalid.json");
        let mut service = AppService::open(directory.path()).unwrap();
        service.respond("home_by_13").unwrap();
        service.respond("confirm_plan").unwrap();
        fs::write(&invalid_path, "{not valid momentum data}").unwrap();

        assert!(service.restore_from(&invalid_path).is_err());
        assert!(service.repository.scenario().unwrap().plan_confirmed);
    }
}
