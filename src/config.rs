use serde::{Deserialize, Serialize};

/// Contains configuration options
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub instance: String,
    pub player: String,
    pub player_args: Vec<String>,
    pub results: u8,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            instance: String::from("https://invidious.snopyta.org"),
            player: String::from("mpv"),
            player_args: Vec::new(),
            results: 19,
        }
    }
}
