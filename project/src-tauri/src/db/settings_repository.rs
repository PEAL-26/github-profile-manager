use crate::error::AppError;
use rusqlite::params;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub id: String,
    pub theme: String,
    pub auto_switch_enabled: bool,
    pub notify_on_switch: bool,
    pub default_ssh_algorithm: String,
    pub auto_start_ssh_agent: bool,
    pub scan_depth: u32,
    pub log_level: String,
    pub log_retention_days: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            theme: "system".to_string(),
            auto_switch_enabled: false,
            notify_on_switch: true,
            default_ssh_algorithm: "ed25519".to_string(),
            auto_start_ssh_agent: false,
            scan_depth: 3,
            log_level: "info".to_string(),
            log_retention_days: 30,
        }
    }
}

pub struct SettingsRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SettingsRepository<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    fn get_value(&self, key: &str) -> Option<String> {
        self.conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .ok()
    }

    fn set_value(&self, key: &str, value: &str) -> Result<(), AppError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get(&self) -> Result<AppSettings, AppError> {
        Ok(AppSettings {
            id: "default".to_string(),
            theme: self.get_value("theme").unwrap_or_else(|| "system".to_string()),
            auto_switch_enabled: self.get_value("auto_switch_enabled")
                .unwrap_or_else(|| "false".to_string())
                == "true",
            notify_on_switch: self.get_value("notify_on_switch")
                .unwrap_or_else(|| "true".to_string())
                == "true",
            default_ssh_algorithm: self.get_value("default_ssh_algorithm")
                .unwrap_or_else(|| "ed25519".to_string()),
            auto_start_ssh_agent: self.get_value("auto_start_ssh_agent")
                .unwrap_or_else(|| "false".to_string())
                == "true",
            scan_depth: self
                .get_value("scan_depth")
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            log_level: self.get_value("log_level").unwrap_or_else(|| "info".to_string()),
            log_retention_days: self
                .get_value("log_retention_days")
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
        })
    }

    pub fn save(&self, settings: &AppSettings) -> Result<(), AppError> {
        self.set_value("theme", &settings.theme)?;
        self.set_value(
            "auto_switch_enabled",
            &settings.auto_switch_enabled.to_string(),
        )?;
        self.set_value(
            "notify_on_switch",
            &settings.notify_on_switch.to_string(),
        )?;
        self.set_value("default_ssh_algorithm", &settings.default_ssh_algorithm)?;
        self.set_value(
            "auto_start_ssh_agent",
            &settings.auto_start_ssh_agent.to_string(),
        )?;
        self.set_value("scan_depth", &settings.scan_depth.to_string())?;
        self.set_value("log_level", &settings.log_level)?;
        self.set_value(
            "log_retention_days",
            &settings.log_retention_days.to_string(),
        )?;
        Ok(())
    }
}
