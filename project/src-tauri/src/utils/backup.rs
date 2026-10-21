//! Backup/Exportação (Módulo 02.5).
//!
//! Formato `.ghpm-backup`:
//! - **Modo padrão (sem senha):** JSON com dados públicos dos perfis,
//!   chave pública e configurações. Nunca inclui chaves privadas ou PATs.
//! - **Modo completo (com senha):** o mesmo JSON (incluindo chaves privadas
//!   e PATs) cifrado com AES-256-GCM; chave derivada via PBKDF2-SHA256
//!   (100.000 iterações, salt aleatório de 16 bytes), IV de 12 bytes único
//!   por exportação. O ficheiro começa com o cabeçalho `GHPM_ENC_V2`.

use crate::db::profile_repository::ProfileRepository;
use crate::db::settings_repository::{AppSettings, SettingsRepository};
use crate::db::ssh_key_repository::SshKeyRepository;
use crate::error::AppError;
use crate::models::profile::CreateProfileDto;
use crate::utils::credentials::CredentialManager;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use pbkdf2::pbkdf2_hmac;
use rand::rngs::OsRng;
use rand::RngCore;
use rusqlite::Connection;
use sha2::Sha256;
use std::fs;
use zeroize::Zeroize;

pub const BACKUP_VERSION: &str = "2.0";
pub const ENCRYPTED_HEADER: &str = "GHPM_ENC_V2";
const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

/// Um perfil dentro do backup. Os campos `ssh_private_key` e `pat_token`
/// só são preenchidos no modo completo e só trafegam em ficheiro
/// criptografado.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupProfile {
    pub name: String,
    pub github_username: String,
    pub email: String,
    pub host_alias: String,
    pub default_directory: Option<String>,
    pub default_branch: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub ssh_key_name: Option<String>,
    pub ssh_key_type: Option<String>,
    pub ssh_fingerprint: Option<String>,
    pub ssh_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_token: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupFile {
    pub version: String,
    pub exported_at: String,
    pub profiles: Vec<BackupProfile>,
    pub settings: AppSettings,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportReport {
    pub profiles_created: usize,
    pub profiles_updated: usize,
    pub keys_restored: usize,
    pub settings_applied: bool,
    pub pats_restored: usize,
}

fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

/// Cifra o JSON com AES-256-GCM. Saída:
/// ```text
/// GHPM_ENC_V2
/// <salt base64>
/// <nonce base64>
/// <ciphertext+tag base64>
/// ```
pub fn encrypt_payload(json: &str, password: &str) -> Result<String, AppError> {
    if password.is_empty() {
        return Err(AppError::ValidationError(
            "A senha do backup não pode ser vazia".to_string(),
        ));
    }

    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let mut key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    key.zeroize();

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), json.as_bytes())
        .map_err(|_| AppError::SystemError("Falha ao criptografar o backup".to_string()))?;

    Ok(format!(
        "{}\n{}\n{}\n{}\n",
        ENCRYPTED_HEADER,
        B64.encode(salt),
        B64.encode(nonce),
        B64.encode(ciphertext)
    ))
}

/// Decifra um ficheiro de backup no formato `GHPM_ENC_V2`.
pub fn decrypt_payload(content: &str, password: &str) -> Result<String, AppError> {
    let mut lines = content.lines();
    let header = lines.next().unwrap_or("").trim();
    if header != ENCRYPTED_HEADER {
        return Err(AppError::ValidationError(
            "Ficheiro de backup não está no formato criptografado GHPM_ENC_V2".to_string(),
        ));
    }

    let decode_line = |lines: &mut dyn Iterator<Item = &str>, what: &str| {
        let line = lines
            .next()
            .ok_or_else(|| AppError::ValidationError(format!("Backup corrompido: falta {}", what)))?;
        B64.decode(line.trim())
            .map_err(|_| AppError::ValidationError(format!("Backup corrompido: {} inválido", what)))
    };

    let salt = decode_line(&mut lines, "salt")?;
    let nonce = decode_line(&mut lines, "nonce")?;
    let ciphertext = decode_line(&mut lines, "dados")?;

    if salt.len() != SALT_LEN || nonce.len() != NONCE_LEN {
        return Err(AppError::ValidationError(
            "Backup corrompido: salt/nonce com tamanho inválido".to_string(),
        ));
    }

    let mut key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    key.zeroize();

    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| {
            AppError::ValidationError(
                "Senha incorreta ou backup corrompido".to_string(),
            )
        })?;

    String::from_utf8(plaintext)
        .map_err(|_| AppError::ValidationError("Backup decifrado não é texto UTF-8".to_string()))
}

