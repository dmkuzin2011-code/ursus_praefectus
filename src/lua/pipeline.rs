//! Pipeline report parsing.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PipelineReport {
    pub action: String,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub checks: serde_json::Value,
}

impl PipelineReport {
    pub fn from_json(v: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(v)
    }

    pub fn is_estop(&self) -> bool { self.action == "estop" }
    pub fn is_slow_down(&self) -> bool { self.action == "slow_down" }
}