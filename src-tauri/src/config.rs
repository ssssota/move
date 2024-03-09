use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[serde(tag = "version")]
#[ts(export)]
pub enum Config {
    V0 {
        source: Option<String>,
        target: Option<String>,
        pattern: Option<String>,
    },
}

impl Default for Config {
    fn default() -> Self {
        Config::V0 {
            source: None,
            target: None,
            pattern: None,
        }
    }
}
