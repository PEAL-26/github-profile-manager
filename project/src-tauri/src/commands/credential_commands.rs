use crate::error::AppError;
use crate::utils::credentials::CredentialManager;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn save_pat(
    _state: State<'_, AppState>,
    profile_id: String,
    token: String,
) -> Result<(), AppError> {
    CredentialManager::save_pat(&profile_id, &token)
}

#[tauri::command]
pub async fn get_pat(
    _state: State<'_, AppState>,
    profile_id: String,
) -> Result<Option<String>, AppError> {
    CredentialManager::get_pat(&profile_id)
}

#[tauri::command]
pub async fn delete_pat(
    _state: State<'_, AppState>,
    profile_id: String,
) -> Result<(), AppError> {
    CredentialManager::delete_pat(&profile_id)
}
