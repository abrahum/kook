use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
}

impl Config {
    pub fn load_from_file() -> Self {
        serde_json::from_str(&std::fs::read_to_string("test.json").unwrap()).unwrap()
    }
}
