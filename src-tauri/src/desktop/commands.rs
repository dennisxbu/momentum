use crate::{application::AppService, domain::*};
use std::sync::Mutex;
use tauri::State;

type CommandResult<T> = std::result::Result<T, String>;

fn with_service<T>(
    state: State<'_, Mutex<AppService>>,
    operation: impl FnOnce(&mut AppService) -> Result<T>,
) -> CommandResult<T> {
    let mut service = state
        .lock()
        .map_err(|_| "Momentum konnte den lokalen Zustand nicht sperren.".to_string())?;
    operation(&mut service).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_briefing(state: State<'_, Mutex<AppService>>) -> CommandResult<BriefingView> {
    with_service(state, |service| service.briefing())
}

#[tauri::command]
pub fn load_workspace(state: State<'_, Mutex<AppService>>) -> CommandResult<WorkspaceView> {
    with_service(state, |service| service.workspace())
}

#[tauri::command]
pub fn respond_to_briefing(
    state: State<'_, Mutex<AppService>>,
    action: String,
) -> CommandResult<WorkspaceView> {
    with_service(state, |service| service.respond(&action))
}

#[tauri::command]
pub fn create_collection(
    state: State<'_, Mutex<AppService>>,
    draft: CollectionDraft,
) -> CommandResult<WorkspaceView> {
    with_service(state, |service| service.create_collection(draft))
}

#[tauri::command]
pub fn create_item(
    state: State<'_, Mutex<AppService>>,
    collection_id: String,
    title: String,
    values: Vec<ItemValueDraft>,
) -> CommandResult<WorkspaceView> {
    with_service(state, |service| {
        service.create_item(&collection_id, &title, values)
    })
}

#[tauri::command]
pub fn rename_property(
    state: State<'_, Mutex<AppService>>,
    property_id: String,
    new_name: String,
) -> CommandResult<WorkspaceView> {
    with_service(state, |service| {
        service.rename_property(&property_id, &new_name)
    })
}

#[tauri::command]
pub fn select_studio_collection(
    state: State<'_, Mutex<AppService>>,
    collection_id: String,
) -> CommandResult<WorkspaceView> {
    with_service(state, |service| service.select_collection(&collection_id))
}

#[tauri::command]
pub fn export_data(state: State<'_, Mutex<AppService>>) -> CommandResult<Option<String>> {
    let Some(path) = rfd::FileDialog::new()
        .set_title("Momentum-Export speichern")
        .set_file_name("momentum-export.json")
        .add_filter("Momentum JSON", &["json"])
        .save_file()
    else {
        return Ok(None);
    };
    with_service(state, |service| {
        service
            .export_to(&path)
            .map(|saved| Some(saved.display().to_string()))
    })
}

#[tauri::command]
pub fn restore_data(state: State<'_, Mutex<AppService>>) -> CommandResult<Option<WorkspaceView>> {
    let Some(path) = rfd::FileDialog::new()
        .set_title("Momentum-Export wiederherstellen")
        .add_filter("Momentum JSON", &["json"])
        .pick_file()
    else {
        return Ok(None);
    };
    with_service(state, |service| service.restore_from(&path).map(Some))
}

#[tauri::command]
pub fn reset_demo(state: State<'_, Mutex<AppService>>) -> CommandResult<WorkspaceView> {
    with_service(state, |service| service.reset_demo())
}
