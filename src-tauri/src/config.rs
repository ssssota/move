use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[serde(tag = "version")]
#[ts(export)]
pub enum Config {
    V0 {
        source: String,
        target: String,
        pattern: String,
    },
}

impl Default for Config {
    fn default() -> Self {
        Config::V0 {
            source: "".to_string(),
            target: "".to_string(),
            pattern: "".to_string(),
        }
    }
}
