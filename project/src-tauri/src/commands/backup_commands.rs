use crate::db::activity_logger::ActivityLogger;
use crate::error::AppError;
use crate::utils::backup::{self, ImportReport};
use crate::utils::error_log;
use crate::AppState;
use serde::Serialize;
use std::fs;
use tauri::State;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupInfo {
    pub encrypted: bool,
    pub version: String,
    pub exported_at: Option<String>,
    pub profile_count: Option<usize>,
}

/// Inspeciona um ficheiro de backup sem aplicar alterações.
#[tauri::command]
pub async fn get_backup_info(
    _state: State<'_, AppState>,
    path: String,
) -> Result<BackupInfo, AppError> {
    let content = error_log::guard(
        "get_backup_info",
        fs::read_to_string(&path)
            .map_err(|e| AppError::SystemError(format!("Falha ao ler o ficheiro: {}", e))),
    )?;

    if backup::is_encrypted(&content) {
        return Ok(BackupInfo {
            encrypted: true,
            version: backup::ENCRYPTED_HEADER.to_string(),
            exported_at: None,
            profile_count: None,
        });
    }

    let parsed: backup::BackupFile = serde_json::from_str(&content).map_err(|e| {
        AppError::ValidationError(format!(
            "O ficheiro não parece um backup GHPM válido: {}",
            e
        ))
    })?;

    Ok(BackupInfo {
        encrypted: false,
        version: parsed.version,
        exported_at: Some(parsed.exported_at),
        profile_count: Some(parsed.profiles.len()),
    })
}

/// Exporta os perfis/chaves/configurações para o caminho indicado.
/// `include_secrets` (chaves privadas + PATs) exige `password` e gera
/// ficheiro cifrado com AES-256-GCM (GHPM_ENC_V2).
#[tauri::command]
pub async fn export_backup(
    state: State<'_, AppState>,
    path: String,
    include_secrets: bool,
    password: Option<String>,
) -> Result<usize, AppError> {
    let conn = state.database.conn.lock().unwrap();

    let (content, count) = error_log::guard(
        "export_backup",
        backup::export_to_string(&conn, include_secrets, password.as_deref()),
    )?;

    error_log::guard(
        "export_backup",
        fs::write(&path, &content)
            .map_err(|e| AppError::SystemError(format!("Falha ao gravar o backup: {}", e))),
    )?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.log(
        "INFO",
        "BACKUP_EXPORT",
        &format!(
            "Exported backup ({} profile(s){})",
            count,
            if include_secrets { ", encrypted" } else { "" }
        ),
        None,
    );

    Ok(count)
}

/// Importa/restaura um backup exportado previamente.
/// Retorna um resumo do que foi criado/atualizado.
#[tauri::command]
pub async fn import_backup(
    state: State<'_, AppState>,
    path: String,
    password: Option<String>,
) -> Result<ImportReport, AppError> {
    let content = error_log::guard(
        "import_backup",
        fs::read_to_string(&path)
            .map_err(|e| AppError::SystemError(format!("Falha ao ler o ficheiro: {}", e))),
    )?;

    let conn = state.database.conn.lock().unwrap();
    let report = error_log::guard(
        "import_backup",
        backup::import_from_string(&conn, &content, password.as_deref()),
    )?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.log(
        "INFO",
        "BACKUP_RESTORE",
        &format!(
            "Restored backup: {} created, {} updated, {} key(s)",
            report.profiles_created, report.profiles_updated, report.keys_restored
        ),
        None,
    );

    Ok(report)
}
