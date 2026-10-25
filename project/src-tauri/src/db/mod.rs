pub mod activity_logger;
pub mod profile_repository;
pub mod repository_repository;
pub mod settings_repository;
pub mod ssh_key_repository;

use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn init_and_migrate(&self) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        init_schema(&conn)
    }
}

/// Executa todas as migrações numa ligação (idempotente).
///
/// Também é usado pelos testes com bases de dados em memória.
pub fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(include_str!("../../migrations/001_initial.sql"))?;
    conn.execute_batch(include_str!("../../migrations/002_repositories_meta.sql"))?;
    ensure_column(conn, "repositories", "current_git_name", "TEXT")?;
    ensure_column(conn, "repositories", "current_git_email", "TEXT")?;
    ensure_column(conn, "repositories", "current_ssh_command", "TEXT")?;
    ensure_column(conn, "repositories", "suggested_profile_id", "TEXT")?;
    ensure_column(
        conn,
        "repositories",
        "suggestion_conflict",
        "INTEGER NOT NULL DEFAULT 0",
    )?;
    Ok(())
}

/// Adiciona uma coluna a uma tabela existente se ela ainda não existir.
///
/// O SQLite não suporta `ALTER TABLE ... ADD COLUMN IF NOT EXISTS`, por isso
/// verificamos via PRAGMA table_info antes de alterar (necessário porque as
/// migrações correm a cada arranque).
fn ensure_column(
    conn: &Connection,
    table: &str,
    column: &str,
    declaration: &str,
) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let exists = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(|name| name.ok())
        .any(|name| name == column);

    if !exists {
        conn.execute(
            &format!(
                "ALTER TABLE {} ADD COLUMN {} {}",
                table, column, declaration
            ),
            [],
        )?;
    }
    Ok(())
}
