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
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MaestroOutput {
    #[serde(default)]
    pub instances: Vec<ClaudeInstance>,
}

impl MaestroOutput {
    pub fn parse(data: &[u8]) -> Self {
        let json_str = String::from_utf8_lossy(data);
        serde_json::from_str(&json_str).unwrap_or_default()
    }
}
