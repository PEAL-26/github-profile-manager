use rusqlite::params;
use crate::error::AppError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLog {
    pub id: String,
    pub level: String,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
    pub created_at: String,
}

pub struct ActivityLogger<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> ActivityLogger<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn log(
        &self,
        level: &str,
        category: &str,
        message: &str,
        details: Option<&str>,
    ) -> Result<(), AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO activity_logs (id, level, category, message, details, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, level, category, message, details, created_at],
        )?;

        Ok(())
    }

    pub fn info(&self, category: &str, message: &str) -> Result<(), AppError> {
        self.log("INFO", category, message, None)
    }

    pub fn warn(&self, category: &str, message: &str) -> Result<(), AppError> {
        self.log("WARN", category, message, None)
    }

    #[allow(dead_code)]
    pub fn error(&self, category: &str, message: &str, details: Option<&str>) -> Result<(), AppError> {
        self.log("ERROR", category, message, details)
    }

    pub fn log_profile_switch(&self, profile_name: &str) -> Result<(), AppError> {
        self.info("profile", &format!("Switched to profile: {}", profile_name))
    }

    pub fn log_key_generated(&self, key_name: &str) -> Result<(), AppError> {
        self.info("ssh_key", &format!("Generated SSH key: {}", key_name))
    }

    pub fn log_key_deleted(&self, key_name: &str) -> Result<(), AppError> {
        self.warn("ssh_key", &format!("Deleted SSH key: {}", key_name))
    }

    pub fn get_all(&self) -> Result<Vec<ActivityLog>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, level, category, message, details, created_at
             FROM activity_logs
             ORDER BY created_at DESC
             LIMIT 100",
        )?;

        let logs = stmt
            .query_map([], |row| {
                Ok(ActivityLog {
                    id: row.get(0)?,
                    level: row.get(1)?,
                    category: row.get(2)?,
                    message: row.get(3)?,
                    details: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS activity_logs (
                id TEXT PRIMARY KEY NOT NULL,
                level TEXT NOT NULL,
                category TEXT NOT NULL,
                message TEXT NOT NULL,
                details TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_log_and_retrieve() {
        let conn = setup_db();
        let logger = ActivityLogger::new(&conn);

        logger.info("test", "Test message").unwrap();

        let logs = logger.get_all().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].level, "INFO");
        assert_eq!(logs[0].category, "test");
        assert_eq!(logs[0].message, "Test message");
    }

    #[test]
    fn test_log_with_details() {
        let conn = setup_db();
        let logger = ActivityLogger::new(&conn);

        logger.error("test", "Error occurred", Some("stack trace")).unwrap();

        let logs = logger.get_all().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].level, "ERROR");
        assert_eq!(logs[0].details, Some("stack trace".to_string()));
    }

    #[test]
    fn test_profile_switch_log() {
        let conn = setup_db();
        let logger = ActivityLogger::new(&conn);

        logger.log_profile_switch("Work").unwrap();

        let logs = logger.get_all().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].category, "profile");
        assert!(logs[0].message.contains("Work"));
    }

    #[test]
    fn test_serialization_uses_camel_case_dates() {
        let conn = setup_db();
        let logger = ActivityLogger::new(&conn);
        logger.info("test", "mensagem").unwrap();

        let logs = logger.get_all().unwrap();
        let value = serde_json::to_value(&logs[0]).unwrap();

        // O frontend lê `createdAt`; sem rename_all o campo saía como
        // `created_at` e a tela renderizava "Invalid Date".
        assert!(value.get("createdAt").is_some(), "esperava campo createdAt: {}", value);
        assert!(value.get("created_at").is_none());
        let raw = value["createdAt"].as_str().unwrap();
        assert!(!raw.is_empty());
    }
}