/// Indica se o conteúdo está no formato criptografado.
pub fn is_encrypted(content: &str) -> bool {
    content.lines().next().unwrap_or("").trim() == ENCRYPTED_HEADER
}

fn profile_to_backup_profile(profile: &crate::models::profile::Profile) -> BackupProfile {
    BackupProfile {
        name: profile.name.clone(),
        github_username: profile.github_username.clone(),
        email: profile.email.clone(),
        host_alias: profile.host_alias.clone(),
        default_directory: profile.default_directory.clone(),
        default_branch: profile.default_branch.clone(),
        description: profile.description.clone(),
        avatar_url: profile.avatar_url.clone(),
        is_active: profile.is_active,
        ssh_key_name: None,
        ssh_key_type: None,
        ssh_fingerprint: None,
        ssh_public_key: None,
        ssh_private_key: None,
        pat_token: None,
    }
}

/// Monta o conteúdo do backup (JSON puro ou cifrado).
///
/// Regra de ouro: chaves privadas e PATs **só** saem daqui em ficheiro
/// cifrado (`include_secrets` exige senha).
pub fn export_to_string(
    conn: &Connection,
    include_secrets: bool,
    password: Option<&str>,
) -> Result<(String, usize), AppError> {
    if include_secrets && password.unwrap_or("").is_empty() {
        return Err(AppError::ValidationError(
            "Uma senha é obrigatória para exportar chaves privadas e tokens".to_string(),
        ));
    }

    let profiles = ProfileRepository::new(conn).get_all()?;
    let settings = SettingsRepository::new(conn).get()?;

    let mut backup_profiles: Vec<BackupProfile> = Vec::with_capacity(profiles.len());

    for profile in &profiles {
        let mut bp = profile_to_backup_profile(profile);

        if let Some(key_id) = &profile.ssh_key_id {
            if let Ok(key) = SshKeyRepository::new(conn).get_by_id(key_id) {
                bp.ssh_key_name = Some(key.name.clone());
                bp.ssh_key_type = Some(key.key_type.clone());
                bp.ssh_fingerprint = Some(key.fingerprint.clone());
                bp.ssh_public_key = fs::read_to_string(&key.public_key_path)
                    .ok()
                    .map(|s| s.trim().to_string());
                if include_secrets {
                    bp.ssh_private_key = fs::read_to_string(&key.private_key_path)
                        .ok()
                        .map(|s| s.trim().to_string());
                }
            }
        }

        if include_secrets {
            // Coleta resiliente: cofre indisponível não invalida o backup.
            bp.pat_token = CredentialManager::get_pat(&profile.id).ok().flatten();
        }

        backup_profiles.push(bp);
    }

    let backup = BackupFile {
        version: BACKUP_VERSION.to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        profiles: backup_profiles,
        settings,
    };

    let json = serde_json::to_string_pretty(&backup)
        .map_err(|e| AppError::SystemError(format!("Falha ao serializar backup: {}", e)))?;

    let count = backup.profiles.len();

    if include_secrets {
        // Segredos presentes: cifra sempre.
        encrypt_payload(&json, password.unwrap_or(""))
    } else if let Some(pw) = password {
        // Sem segredos, mas usuário forneceu senha: cifra mesmo assim.
        encrypt_payload(&json, pw)
    } else {
        Ok(json)
    }
    .map(|content| (content, count))
}

