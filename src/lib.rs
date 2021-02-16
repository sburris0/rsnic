pub mod config;
pub mod invidious;
pub mod prompt;
pub mod search;

pub use crate::config::Config;
pub use crate::invidious::*;
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;
use termion::{color, style};

/// Displays a list of videos
pub fn print_videos(cfg: &Config, videos: &[Video]) -> Result<(), Box<dyn Error>> {
    let mut writer = io::BufWriter::new(io::stdout());

    writeln!(writer,
        "{}{}│Item\t Title\n├─────────────────────────────────────────────────────────────────────{}",
        style::Bold,
        color::Fg(color::Yellow),
        style::Reset
    )?;
    for (i, video) in videos.iter().enumerate() {
        if i < cfg.results as usize {
            write!(writer, "{}│", color::Fg(color::Yellow))?;

            if i % 2 == 0 {
                writeln!(writer, "{}{}\t{}", color::Fg(color::Green), i + 1, video)?;
            } else {
                writeln!(writer, "{}{}\t{}", color::Fg(color::Blue), i + 1, video)?;
            }
        }
    }

    writeln!(
        writer,
        "{}{}└─────────────────────────────────────────────────────────────────────{}",
        style::Bold,
        color::Fg(color::Yellow),
        style::Reset
    )?;
    writer.flush()?;
    Ok(())
}

/// Plays a video in the video player
pub fn play(cfg: &Config, url: &str) {
    println!("Opening video, please wait...");
    Command::new(&cfg.player)
        .args(&cfg.player_args)
        .arg(url)
        .output()
        .expect("Could not open MPV");
}
