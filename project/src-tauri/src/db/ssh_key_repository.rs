use crate::error::AppError;
use crate::models::ssh_key::SshKey;
use rusqlite::params;
use uuid::Uuid;

pub struct SshKeyRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SshKeyRepository<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn get_all(&self) -> Result<Vec<SshKey>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, key_type, public_key_path, private_key_path, fingerprint, has_passphrase, created_at FROM ssh_keys ORDER BY name",
        )?;

        let keys = stmt
            .query_map([], |row| {
                Ok(SshKey {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    key_type: row.get(2)?,
                    public_key_path: row.get(3)?,
                    private_key_path: row.get(4)?,
                    fingerprint: row.get(5)?,
                    has_passphrase: row.get::<_, i32>(6)? == 1,
                    created_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(keys)
    }

    pub fn get_by_id(&self, id: &str) -> Result<SshKey, AppError> {
        self.conn.query_row(
            "SELECT id, name, key_type, public_key_path, private_key_path, fingerprint, has_passphrase, created_at FROM ssh_keys WHERE id = ?1",
            params![id],
            |row| {
                Ok(SshKey {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    key_type: row.get(2)?,
                    public_key_path: row.get(3)?,
                    private_key_path: row.get(4)?,
                    fingerprint: row.get(5)?,
                    has_passphrase: row.get::<_, i32>(6)? == 1,
                    created_at: row.get(7)?,
                })
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::SshKeyNotFound {
                path: id.to_string(),
            },
            other => AppError::DatabaseError(other.to_string()),
        })
    }

    pub fn create(
        &self,
        name: &str,
        key_type: &str,
        public_key_path: &str,
        private_key_path: &str,
        fingerprint: &str,
        has_passphrase: bool,
    ) -> Result<SshKey, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO ssh_keys (id, name, key_type, public_key_path, private_key_path, fingerprint, has_passphrase, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                name,
                key_type,
                public_key_path,
                private_key_path,
                fingerprint,
                has_passphrase as i32,
                now,
            ],
        )?;

        self.get_by_id(&id)
    }

    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        let affected = self
            .conn
            .execute("DELETE FROM ssh_keys WHERE id = ?1", params![id])?;

        if affected == 0 {
            return Err(AppError::SshKeyNotFound {
                path: id.to_string(),
            });
        }

        Ok(())
    }
}
