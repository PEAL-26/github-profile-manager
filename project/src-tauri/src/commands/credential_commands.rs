use crate::error::AppError;
use crate::utils::credentials::CredentialManager;
use crate::utils::error_log;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn save_pat(
    _state: State<'_, AppState>,
    profile_id: String,
    token: String,
) -> Result<(), AppError> {
    error_log::guard("save_pat", CredentialManager::save_pat(&profile_id, &token))
}

#[tauri::command]
pub async fn get_pat(
    _state: State<'_, AppState>,
    profile_id: String,
) -> Result<Option<String>, AppError> {
    error_log::guard("get_pat", CredentialManager::get_pat(&profile_id))
}

#[tauri::command]
pub async fn delete_pat(
    _state: State<'_, AppState>,
    profile_id: String,
) -> Result<(), AppError> {
    error_log::guard("delete_pat", CredentialManager::delete_pat(&profile_id))
}
