//! Pipeline report parsing.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PipelineReport {
    pub status: String,
    pub action: String,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub metrics: serde_json::Value,
}

impl PipelineReport {
    pub fn from_json(v: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(v)
    }

    pub fn is_ok(&self) -> bool {
        self.action == "ok" || self.action == "normal"
    }

    pub fn is_critical(&self) -> bool {
        self.action == "critical"
    }

    pub fn is_estop(&self) -> bool {
        self.action == "estop"
    }

    pub fn is_slow_down(&self) -> bool {
        self.action == "slow_down"
    }

    pub fn is_speed_up(&self) -> bool {
        self.action == "speed_up"
    }

    pub fn is_target(&self, target: &str) -> bool {
        self.target.as_deref() == Some(target)
    }

    pub fn success(&self) -> bool {
        matches!(self.action.as_str(), Some("ok" | "normal" | "clear"))
    }

    pub fn critical(&self) -> bool {
        matches!(self.action.as_str(), Some("critical" | "estop" | "slow_down"))
    }
}