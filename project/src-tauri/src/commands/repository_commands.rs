use crate::db::activity_logger::ActivityLogger;
use crate::db::profile_repository::ProfileRepository;
use crate::db::repository_repository::{RepositoryRepository, ScannedRepo};
use crate::db::settings_repository::SettingsRepository;
use crate::db::ssh_key_repository::SshKeyRepository;
use crate::error::AppError;
use crate::models::repository::{Repository, ScanRoot};
use crate::utils::{error_log, scanner};
use crate::AppState;
use std::path::{Path, PathBuf};
use tauri::State;

#[tauri::command]
pub async fn list_repositories(state: State<'_, AppState>) -> Result<Vec<Repository>, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = RepositoryRepository::new(&conn);
    error_log::guard("list_repositories", repo.get_all())
}

#[tauri::command]
pub async fn list_scan_roots(state: State<'_, AppState>) -> Result<Vec<ScanRoot>, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = RepositoryRepository::new(&conn);
    error_log::guard("list_scan_roots", repo.get_roots())
}

#[tauri::command]
pub async fn add_scan_root(
    state: State<'_, AppState>,
    path: String,
) -> Result<ScanRoot, AppError> {
    let trimmed = path.trim().to_string();
    let as_path = PathBuf::from(&trimmed);

    if !as_path.is_dir() {
        return error_log::guard(
            "add_scan_root",
            Err(AppError::ValidationError(format!(
                "A pasta \"{}\" não existe ou não é um diretório.",
                trimmed
            ))),
        );
    }

    let conn = state.database.conn.lock().unwrap();
    let repo = RepositoryRepository::new(&conn);
    error_log::guard("add_scan_root", repo.add_root(&trimmed))
}

#[tauri::command]
pub async fn remove_scan_root(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = RepositoryRepository::new(&conn);
    error_log::guard("remove_scan_root", repo.remove_root(&id))
}

/// Varre todas as pastas raiz guardadas à procura de repositórios Git,
/// extrai metadados (remote, identidade local), deteta o perfil sugerido,
/// faz upsert e remove registos cujo caminho já não existe.
#[tauri::command]
pub async fn scan_repositories(state: State<'_, AppState>) -> Result<Vec<Repository>, AppError> {
    let conn = state.database.conn.lock().unwrap();

    let repositories = RepositoryRepository::new(&conn);
    let roots = error_log::guard("scan_repositories", repositories.get_roots())?;
    if roots.is_empty() {
        return error_log::guard(
            "scan_repositories",
            Err(AppError::ValidationError(
                "Adicione uma pasta raiz para escanear.".to_string(),
            )),
        );
    }

    let settings =
        error_log::guard("scan_repositories", SettingsRepository::new(&conn).get())?;
    let profiles = error_log::guard("scan_repositories", ProfileRepository::new(&conn).get_all())?;

    let root_paths: Vec<PathBuf> = roots.iter().map(|r| PathBuf::from(&r.path)).collect();
    let found = scanner::scan_roots(&root_paths, settings.scan_depth);

    let mut found_count = 0usize;
    for repo in &found {
        let metadata = scanner::read_metadata(&repo.path);
        let detection = scanner::detect_profile(
            metadata.git_email.as_deref(),
            metadata.remote_url.as_deref(),
            &profiles,
        );

        let scanned = ScannedRepo {
            path: repo.path.to_string_lossy().to_string(),
            name: repo.name.clone(),
            remote_url: metadata.remote_url,
            current_git_name: metadata.git_name,
            current_git_email: metadata.git_email,
            current_ssh_command: metadata.ssh_command,
            suggested_profile_id: detection.suggested_profile_id,
            suggestion_conflict: detection.conflict,
        };

        error_log::guard("scan_repositories", repositories.upsert_scanned(&scanned))?;
        found_count += 1;
    }

    let removed = error_log::guard("scan_repositories", repositories.prune_missing())?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.info(
        "repository",
        &format!(
            "Scan: {} repositório(s) encontrado(s), {} registo(s) obsoleto(s) removido(s)",
            found_count,
            removed.len()
        ),
    );

    error_log::guard("scan_repositories", repositories.get_all())
}

/// Estratégia 1 do doc 04 §4.1: escreve `user.name`, `user.email` e
/// `core.sshCommand` no `.git/config` do repositório e ativa o perfil
/// globalmente (identidade global + bandeja), tal como o fluxo normal de
/// troca de perfil.
#[tauri::command]
pub async fn apply_profile_to_repository(
    state: State<'_, AppState>,
    repository_id: String,
    profile_id: String,
) -> Result<Repository, AppError> {
    let conn = state.database.conn.lock().unwrap();

    let repositories = RepositoryRepository::new(&conn);
    let profiles = ProfileRepository::new(&conn);

    let repository =
        error_log::guard("apply_profile_to_repository", repositories.get_by_id(&repository_id))?;
    let profile =
        error_log::guard("apply_profile_to_repository", profiles.get_by_id(&profile_id))?;

    let ssh_key_path = profile.ssh_key_id.as_ref().and_then(|key_id| {
        SshKeyRepository::new(&conn)
            .get_by_id(key_id)
            .ok()
            .map(|k| k.private_key_path)
    });
    let ssh_command = ssh_key_path
        .as_ref()
        .map(|p| format!("ssh -i \"{}\"", p));

    // 1) Injeção local no repositório (falha aqui interrompe SEM trocar o global)
    error_log::guard(
        "apply_profile_to_repository",
        scanner::inject_local_identity(
            Path::new(&repository.path),
            &profile.name,
            &profile.email,
            ssh_key_path.as_deref(),
        ),
    )?;

    // 2) Regista a aplicação na tabela repositories
    error_log::guard(
        "apply_profile_to_repository",
        repositories.set_applied(
            &repository_id,
            &profile_id,
            Some(&profile.name),
            Some(&profile.email),
            ssh_command.as_deref(),
        ),
    )?;

    // 3) Ativa o perfil globalmente (mesmo fluxo de set_active_profile)
    error_log::guard("apply_profile_to_repository", profiles.set_active(&profile_id))?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.log_profile_switch(&profile.name);
    let _ = logger.info(
        "repository",
        &format!(
            "Perfil \"{}\" aplicado ao repositório {}",
            profile.name, repository.name
        ),
    );

    drop(conn);

    error_log::guard(
        "apply_profile_to_repository",
        crate::utils::git::set_global_git_identity(
            &profile.name,
            &profile.email,
            ssh_key_path.as_deref(),
        ),
    )?;

    let conn = state.database.conn.lock().unwrap();
    error_log::guard(
        "apply_profile_to_repository",
        RepositoryRepository::new(&conn).get_by_id(&repository_id),
    )
}
