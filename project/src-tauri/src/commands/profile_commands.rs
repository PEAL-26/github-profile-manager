use crate::db::activity_logger::ActivityLogger;
use crate::db::profile_repository::ProfileRepository;
use crate::db::ssh_key_repository::SshKeyRepository;
use crate::error::AppError;
use crate::models::profile::{CreateProfileDto, Profile};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_all_profiles(state: State<'_, AppState>) -> Result<Vec<Profile>, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = ProfileRepository::new(&conn);
    repo.get_all()
}

#[tauri::command]
pub async fn get_profile_by_id(
    state: State<'_, AppState>,
    id: String,
) -> Result<Profile, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = ProfileRepository::new(&conn);
    repo.get_by_id(&id)
}

#[tauri::command]
pub async fn create_profile(
    state: State<'_, AppState>,
    payload: CreateProfileDto,
) -> Result<Profile, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = ProfileRepository::new(&conn);
    let profile = repo.create(&payload)?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.info("profile", &format!("Created profile: {}", profile.name));

    Ok(profile)
}

#[tauri::command]
pub async fn update_profile(
    state: State<'_, AppState>,
    id: String,
    payload: CreateProfileDto,
) -> Result<Profile, AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = ProfileRepository::new(&conn);
    let profile = repo.update(&id, &payload)?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.info("profile", &format!("Updated profile: {}", profile.name));

    Ok(profile)
}

#[tauri::command]
pub async fn delete_profile(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = ProfileRepository::new(&conn);
    let profile = repo.get_by_id(&id)?;
    repo.delete(&id)?;

    let logger = ActivityLogger::new(&conn);
    let _ = logger.warn("profile", &format!("Deleted profile: {}", profile.name));

    Ok(())
}

#[tauri::command]
pub async fn set_active_profile(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let conn = state.database.conn.lock().unwrap();
    let repo = ProfileRepository::new(&conn);
    repo.set_active(&id)?;

    let profile = repo.get_by_id(&id)?;

    let ssh_key_path = profile
        .ssh_key_id
        .as_ref()
        .and_then(|key_id| {
            let key_repo = SshKeyRepository::new(&conn);
            key_repo.get_by_id(key_id).ok().map(|k| k.private_key_path)
        });

    let logger = ActivityLogger::new(&conn);
    let _ = logger.log_profile_switch(&profile.name);

    drop(conn);

    crate::utils::git::set_global_git_identity(
        &profile.name,
        &profile.email,
        ssh_key_path.as_deref(),
    )?;

    Ok(())
}
