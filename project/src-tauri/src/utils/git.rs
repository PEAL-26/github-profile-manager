use crate::error::AppError;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn get_gitconfig_path() -> Result<PathBuf, AppError> {
    let home = std::env::var("USERPROFILE").map_err(|e| AppError::SystemError(e.to_string()))?;
    Ok(PathBuf::from(home).join(".gitconfig"))
}

pub fn get_global_git_identity() -> Result<(String, String), AppError> {
    let output = std::process::Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .map_err(|e| AppError::SystemError(format!("Falha ao executar git: {}", e)))?;

    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = std::process::Command::new("git")
        .args(["config", "--global", "user.email"])
        .output()
        .map_err(|e| AppError::SystemError(format!("Falha ao executar git: {}", e)))?;

    let email = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok((name, email))
}

pub fn set_global_git_identity(
    name: &str,
    email: &str,
    ssh_key_path: Option<&str>,
) -> Result<(), AppError> {
    std::process::Command::new("git")
        .args(["config", "--global", "user.name", name])
        .output()
        .map_err(|e| AppError::SystemError(format!("Falha ao executar git: {}", e)))?;

    std::process::Command::new("git")
        .args(["config", "--global", "user.email", email])
        .output()
        .map_err(|e| AppError::SystemError(format!("Falha ao executar git: {}", e)))?;

    if let Some(key_path) = ssh_key_path {
        let ssh_command = format!("ssh -i \"{}\"", key_path);
        std::process::Command::new("git")
            .args(["config", "--global", "core.sshCommand", &ssh_command])
            .output()
            .map_err(|e| AppError::SystemError(format!("Falha ao executar git: {}", e)))?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn add_ssh_config_block(host_alias: &str, identity_file: &str) -> Result<(), AppError> {
    let ssh_dir = crate::utils::ssh::get_ssh_dir()?;
    let config_path = ssh_dir.join("config");

    let begin_marker = format!("# GHPM_BEGIN: {}", host_alias);
    let end_marker = format!("# GHPM_END: {}", host_alias);

    let content = if config_path.exists() {
        fs::read_to_string(&config_path).unwrap_or_default()
    } else {
        String::new()
    };

    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let start_idx = lines.iter().position(|l| l == &begin_marker);
    let end_idx = lines.iter().position(|l| l == &end_marker);

    if let (Some(start), Some(end)) = (start_idx, end_idx) {
        for _ in start..=end {
            lines.remove(start);
        }
    }

    let block = vec![
        begin_marker,
        format!("Host {}", host_alias),
        "    HostName github.com".to_string(),
        "    User git".to_string(),
        format!("    IdentityFile {}", identity_file),
        "    IdentitiesOnly yes".to_string(),
        end_marker,
    ];

    if !lines.is_empty() && !lines.last().unwrap().is_empty() {
        lines.push(String::new());
    }

    lines.extend(block);

    let new_content = lines.join("\n") + "\n";
    fs::write(&config_path, new_content)
        .map_err(|e| AppError::SystemError(format!("Falha ao escrever ~/.ssh/config: {}", e)))?;

    Ok(())
}

#[allow(dead_code)]
pub fn remove_ssh_config_block(host_alias: &str) -> Result<(), AppError> {
    let ssh_dir = crate::utils::ssh::get_ssh_dir()?;
    let config_path = ssh_dir.join("config");

    if !config_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| AppError::SystemError(format!("Falha ao ler ~/.ssh/config: {}", e)))?;

    let begin_marker = format!("# GHPM_BEGIN: {}", host_alias);
    let end_marker = format!("# GHPM_END: {}", host_alias);

    let lines: Vec<String> = content.lines().map(String::from).collect();
    let mut new_lines = Vec::new();
    let mut skip = false;

    for line in &lines {
        if line == &begin_marker {
            skip = true;
            continue;
        }
        if line == &end_marker {
            skip = false;
            continue;
        }
        if !skip {
            new_lines.push(line.clone());
        }
    }

    let new_content = new_lines.join("\n") + "\n";
    fs::write(&config_path, new_content)
        .map_err(|e| AppError::SystemError(format!("Falha ao escrever ~/.ssh/config: {}", e)))?;

    Ok(())
}
