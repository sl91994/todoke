use serde::{Deserialize, Serialize};

/// 深刻度
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}
