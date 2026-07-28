use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshKey {
    pub id: String,
    pub name: String,
    pub key_type: String,
    pub public_key_path: String,
    pub private_key_path: String,
    pub fingerprint: String,
    pub has_passphrase: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshTestResultDto {
    pub success: bool,
    pub username: Option<String>,
    pub raw_output: String,
    pub latency_ms: u64,
}
