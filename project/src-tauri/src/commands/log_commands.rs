use crate::utils::error_log;

/// Permite que o frontend registre erros de validação e falhas de UI
/// no mesmo ficheiro ghpm_errors.txt usado pelo backend.
#[tauri::command]
pub fn log_frontend_error(message: String) {
    error_log::log_message("frontend", &message);
}
