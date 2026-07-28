use crate::error::AppError;
use keyring::Entry;

const SERVICE_NAME: &str = "github-profile-manager";

pub struct CredentialManager;

impl CredentialManager {
    pub fn save_pat(profile_id: &str, token: &str) -> Result<(), AppError> {
        let entry = Entry::new(SERVICE_NAME, profile_id)
            .map_err(|e| AppError::SystemError(format!("Failed to create credential entry: {}", e)))?;

        entry
            .set_password(token)
            .map_err(|e| AppError::SystemError(format!("Failed to save PAT: {}", e)))?;

        tracing::info!("PAT saved for profile: {}", profile_id);
        Ok(())
    }

    pub fn get_pat(profile_id: &str) -> Result<Option<String>, AppError> {
        let entry = Entry::new(SERVICE_NAME, profile_id)
            .map_err(|e| AppError::SystemError(format!("Failed to create credential entry: {}", e)))?;

        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(AppError::SystemError(format!("Failed to retrieve PAT: {}", e))),
        }
    }

    pub fn delete_pat(profile_id: &str) -> Result<(), AppError> {
        let entry = Entry::new(SERVICE_NAME, profile_id)
            .map_err(|e| AppError::SystemError(format!("Failed to create credential entry: {}", e)))?;

        match entry.delete_credential() {
            Ok(()) => {
                tracing::info!("PAT deleted for profile: {}", profile_id);
                Ok(())
            }
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(AppError::SystemError(format!("Failed to delete PAT: {}", e))),
        }
    }

    #[allow(dead_code)]
    pub fn sanitize_log(input: &str) -> String {
        let mut result = input.to_string();
        if let Some(start) = result.find("ghp_") {
            let end = std::cmp::min(start + 40, result.len());
            result.replace_range(start..end, "ghp_****");
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_log_with_token() {
        let log = "Token: ghp_1234567890abcdef1234567890abcdef1234";
        let sanitized = CredentialManager::sanitize_log(log);
        assert_eq!(sanitized, "Token: ghp_****");
    }

    #[test]
    fn test_sanitize_log_without_token() {
        let log = "No token here";
        let sanitized = CredentialManager::sanitize_log(log);
        assert_eq!(sanitized, "No token here");
    }

    #[test]
    fn test_sanitize_log_empty() {
        let log = "";
        let sanitized = CredentialManager::sanitize_log(log);
        assert_eq!(sanitized, "");
    }
}
