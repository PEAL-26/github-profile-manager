use crate::db::activity_logger::{ActivityLog, ActivityLogger};
use crate::db::settings_repository::{AppSettings, SettingsRepository};
use crate::error::AppError;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = SettingsRepository::new(&conn);
    repo.get()
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = SettingsRepository::new(&conn);
    repo.save(&settings)
}

#[tauri::command]
pub async fn get_activity_logs(state: State<'_, AppState>) -> Result<Vec<ActivityLog>, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let logger = ActivityLogger::new(&conn);
    logger.get_all()
}
