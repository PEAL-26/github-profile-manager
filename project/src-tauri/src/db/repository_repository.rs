use crate::error::AppError;
use crate::models::profile::Profile;
use crate::models::repository::{Repository, ScanRoot};
use rusqlite::params;
use std::path::Path;
use uuid::Uuid;

/// Dados capturados pelo scanner para um repositório encontrado em disco.
pub struct ScannedRepo {
    pub path: String,
    pub name: String,
    pub remote_url: Option<String>,
    pub current_git_name: Option<String>,
    pub current_git_email: Option<String>,
    pub current_ssh_command: Option<String>,
    pub suggested_profile_id: Option<String>,
    pub suggestion_conflict: bool,
}

struct RawRepository {
    id: String,
    name: String,
    path: String,
    profile_id: Option<String>,
    current_git_name: Option<String>,
    current_git_email: Option<String>,
    current_ssh_command: Option<String>,
    remote_url: Option<String>,
    suggested_profile_id: Option<String>,
    suggestion_conflict: bool,
    last_scanned_at: String,
}

/// Validação de integridade (doc 04 §5): o repositório está "Sincronizado"
/// quando o e-mail local e o SSH configurados batem exatamente com o perfil
/// vinculado.
fn is_profile_valid(
    current_git_email: Option<&str>,
    current_ssh_command: Option<&str>,
    profile: &Profile,
    private_key_path: Option<&str>,
) -> bool {
    let email_ok = current_git_email
        .map(|e| e.trim().eq_ignore_ascii_case(profile.email.trim()))
        .unwrap_or(false);

    let ssh_ok = match (&profile.ssh_key_id, private_key_path) {
        // Perfil sem chave SSH: só o e-mail conta.
        (None, _) => true,
        (Some(_), None) => false, // chave vinculada já não existe
        (Some(_), Some(key_path)) => current_ssh_command
            .map(|cmd| cmd.contains(key_path))
            .unwrap_or(false),
    };

    email_ok && ssh_ok
}

