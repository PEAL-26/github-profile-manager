use crate::error::AppError;
use crate::models::profile::{CreateProfileDto, Profile};
use rusqlite::params;
use uuid::Uuid;

/// Converte erros de violação de restrição do SQLite em mensagens amigáveis.
/// O caso mais comum é host_alias duplicado (UNIQUE), que falhava silenciosamente
/// na UI porque o erro genérico de banco não era exibido ao usuário.
fn map_constraint_error(err: rusqlite::Error, host_alias: &str) -> AppError {
    match &err {
        rusqlite::Error::SqliteFailure(sqlite_err, msg)
            if sqlite_err.code == rusqlite::ErrorCode::ConstraintViolation
                && msg.as_deref().unwrap_or("").contains("host_alias") =>
        {
            AppError::ValidationError(format!(
                "Já existe um perfil com o host alias \"{}\". Escolha outro alias.",
                host_alias
            ))
        }
        _ => AppError::DatabaseError(err.to_string()),
    }
}

pub struct ProfileRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> ProfileRepository<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn get_all(&self) -> Result<Vec<Profile>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, github_username, email, host_alias, ssh_key_id, default_directory, default_branch, description, avatar_url, is_active, created_at, updated_at FROM profiles ORDER BY name",
        )?;

        let profiles = stmt
            .query_map([], |row| {
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
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(profiles)
    }

    pub fn get_by_id(&self, id: &str) -> Result<Profile, AppError> {
        self.conn.query_row(
            "SELECT id, name, github_username, email, host_alias, ssh_key_id, default_directory, default_branch, description, avatar_url, is_active, created_at, updated_at FROM profiles WHERE id = ?1",
            params![id],
            |row| {
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
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::ProfileNotFound {
                id: id.to_string(),
            },
            other => AppError::DatabaseError(other.to_string()),
        })
    }

    pub fn create(&self, dto: &CreateProfileDto) -> Result<Profile, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        self.conn
            .execute(
                "INSERT INTO profiles (id, name, github_username, email, host_alias, ssh_key_id, default_directory, default_branch, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    id,
                    dto.name,
                    dto.github_username,
                    dto.email,
                    dto.host_alias,
                    dto.ssh_key_id,
                    dto.default_directory,
                    dto.default_branch,
                    dto.description,
                    now,
                    now,
                ],
            )
            .map_err(|e| map_constraint_error(e, &dto.host_alias))?;

        self.get_by_id(&id)
    }

    pub fn update(&self, id: &str, dto: &CreateProfileDto) -> Result<Profile, AppError> {
        let now = chrono::Utc::now().to_rfc3339();

        let affected = self
            .conn
            .execute(
                "UPDATE profiles SET name = ?1, github_username = ?2, email = ?3, host_alias = ?4, ssh_key_id = ?5, default_directory = ?6, default_branch = ?7, description = ?8, updated_at = ?9 WHERE id = ?10",
                params![
                    dto.name,
                    dto.github_username,
                    dto.email,
                    dto.host_alias,
                    dto.ssh_key_id,
                    dto.default_directory,
                    dto.default_branch,
                    dto.description,
                    now,
                    id,
                ],
            )
            .map_err(|e| map_constraint_error(e, &dto.host_alias))?;

        if affected == 0 {
            return Err(AppError::ProfileNotFound {
                id: id.to_string(),
            });
        }

        self.get_by_id(id)
    }

    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        let affected = self
            .conn
            .execute("DELETE FROM profiles WHERE id = ?1", params![id])?;

        if affected == 0 {
            return Err(AppError::ProfileNotFound {
                id: id.to_string(),
            });
        }

        Ok(())
    }

    pub fn set_active(&self, id: &str) -> Result<(), AppError> {
        self.conn
            .execute("UPDATE profiles SET is_active = 0", [])?;
        let affected = self.conn.execute(
            "UPDATE profiles SET is_active = 1 WHERE id = ?1",
            params![id],
        )?;

        if affected == 0 {
            return Err(AppError::ProfileNotFound {
                id: id.to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../migrations/001_initial.sql"))
            .unwrap();
        conn
    }

    fn dto(name: &str, host_alias: &str) -> CreateProfileDto {
        CreateProfileDto {
            name: name.to_string(),
            github_username: format!("{}-user", name),
            email: format!("{}@example.com", name),
            host_alias: host_alias.to_string(),
            ssh_key_id: None,
            default_directory: None,
            default_branch: "main".to_string(),
            description: None,
            pat_token: None,
        }
    }

    #[test]
    fn test_create_and_list_more_than_two_profiles() {
        let conn = setup_db();
        let repo = ProfileRepository::new(&conn);

        for i in 1..=5 {
            repo.create(&dto(&format!("perfil-{}", i), &format!("github-perfil-{}", i)))
                .unwrap();
        }

        let profiles = repo.get_all().unwrap();
        assert_eq!(profiles.len(), 5);
        assert!(profiles.iter().all(|p| !p.is_active));
    }

    #[test]
    fn test_create_duplicate_host_alias_returns_friendly_error() {
        let conn = setup_db();
        let repo = ProfileRepository::new(&conn);

        repo.create(&dto("Pessoal", "github.com")).unwrap();
        let err = repo
            .create(&dto("Trabalho", "github.com"))
            .expect_err("alias duplicado deve falhar");

        match err {
            AppError::ValidationError(msg) => {
                assert!(msg.contains("github.com"), "mensagem deve citar o alias: {}", msg);
            }
            other => panic!("esperava ValidationError, veio: {:?}", other),
        }
    }

    #[test]
    fn test_update_duplicate_host_alias_returns_friendly_error() {
        let conn = setup_db();
        let repo = ProfileRepository::new(&conn);

        let first = repo.create(&dto("Pessoal", "github-pessoal")).unwrap();
        repo.create(&dto("Trabalho", "github-trabalho")).unwrap();

        let err = repo
            .update(&first.id, &dto("Pessoal Editado", "github-trabalho"))
            .expect_err("alias duplicado no update deve falhar");

        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn test_set_active_switches_exclusively() {
        let conn = setup_db();
        let repo = ProfileRepository::new(&conn);

        let a = repo.create(&dto("A", "github-a")).unwrap();
        let b = repo.create(&dto("B", "github-b")).unwrap();

        repo.set_active(&a.id).unwrap();
        repo.set_active(&b.id).unwrap();

        let profiles = repo.get_all().unwrap();
        let actives: Vec<_> = profiles.iter().filter(|p| p.is_active).collect();
        assert_eq!(actives.len(), 1);
        assert_eq!(actives[0].id, b.id);
    }
}
