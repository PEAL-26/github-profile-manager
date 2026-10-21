use crate::error::AppError;
use crate::utils::process::run_hidden;
use std::path::PathBuf;

pub fn get_ssh_dir() -> Result<PathBuf, AppError> {
    let home = std::env::var("USERPROFILE").map_err(|e| AppError::SystemError(e.to_string()))?;
    Ok(PathBuf::from(home).join(".ssh"))
}

pub fn generate_ssh_key(
    name: &str,
    key_type: &str,
    passphrase: Option<&str>,
) -> Result<String, AppError> {
    let ssh_dir = get_ssh_dir()?;
    std::fs::create_dir_all(&ssh_dir).ok();

    let key_filename = format!("id_{}_{}", key_type, name);
    let key_path = ssh_dir.join(&key_filename);

    if key_path.exists() {
        return Err(AppError::ValidationError(format!(
            "Chave já existe: {}",
            key_path.display()
        )));
    }

    let comment = format!("ghpm-{}", name);
    let args = vec![
        "-t",
        key_type,
        "-f",
        key_path.to_str().unwrap(),
        "-N",
        passphrase.unwrap_or(""),
        "-C",
        &comment,
    ];

    let output = run_hidden("ssh-keygen", &args)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::SystemError(format!(
            "ssh-keygen falhou: {}",
            stderr
        )));
    }

    Ok(key_path.to_string_lossy().to_string())
}

pub fn get_public_key_path(name: &str, key_type: &str) -> Result<PathBuf, AppError> {
    let ssh_dir = get_ssh_dir()?;
    Ok(ssh_dir.join(format!("id_{}_{}.pub", key_type, name)))
}

pub fn get_fingerprint(private_key_path: &str) -> Result<String, AppError> {
    let output = run_hidden("ssh-keygen", &["-lf", private_key_path])?;

    if !output.status.success() {
        return Err(AppError::SystemError(
            "Falha ao obter fingerprint".to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let fingerprint = stdout
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string();

    Ok(fingerprint)
}

pub fn test_ssh_connection(host_alias: &str) -> Result<(bool, Option<String>, String, u64), AppError> {
    let start = std::time::Instant::now();

    let output = run_hidden("ssh", &["-T", &format!("git@{}", host_alias)])?;

    let latency = start.elapsed().as_millis() as u64;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}{}", stdout, stderr);

    let success = output.status.success()
        || combined.contains("successfully authenticated");

    let username = if success {
        combined
            .lines()
            .find(|l| l.contains("Hi "))
            .and_then(|l| l.split_whitespace().nth(1))
            .map(|s| s.trim_end_matches('!').to_string())
    } else {
        None
    };

    Ok((success, username, combined, latency))
}
