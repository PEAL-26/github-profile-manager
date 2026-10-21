use crate::error::AppError;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const LOG_FILENAME: &str = "ghpm_errors.txt";

/// Diretório de instalação do aplicativo (pasta do executável).
fn install_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
}

/// Fallback: pasta de dados do app (~/.ghpm), usada quando a pasta de
/// instalação não permite escrita (ex.: Program Files).
fn fallback_dir() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".ghpm")
}

fn timestamp() -> String {
    chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

fn append_entry(path: &PathBuf, entry: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(entry.as_bytes())
}

/// Escreve no ficheiro da pasta de instalação; se falhar, usa o fallback.
fn write_entry(entry: &str) {
    let install_path = install_dir().map(|d| d.join(LOG_FILENAME));
    let wrote = install_path
        .as_ref()
        .map(|p| append_entry(p, entry).is_ok())
        .unwrap_or(false);

    if !wrote {
        let fallback = fallback_dir().join(LOG_FILENAME);
        if let Err(e) = append_entry(&fallback, entry) {
            tracing::warn!("Falha ao escrever log de erros: {}", e);
        }
    }
}

/// Registra um erro no ficheiro ghpm_errors.txt (apenas erros, não atividade).
///
/// Formato: [2026-10-20 14:30:05] [create_profile] VALIDATION_ERROR: mensagem
pub fn log_error(scope: &str, err: &AppError) {
    let entry = format!(
        "[{}] [{}] {}: {}\n",
        timestamp(),
        scope,
        err.code(),
        err
    );
    write_entry(&entry);
}

/// Registra uma entrada bruta de erro (ex.: mensagens capturadas no frontend).
pub fn log_message(scope: &str, message: &str) {
    let one_line: String = message.replace('\n', " ");
    // Trunca por caracteres (não bytes) para não cortar no meio de um acento.
    let truncated: String = one_line.chars().take(2000).collect();
    let entry = format!("[{}] [{}] ERROR: {}\n", timestamp(), scope, truncated);
    write_entry(&entry);
}

/// Registra um panic no ficheiro de erros (usado pelo panic hook global).
pub fn log_panic(info: &std::panic::PanicHookInfo<'_>) {
    let payload = info.payload();
    let message = payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "panic desconhecido".to_string());
    let location = info
        .location()
        .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
        .unwrap_or_else(|| "local desconhecido".to_string());

    let entry = format!("[{}] [PANIC] {} em {}\n", timestamp(), message, location);
    write_entry(&entry);
}

/// Envolve o resultado de um command Tauri: erros são registrados no
/// ficheiro antes de serem propagados para a UI.
pub fn guard<T>(command: &str, result: Result<T, AppError>) -> Result<T, AppError> {
    if let Err(err) = &result {
        log_error(command, err);
    }
    result
}

/// Caminho do ficheiro de log de erros (efetivamente usado).
pub fn log_file_path() -> PathBuf {
    if let Some(dir) = install_dir() {
        let candidate = dir.join(LOG_FILENAME);
        // Considera a pasta de instalação se ela existir e for gravável.
        if dir.is_dir()
            && OpenOptions::new()
                .append(true)
                .create(true)
                .open(&candidate)
                .is_ok()
        {
            return candidate;
        }
    }
    fallback_dir().join(LOG_FILENAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_error_appends_readable_entries() {
        let dir = std::env::temp_dir().join(format!("ghpm_errlog_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(LOG_FILENAME);

        let entry = format!(
            "[{}] [create_profile] VALIDATION_ERROR: alias duplicado\n",
            timestamp()
        );
        append_entry(&path, &entry).unwrap();
        append_entry(&path, &entry).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("[create_profile] VALIDATION_ERROR: alias duplicado"));
        // timestamp no formato AAAA-MM-DD HH:MM:SS
        assert!(lines[0].starts_with('['));
        assert!(lines[0].contains(']'));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_guard_logs_and_propagates_error() {
        let dir = std::env::temp_dir().join(format!("ghpm_guard_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();

        let result: Result<(), AppError> = Err(AppError::ValidationError("teste".to_string()));
        let guarded: Result<(), AppError> = guard("qualquer_command", result);
        assert!(guarded.is_err());

        fs::remove_dir_all(&dir).ok();
    }
}