/// Heurística: chaves OpenSSH com passphrase usam KDF `bcrypt` na
/// segunda linha do ficheiro; sem passphrase usam `none`.
fn detect_passphrase(private_key: &str) -> bool {
    let b64_body = private_key.lines().nth(1).unwrap_or("");
    match B64.decode(b64_body.trim()) {
        Ok(raw) => raw.windows(6).any(|w| w == b"bcrypt"),
        Err(_) => false,
    }
}

fn restore_ssh_key(
    conn: &Connection,
    name: &str,
    key_type: &str,
    fingerprint: Option<&str>,
    public_key: &str,
    private_key: &str,
) -> Result<String, AppError> {
    let ssh_dir = crate::utils::ssh::get_ssh_dir()?;
    fs::create_dir_all(&ssh_dir).ok();

    let filename = format!("id_{}_{}", key_type, name);
    let private_path = ssh_dir.join(&filename);
    let public_path = ssh_dir.join(format!("{}.pub", filename));

    // Nunca sobrescreve chaves já existentes na máquina.
    if !private_path.exists() {
        fs::write(&private_path, private_key)
            .map_err(|e| AppError::SystemError(format!("Falha ao restaurar chave privada: {}", e)))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&private_path, fs::Permissions::from_mode(0o600));
        }
    }
    if !public_path.exists() {
        fs::write(&public_path, public_key)
            .map_err(|e| AppError::SystemError(format!("Falha ao restaurar chave pública: {}", e)))?;
    }

    let repo = SshKeyRepository::new(conn);
    let private_str = private_path.to_string_lossy().to_string();
    let public_str = public_path.to_string_lossy().to_string();

    // Reutiliza o registo do banco se este ficheiro já foi vinculado.
    if let Ok(all) = repo.get_all() {
        if let Some(existing) = all.iter().find(|k| k.private_key_path == private_str) {
            return Ok(existing.id.clone());
        }
    }

    let fp = fingerprint.unwrap_or("").to_string();
    let key = repo.create(
        name,
        key_type,
        &public_str,
        &private_str,
        &fp,
        detect_passphrase(private_key),
    )?;
    Ok(key.id)
}

