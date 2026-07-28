use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Perfil não encontrado: {id}")]
    ProfileNotFound { id: String },

    #[error("Chave SSH não encontrada: {path}")]
    SshKeyNotFound { path: String },

    #[error("Token GitHub inválido")]
    #[allow(dead_code)]
    InvalidGitHubToken,

    #[error("Erro no banco de dados: {0}")]
    DatabaseError(String),

    #[error("Erro de sistema: {0}")]
    SystemError(String),

    #[error("Configuração inválida: {0}")]
    #[allow(dead_code)]
    ConfigError(String),

    #[error("Erro de validação: {0}")]
    ValidationError(String),
}

#[derive(Serialize)]
pub struct AppErrorDto {
    pub code: String,
    pub message: String,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let dto = AppErrorDto {
            code: match self {
                AppError::ProfileNotFound { .. } => "PROFILE_NOT_FOUND".to_string(),
                AppError::SshKeyNotFound { .. } => "SSH_KEY_NOT_FOUND".to_string(),
                AppError::InvalidGitHubToken => "INVALID_GITHUB_TOKEN".to_string(),
                AppError::DatabaseError(_) => "DATABASE_ERROR".to_string(),
                AppError::SystemError(_) => "SYSTEM_ERROR".to_string(),
                AppError::ConfigError(_) => "CONFIG_ERROR".to_string(),
                AppError::ValidationError(_) => "VALIDATION_ERROR".to_string(),
            },
            message: self.to_string(),
        };
        dto.serialize(serializer)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}
