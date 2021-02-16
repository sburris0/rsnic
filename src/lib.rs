pub mod config;
pub mod invidious;
pub mod prompt;
pub mod search;

pub use crate::config::Config;
pub use crate::invidious::*;
use std::process::Command;

/// Plays a video in the video player
pub fn play(cfg: &Config, url: &str) {
    println!("Opening video, please wait...");
    Command::new(&cfg.player)
        .args(&cfg.player_args)
        .arg(url)
        .output()
        .expect("Could not open MPV");
}
