use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstanceStatus {
    #[default]
    Running,
    Waiting,
}

impl InstanceStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            InstanceStatus::Running => "⚡",
            InstanceStatus::Waiting => "⏳",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeInstance {
    pub pane_id: u32,
    pub folder: String,
    pub status: InstanceStatus,
    #[serde(default)]
    #[allow(dead_code)]
    pub timestamp: u64,
}

impl ClaudeInstance {
    pub fn parse_json(data: &[u8]) -> Vec<ClaudeInstance> {
        let json_str = String::from_utf8_lossy(data);
        serde_json::from_str(&json_str).unwrap_or_default()
    }
}
