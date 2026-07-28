use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub github_username: String,
    pub email: String,
    pub host_alias: String,
    pub ssh_key_id: Option<String>,
    pub default_directory: Option<String>,
    pub default_branch: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileDto {
    pub name: String,
    pub github_username: String,
    pub email: String,
    pub host_alias: String,
    pub ssh_key_id: Option<String>,
    pub default_directory: Option<String>,
    pub default_branch: String,
    pub description: Option<String>,
    pub pat_token: Option<String>,
}
