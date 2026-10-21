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

impl AppError {
    /// Código estável do erro, compartilhado entre a API IPC e o log de erros.
    pub fn code(&self) -> &'static str {
        match self {
            AppError::ProfileNotFound { .. } => "PROFILE_NOT_FOUND",
            AppError::SshKeyNotFound { .. } => "SSH_KEY_NOT_FOUND",
            AppError::InvalidGitHubToken => "INVALID_GITHUB_TOKEN",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::SystemError(_) => "SYSTEM_ERROR",
            AppError::ConfigError(_) => "CONFIG_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let dto = AppErrorDto {
            code: self.code().to_string(),
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
