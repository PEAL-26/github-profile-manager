use crate::db::activity_logger::ActivityLogger;
use crate::db::ssh_key_repository::SshKeyRepository;
use crate::error::AppError;
use crate::models::ssh_key::{SshKey, SshTestResultDto};
use crate::utils::ssh;
use crate::utils::error_log;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_ssh_keys(state: State<'_, AppState>) -> Result<Vec<SshKey>, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = SshKeyRepository::new(&conn);
    error_log::guard("list_ssh_keys", repo.get_all())
}

#[tauri::command]
pub async fn generate_ssh_key(
    state: State<'_, AppState>,
    name: String,
    key_type: String,
    passphrase: Option<String>,
) -> Result<SshKey, AppError> {
    let key_path = error_log::guard(
        "generate_ssh_key",
        ssh::generate_ssh_key(&name, &key_type, passphrase.as_deref()),
    )?;
    let pub_path = error_log::guard("generate_ssh_key", ssh::get_public_key_path(&name, &key_type))?;
    let fingerprint = error_log::guard("generate_ssh_key", ssh::get_fingerprint(&key_path))?;

    let conn = state.database.conn.lock().unwrap();
    let repo = SshKeyRepository::new(&conn);
    let key = error_log::guard(
        "generate_ssh_key",
        repo.create(
            &name,
            &key_type,
            &pub_path.to_string_lossy(),
            &key_path,
            &fingerprint,
            passphrase.is_some(),
        ),
    )?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.log_key_generated(&key.name);

    Ok(key)
}

#[tauri::command]
pub async fn delete_ssh_key(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = SshKeyRepository::new(&conn);
    let key = error_log::guard("delete_ssh_key", repo.get_by_id(&id))?;

    let private_path = std::path::PathBuf::from(&key.private_key_path);
    let pub_path = std::path::PathBuf::from(&key.public_key_path);

    if private_path.exists() {
        std::fs::remove_file(&private_path).ok();
    }
    if pub_path.exists() {
        std::fs::remove_file(&pub_path).ok();
    }

    error_log::guard("delete_ssh_key", repo.delete(&id))?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.log_key_deleted(&key.name);

    Ok(())
}

#[tauri::command]
pub async fn test_ssh_connection(host_alias: String) -> Result<SshTestResultDto, AppError> {
    let (success, username, raw_output, latency_ms) =
        error_log::guard("test_ssh_connection", ssh::test_ssh_connection(&host_alias))?;

    Ok(SshTestResultDto {
        success,
        username,
        raw_output,
        latency_ms,
    })
}

#[tauri::command]
pub async fn read_ssh_key_content(path: String) -> Result<String, AppError> {
    error_log::guard(
        "read_ssh_key_content",
        std::fs::read_to_string(&path)
            .map_err(|e| AppError::SystemError(format!("Falha ao ler chave: {}", e))),
    )
}
