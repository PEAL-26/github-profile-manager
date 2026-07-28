use crate::error::AppError;
use crate::models::profile::{CreateProfileDto, Profile};
use rusqlite::params;
use uuid::Uuid;

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

        self.conn.execute(
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
        )?;

        self.get_by_id(&id)
    }

    pub fn update(&self, id: &str, dto: &CreateProfileDto) -> Result<Profile, AppError> {
        let now = chrono::Utc::now().to_rfc3339();

        let affected = self.conn.execute(
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
        )?;

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
