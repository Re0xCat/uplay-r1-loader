use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub uplay: Uplay,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Uplay {
    pub name: String,
    pub saves: String,
    pub cd_keys: Vec<String>,
    pub language: String,
    pub offline_mode: bool,
    pub install_hooks: bool,
    pub log: Log,
    pub profile: Profile,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Log {
    pub write: bool,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    pub account_id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub ticket: String,
}