pub struct RepositoryRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> RepositoryRepository<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    fn raw_by_path(&self, path: &str) -> Result<Option<RawRepository>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, profile_id, current_git_name, current_git_email, current_ssh_command, remote_url, suggested_profile_id, suggestion_conflict, last_scanned_at FROM repositories WHERE path = ?1",
        )?;
        let mut rows = stmt.query_map(params![path], row_to_raw)?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn get_by_id(&self, id: &str) -> Result<Repository, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, profile_id, current_git_name, current_git_email, current_ssh_command, remote_url, suggested_profile_id, suggestion_conflict, last_scanned_at FROM repositories WHERE id = ?1",
        )?;
        let raw = stmt
            .query_row(params![id], row_to_raw)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => AppError::ValidationError(
                    "Repositório não encontrado. Execute um novo scan.".to_string(),
                ),
                other => AppError::DatabaseError(other.to_string()),
            })?;

        self.enrich(vec![raw])?
            .pop()
            .ok_or_else(|| AppError::DatabaseError("repositório desapareceu".to_string()))
    }

    pub fn get_all(&self) -> Result<Vec<Repository>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, profile_id, current_git_name, current_git_email, current_ssh_command, remote_url, suggested_profile_id, suggestion_conflict, last_scanned_at FROM repositories ORDER BY name",
        )?;
        let raws = stmt
            .query_map([], row_to_raw)?
            .collect::<Result<Vec<_>, _>>()?;

        self.enrich(raws)
    }

    /// Junta os dados dos perfis (nome, validação, sugestões) às linhas brutas.
    fn enrich(&self, raws: Vec<RawRepository>) -> Result<Vec<Repository>, AppError> {
        let profiles = {
            let mut stmt = self.conn.prepare(
                "SELECT id, name, github_username, email, host_alias, ssh_key_id, default_directory, default_branch, description, avatar_url, is_active, created_at, updated_at FROM profiles",
            )?;
            // Vincular o iterator a uma variável para que o MappedRows
            // (que empresta `stmt`) seja descartado antes do stmt.
            let rows = stmt.query_map([], row_to_profile)?;
            rows.collect::<Result<Vec<_>, _>>()?
        };

        let key_paths = {
            let mut stmt = self
                .conn
                .prepare("SELECT id, private_key_path FROM ssh_keys")?;
            let mut map = std::collections::HashMap::new();
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            for row in rows {
                let (id, path) = row?;
                map.insert(id, path);
            }
            map
        };

        let by_id: std::collections::HashMap<&str, &Profile> = profiles
            .iter()
            .map(|p| (p.id.as_str(), p))
            .collect();

        Ok(raws
            .into_iter()
            .map(|raw| {
                let profile = raw.profile_id.as_deref().and_then(|id| by_id.get(id));
                let key_path = profile.and_then(|p| {
                    p.ssh_key_id
                        .as_ref()
                        .and_then(|kid| key_paths.get(kid).map(|s| s.as_str()))
                });
                let is_valid = match profile {
                    Some(p) => is_profile_valid(
                        raw.current_git_email.as_deref(),
                        raw.current_ssh_command.as_deref(),
                        p,
                        key_path,
                    ),
                    None => false,
                };

                Repository {
                    id: raw.id,
                    name: raw.name,
                    path: raw.path,
                    profile_id: raw.profile_id,
                    profile_name: profile.map(|p| p.name.clone()),
                    current_git_name: raw.current_git_name,
                    current_git_email: raw.current_git_email,
                    current_ssh_command: raw.current_ssh_command,
                    is_valid_profile: is_valid,
                    remote_url: raw.remote_url,
                    suggested_profile_id: raw.suggested_profile_id,
                    suggestion_conflict: raw.suggestion_conflict,
                    last_scanned_at: raw.last_scanned_at,
                }
            })
            .collect())
    }

    /// Insere ou atualiza um repositório encontrado no scan (upsert por path).
    pub fn upsert_scanned(&self, scanned: &ScannedRepo) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(existing) = self.raw_by_path(&scanned.path)? {
            self.conn.execute(
                "UPDATE repositories SET name = ?1, remote_url = ?2, current_git_name = ?3, current_git_email = ?4, current_ssh_command = ?5, suggested_profile_id = ?6, suggestion_conflict = ?7, last_scanned_at = ?8 WHERE id = ?9",
                params![
                    scanned.name,
                    scanned.remote_url,
                    scanned.current_git_name,
                    scanned.current_git_email,
                    scanned.current_ssh_command,
                    scanned.suggested_profile_id,
                    scanned.suggestion_conflict as i32,
                    now,
                    existing.id,
                ],
            )?;
        } else {
            self.conn.execute(
                "INSERT INTO repositories (id, name, path, profile_id, remote_url, current_git_name, current_git_email, current_ssh_command, suggested_profile_id, suggestion_conflict, last_scanned_at, created_at) VALUES (?1, ?2, ?3, NULL, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?10)",
                params![
                    Uuid::new_v4().to_string(),
                    scanned.name,
                    scanned.path,
                    scanned.remote_url,
                    scanned.current_git_name,
                    scanned.current_git_email,
                    scanned.current_ssh_command,
                    scanned.suggested_profile_id,
                    scanned.suggestion_conflict as i32,
                    now,
                ],
            )?;
        }

        Ok(())
    }

    /// Apaga registos cujo caminho já não existe em disco (a tabela reflete
    /// o disco — decisão alinhada com o utilizador).
    pub fn prune_missing(&self) -> Result<Vec<String>, AppError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, path FROM repositories")?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        drop(stmt);

        let mut removed = Vec::new();
        for (id, path) in rows {
            if !Path::new(&path).is_dir() {
                self.conn
                    .execute("DELETE FROM repositories WHERE id = ?1", params![id])?;
                removed.push(path);
            }
        }
        Ok(removed)
    }

    /// Regista a aplicação de um perfil ao repositório (injeção local feita
    /// pelo command antes de chamar este método).
    pub fn set_applied(
        &self,
        id: &str,
        profile_id: &str,
        git_name: Option<&str>,
        git_email: Option<&str>,
        ssh_command: Option<&str>,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        let affected = self.conn.execute(
            "UPDATE repositories SET profile_id = ?1, current_git_name = ?2, current_git_email = ?3, current_ssh_command = ?4, last_scanned_at = ?5 WHERE id = ?6",
            params![profile_id, git_name, git_email, ssh_command, now, id],
        )?;

        if affected == 0 {
            return Err(AppError::ValidationError(
                "Repositório não encontrado. Execute um novo scan.".to_string(),
            ));
        }
        Ok(())
    }

    // ---------- Pastas raiz do scan ----------

    pub fn get_roots(&self) -> Result<Vec<ScanRoot>, AppError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, path, created_at FROM scan_roots ORDER BY path")?;
        let roots = stmt
            .query_map([], |row| {
                Ok(ScanRoot {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(roots)
    }

    pub fn add_root(&self, path: &str) -> Result<ScanRoot, AppError> {
        let normalized = path.trim().trim_end_matches(['/', '\\']).to_string();
        if normalized.is_empty() {
            return Err(AppError::ValidationError(
                "Caminho da pasta inválido.".to_string(),
            ));
        }

        let exists: bool = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM scan_roots WHERE lower(path) = lower(?1)",
                params![normalized],
                |row| row.get::<_, i64>(0).map(|c| c > 0),
            )?;
        if exists {
            return Err(AppError::ValidationError(format!(
                "A pasta \"{}\" já está na lista de varrimento.",
                normalized
            )));
        }

        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO scan_roots (id, path, created_at) VALUES (?1, ?2, ?3)",
            params![id, normalized, now],
        )?;

        Ok(ScanRoot {
            id,
            path: normalized,
            created_at: now,
        })
    }

    pub fn remove_root(&self, id: &str) -> Result<(), AppError> {
        let affected = self
            .conn
            .execute("DELETE FROM scan_roots WHERE id = ?1", params![id])?;
        if affected == 0 {
            return Err(AppError::ValidationError(
                "Pasta de varrimento não encontrada.".to_string(),
            ));
        }
        Ok(())
    }
}

