use serde::{Deserialize, Serialize};

/// Repositório Git rastreado pelo Scanner (Módulo 04).
///
/// `current_git_*`, `suggested_profile_id` e `suggestion_conflict` refletem o
/// estado capturado no último scan (ou na última aplicação de perfil) —
/// `is_valid_profile` e `profile_name` são calculados a cada leitura.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub path: String,
    pub profile_id: Option<String>,
    pub profile_name: Option<String>,
    pub current_git_name: Option<String>,
    pub current_git_email: Option<String>,
    pub current_ssh_command: Option<String>,
    pub is_valid_profile: bool,
    pub remote_url: Option<String>,
    pub suggested_profile_id: Option<String>,
    pub suggestion_conflict: bool,
    pub last_scanned_at: String,
}

/// Pasta raiz configurada para a varredura do scanner.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanRoot {
    pub id: String,
    pub path: String,
    pub created_at: String,
}
