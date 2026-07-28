use crate::error::AppError;
use crate::utils::git;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_global_git_identity(
    _state: State<'_, AppState>,
) -> Result<(String, String), AppError> {
    git::get_global_git_identity()
}

#[tauri::command]
pub async fn set_global_git_identity(
    _state: State<'_, AppState>,
    name: String,
    email: String,
    ssh_key_path: Option<String>,
) -> Result<(), AppError> {
    git::set_global_git_identity(&name, &email, ssh_key_path.as_deref())
}