fn row_to_raw(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawRepository> {
    Ok(RawRepository {
        id: row.get(0)?,
        name: row.get(1)?,
        path: row.get(2)?,
        profile_id: row.get(3)?,
        current_git_name: row.get(4)?,
        current_git_email: row.get(5)?,
        current_ssh_command: row.get(6)?,
        remote_url: row.get(7)?,
        suggested_profile_id: row.get(8)?,
        suggestion_conflict: row.get::<_, i32>(9)? == 1,
        last_scanned_at: row.get(10)?,
    })
}

fn row_to_profile(row: &rusqlite::Row<'_>) -> rusqlite::Result<Profile> {
    Ok(Profile {
        id: row.get(0)?,
        name: row.get(1)?,
        github_username: row.get(2)?,
        email: row.get(3)?,
        host_alias: row.get(4)?,
        ssh_key_id: row.get(5)?,
        default_directory: row.get(6)?,
        default_branch: row.get(7)?,
        description: row.get(8)?,
        avatar_url: row.get(9)?,
        is_active: row.get::<_, i32>(10)? == 1,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_schema;
    use crate::models::profile::CreateProfileDto;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_schema(&conn).unwrap();
        conn
    }

    use rusqlite::Connection;

    fn profile_dto(name: &str, email: &str) -> CreateProfileDto {
        CreateProfileDto {
            name: name.to_string(),
            github_username: format!("{}-gh", name),
            email: email.to_string(),
            host_alias: format!("alias-{}", name),
            ssh_key_id: None,
            default_directory: None,
            default_branch: "main".to_string(),
            description: None,
            pat_token: None,
        }
    }

    fn scanned(path: &str, email: Option<&str>) -> ScannedRepo {
        ScannedRepo {
            path: path.to_string(),
            name: Path::new(path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string()),
            remote_url: Some("git@github.com:user/repo.git".to_string()),
            current_git_name: Some("User".to_string()),
            current_git_email: email.map(|e| e.to_string()),
            current_ssh_command: None,
            suggested_profile_id: None,
            suggestion_conflict: false,
        }
    }

    #[test]
    fn test_upsert_is_idempotent_by_path() {
        let conn = setup_db();
        let repo = RepositoryRepository::new(&conn);

        // Caminho inexistente em disco ainda é inserido (prune trata depois).
        repo.upsert_scanned(&scanned("/tmp/ghpm-repo-x/proj-a", Some("a@x.com")))
            .unwrap();
        repo.upsert_scanned(&scanned("/tmp/ghpm-repo-x/proj-a", Some("novo@x.com")))
            .unwrap();

        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 1, "upsert não deve duplicar por path");
        assert_eq!(all[0].current_git_email.as_deref(), Some("novo@x.com"));
    }

    #[test]
    fn test_prune_removes_missing_paths_only() {
        let conn = setup_db();
        let repo = RepositoryRepository::new(&conn);

        let alive = std::env::temp_dir().join(format!("ghpm_alive_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&alive).unwrap();
        let alive_str = alive.to_string_lossy().to_string();
        let dead = format!(
            "{}/ghpm_dead_{}",
            std::env::temp_dir().to_string_lossy(),
            Uuid::new_v4()
        );

        repo.upsert_scanned(&scanned(&alive_str, None)).unwrap();
        repo.upsert_scanned(&scanned(&dead, None)).unwrap();

        let removed = repo.prune_missing().unwrap();
        assert_eq!(removed, vec![dead.clone()]);

        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].path, alive_str);

        std::fs::remove_dir_all(&alive).ok();
    }

    #[test]
    fn test_status_validation_synced_vs_divergent_vs_unmapped() {
        let conn = setup_db();
        let profiles = crate::db::profile_repository::ProfileRepository::new(&conn);
        let work = profiles.create(&profile_dto("Work", "work@corp.com")).unwrap();

        let repo = RepositoryRepository::new(&conn);
        let path = "/tmp/ghpm-status-repo";

        // Não mapeado
        repo.upsert_scanned(&scanned(path, Some("work@corp.com"))).unwrap();
        let unlinked = repo.get_all().unwrap().pop().unwrap();
        assert!(!unlinked.is_valid_profile);
        assert!(unlinked.profile_id.is_none());

        // Sincronizado: vinculado + e-mail igual (perfil sem chave SSH)
        repo.set_applied(&unlinked.id, &work.id, Some("Work"), Some("work@corp.com"), None)
            .unwrap();
        let synced = repo.get_all().unwrap().pop().unwrap();
        assert!(synced.is_valid_profile);
        assert_eq!(synced.profile_name.as_deref(), Some("Work"));

        // Divergente: e-mail local difere do perfil vinculado
        repo.set_applied(&unlinked.id, &work.id, Some("Work"), Some("outro@x.com"), None)
            .unwrap();
        let divergent = repo.get_all().unwrap().pop().unwrap();
        assert!(!divergent.is_valid_profile);
        assert!(divergent.profile_id.is_some());
    }

    #[test]
    fn test_add_root_rejects_duplicates_case_insensitive() {
        let conn = setup_db();
        let repo = RepositoryRepository::new(&conn);

        repo.add_root("C:/dev").unwrap();
        let err = repo
            .add_root("c:/dev/")
            .expect_err("duplicado deve falhar");

        assert!(matches!(err, AppError::ValidationError(_)));
        assert_eq!(repo.get_roots().unwrap().len(), 1);
    }

    #[test]
    fn test_remove_root() {
        let conn = setup_db();
        let repo = RepositoryRepository::new(&conn);
        let root = repo.add_root("/tmp/dev").unwrap();

        repo.remove_root(&root.id).unwrap();
        assert!(repo.get_roots().unwrap().is_empty());
        assert!(repo.remove_root(&root.id).is_err());
    }
}