/// Importa/restaura um backup no banco e no ~/.ssh.
///
/// Perfis são identificados pelo `host_alias`: existente → atualiza,
/// inexistente → cria (importação idempotente).
pub fn import_from_string(
    conn: &Connection,
    content: &str,
    password: Option<&str>,
) -> Result<ImportReport, AppError> {
    let json = if is_encrypted(content) {
        let pw = password.ok_or_else(|| {
            AppError::ValidationError(
                "Este backup está protegido com senha".to_string(),
            )
        })?;
        decrypt_payload(content, pw)?
    } else {
        content.to_string()
    };

    let backup: BackupFile = serde_json::from_str(&json)
        .map_err(|e| AppError::ValidationError(format!("Ficheiro de backup inválido: {}", e)))?;

    if backup.version != BACKUP_VERSION {
        return Err(AppError::ValidationError(format!(
            "Versão de backup não suportada: {} (esperada {})",
            backup.version, BACKUP_VERSION
        )));
    }

    let mut report = ImportReport::default();

    SettingsRepository::new(conn).save(&backup.settings)?;
    report.settings_applied = true;

    let profile_repo = ProfileRepository::new(conn);

    for bp in &backup.profiles {
        let mut ssh_key_id = None;
        if let (Some(key_name), Some(key_type), Some(private_key), Some(public_key)) = (
            bp.ssh_key_name.as_deref(),
            bp.ssh_key_type.as_deref(),
            bp.ssh_private_key.as_deref(),
            bp.ssh_public_key.as_deref(),
        ) {
            match restore_ssh_key(
                conn,
                key_name,
                key_type,
                bp.ssh_fingerprint.as_deref(),
                public_key,
                private_key,
            ) {
                Ok(id) => {
                    report.keys_restored += 1;
                    ssh_key_id = Some(id);
                }
                Err(e) => {
                    tracing::warn!("Falha ao restaurar chave '{}': {}", key_name, e);
                }
            }
        }

        let dto = CreateProfileDto {
            name: bp.name.clone(),
            github_username: bp.github_username.clone(),
            email: bp.email.clone(),
            host_alias: bp.host_alias.clone(),
            ssh_key_id,
            default_directory: bp.default_directory.clone(),
            default_branch: bp.default_branch.clone(),
            description: bp.description.clone(),
            pat_token: bp.pat_token.clone(),
        };

        let existing = profile_repo
            .get_all()?
            .into_iter()
            .find(|p| p.host_alias == bp.host_alias);

        let restored = match existing {
            Some(p) => {
                let updated = profile_repo.update(&p.id, &dto)?;
                report.profiles_updated += 1;
                updated
            }
            None => {
                let created = profile_repo.create(&dto)?;
                report.profiles_created += 1;
                created
            }
        };

        if bp.is_active {
            profile_repo.set_active(&restored.id)?;
        }

        if let Some(pat) = bp.pat_token.as_deref() {
            if CredentialManager::save_pat(&restored.id, pat).is_ok() {
                report.pats_restored += 1;
            }
        }
    }

    Ok(report)
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

    fn dto(name: &str, alias: &str) -> CreateProfileDto {
        CreateProfileDto {
            name: name.to_string(),
            github_username: format!("{}-user", name),
            email: format!("{}@example.com", name),
            host_alias: alias.to_string(),
            ssh_key_id: None,
            default_directory: None,
            default_branch: "main".to_string(),
            description: None,
            pat_token: None,
        }
    }

    const SAMPLE_JSON: &str = r#"{"version":"2.0","profiles":[{"name":"Pessoal"}]}"#;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let encrypted = encrypt_payload(SAMPLE_JSON, "senha-forte-123").unwrap();
        assert!(is_encrypted(&encrypted));
        assert!(encrypted.starts_with("GHPM_ENC_V2\n"));

        let decrypted = decrypt_payload(&encrypted, "senha-forte-123").unwrap();
        assert_eq!(decrypted, SAMPLE_JSON);
    }

    #[test]
    fn test_encrypt_uses_random_salt_and_nonce() {
        let a = encrypt_payload(SAMPLE_JSON, "senha").unwrap();
        let b = encrypt_payload(SAMPLE_JSON, "senha").unwrap();
        // Mesmo conteúdo + mesma senha geram ficheiros diferentes (salt/nonce
        // aleatórios por exportação).
        assert_ne!(a, b);
    }

    #[test]
    fn test_decrypt_wrong_password_fails() {
        let encrypted = encrypt_payload(SAMPLE_JSON, "senha-correta").unwrap();
        let err = decrypt_payload(&encrypted, "senha-errada").expect_err("deve falhar");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn test_decrypt_tampered_content_fails() {
        let encrypted = encrypt_payload(SAMPLE_JSON, "senha").unwrap();
        let mut lines: Vec<&str> = encrypted.lines().collect();
        // Altera um byte do ciphertext (base64).
        let mut data = lines[3].to_string();
        data.replace_range(0..1, if data.starts_with("A") { "B" } else { "A" });
        lines[3] = &data;
        let tampered = lines.join("\n");
        let err = decrypt_payload(&tampered, "senha").expect_err("deve falhar");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn test_export_plain_has_no_secrets_fields() {
        let conn = setup_db();
        ProfileRepository::new(&conn).create(&dto("Pessoal", "github-pessoal")).unwrap();

        let (content, count) = export_to_string(&conn, false, None).unwrap();
        assert_eq!(count, 1);
        assert!(!is_encrypted(&content));
        assert!(content.contains("\"github_username\""));
        assert!(!content.contains("ssh_private_key"));
        assert!(!content.contains("pat_token"));

        let parsed: BackupFile = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.version, BACKUP_VERSION);
        assert_eq!(parsed.profiles.len(), 1);
        assert_eq!(parsed.profiles[0].host_alias, "github-pessoal");
    }

    #[test]
    fn test_export_secrets_without_password_fails() {
        let conn = setup_db();
        ProfileRepository::new(&conn).create(&dto("Pessoal", "github-pessoal")).unwrap();

        let err = export_to_string(&conn, true, None).expect_err("deve exigir senha");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn test_import_creates_profiles_and_settings() {
        let source = setup_db();
        ProfileRepository::new(&source).create(&dto("Pessoal", "github-pessoal")).unwrap();
        ProfileRepository::new(&source).create(&dto("Trabalho", "github-trabalho")).unwrap();

        let (content, _) = export_to_string(&source, false, None).unwrap();

        let target = setup_db();
        let report = import_from_string(&target, &content, None).unwrap();

        assert_eq!(report.profiles_created, 2);
        assert_eq!(report.profiles_updated, 0);
        assert!(report.settings_applied);

        let profiles = ProfileRepository::new(&target).get_all().unwrap();
        assert_eq!(profiles.len(), 2);
    }

    #[test]
    fn test_import_is_idempotent_by_host_alias() {
        let source = setup_db();
        ProfileRepository::new(&source).create(&dto("Pessoal", "github-pessoal")).unwrap();
        let (content, _) = export_to_string(&source, false, None).unwrap();

        let target = setup_db();
        import_from_string(&target, &content, None).unwrap();
        let second = import_from_string(&target, &content, None).unwrap();

        assert_eq!(second.profiles_created, 0);
        assert_eq!(second.profiles_updated, 1);
        assert_eq!(ProfileRepository::new(&target).get_all().unwrap().len(), 1);
    }

    #[test]
    fn test_import_encrypted_requires_password() {
        let conn = setup_db();
        let err = import_from_string(&conn, "GHPM_ENC_V2\nxx\nyy\nzz\n", None)
            .expect_err("deve exigir senha");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn test_import_encrypted_roundtrip() {
        let source = setup_db();
        ProfileRepository::new(&source).create(&dto("Cifrado", "github-cifrado")).unwrap();
        let (content, _) = export_to_string(&source, false, Some("minha-senha")).unwrap();
        assert!(is_encrypted(&content));

        let target = setup_db();
        let report = import_from_string(&target, &content, Some("minha-senha")).unwrap();
        assert_eq!(report.profiles_created, 1);

        let wrong = import_from_string(&setup_db(), &content, Some("errada"));
        assert!(wrong.is_err());
    }

    #[test]
    fn test_import_rejects_unsupported_version() {
        let conn = setup_db();
        let json = r#"{"version":"99.0","exported_at":"now","profiles":[],"settings":{"id":"default","theme":"system","autoSwitchEnabled":false,"notifyOnSwitch":true,"defaultSshAlgorithm":"ed25519","autoStartSshAgent":false,"scanDepth":3,"logLevel":"info","logRetentionDays":30}}"#;
        let err = import_from_string(&conn, json, None).expect_err("versão inválida");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn test_detect_passphrase() {
        // Simula a estrutura base64 de uma chave OpenSSH: linha 1 com
        // "openssh-key-v1" + marcador kdf.
        let plain_body = {
            let mut v = b"openssh-key-v1\0none".to_vec();
            v.extend_from_slice(&[0u8; 8]);
            B64.encode(v)
        };
        let plain_key = format!("-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----", plain_body);
        assert!(!detect_passphrase(&plain_key));

        let enc_body = {
            let mut v = b"openssh-key-v1\0bcrypt".to_vec();
            v.extend_from_slice(&[0u8; 8]);
            B64.encode(v)
        };
        let enc_key = format!("-----BEGIN OPENSSH PRIVATE KEY-----\n{}\n-----END OPENSSH PRIVATE KEY-----", enc_body);
        assert!(detect_passphrase(&enc_key));
    }
}
