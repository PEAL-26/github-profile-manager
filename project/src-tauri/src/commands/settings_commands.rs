use crate::db::activity_logger::{ActivityLog, ActivityLogger};
use crate::db::settings_repository::{AppSettings, SettingsRepository};
use crate::error::AppError;
use crate::utils::error_log;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = SettingsRepository::new(&conn);
    error_log::guard("get_settings", repo.get())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = SettingsRepository::new(&conn);
    error_log::guard("update_settings", repo.save(&settings))
}

#[tauri::command]
pub async fn get_activity_logs(state: State<'_, AppState>) -> Result<Vec<ActivityLog>, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let logger = ActivityLogger::new(&conn);
    error_log::guard("get_activity_logs", logger.get_all())
}
