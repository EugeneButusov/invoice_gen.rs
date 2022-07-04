pub mod client;

#[derive(Serialize, Clone)]
pub struct ClockifySettings {
    pub api_key: String,
    pub workspace_id: String,
    pub user_id: String,
}
