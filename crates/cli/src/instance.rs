use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Running,
    Waiting,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Running => write!(f, "running"),
            Status::Waiting => write!(f, "waiting"),
        }
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "running" => Ok(Status::Running),
            "waiting" => Ok(Status::Waiting),
            _ => Err(format!("Invalid status: {s}. Use 'running' or 'waiting'")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub pane_id: u32,
    pub folder: String,
    pub status: Status,
}

pub struct InstanceStore {
    path: PathBuf,
}

impl InstanceStore {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from("/tmp/maestro-ai/instances.json"),
        }
    }

    pub fn load(&self) -> Vec<Instance> {
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, instances: &[Instance]) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(instances)?;
        fs::write(&self.path, json)
    }

    pub fn register(&self, pane_id: u32, folder: String) -> std::io::Result<()> {
        let mut instances = self.load();

        // Remove existing entry with same pane_id
        instances.retain(|i| i.pane_id != pane_id);

        // Add new instance
        instances.push(Instance {
            pane_id,
            folder,
            status: Status::Running,
        });

        self.save(&instances)
    }

    pub fn update_status(&self, pane_id: u32, status: Status) -> std::io::Result<()> {
        let mut instances = self.load();

        for instance in &mut instances {
            if instance.pane_id == pane_id {
                instance.status = status;
                break;
            }
        }

        self.save(&instances)
    }

    pub fn unregister(&self, pane_id: u32) -> std::io::Result<()> {
        let mut instances = self.load();
        instances.retain(|i| i.pane_id != pane_id);
        self.save(&instances)
    }
}
