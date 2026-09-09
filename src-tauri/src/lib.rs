mod application;
mod desktop;
mod domain;
mod infrastructure;
#[allow(dead_code)]
mod tools;

use application::AppService;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path()
                .app_local_data_dir()
                .map_err(|error| Box::<dyn std::error::Error>::from(error.to_string()))?;
            let service = AppService::open(&data_dir)?;
            app.manage(Mutex::new(service));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            desktop::load_briefing,
            desktop::load_workspace,
            desktop::respond_to_briefing,
            desktop::create_collection,
            desktop::create_item,
            desktop::rename_property,
            desktop::select_studio_collection,
            desktop::export_data,
            desktop::restore_data,
            desktop::reset_demo,
        ])
        .run(tauri::generate_context!())
        .expect("Momentum konnte nicht gestartet werden");
}
