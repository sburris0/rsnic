use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process::Command;
use termion::{color, style};

/// Contains configuration options
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    instance: String,
    player: String,
    player_args: Vec<String>,
    results: u8,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            instance: "https://invidious.snopyta.org".to_string(),
            player: "mpv".to_string(),
            player_args: Vec::new(),
            results: 20,
        }
    }
}

/// Video type that API responses are parsed into
/// The Invidious API is documented [here](https://github.com/iv-org/documentation/blob/master/API.md).
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub title: String,
    pub video_id: String,
    pub author: String,
    pub description: String,
    pub published: i64,
    pub length_seconds: i32,
}

impl fmt::Display for Video {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = self.length_seconds / 60;
        let seconds = self.length_seconds % 60;
        write!(
            f,
            "\"{}\" ({}:{:0>2}) - {}",
            self.title, minutes, seconds, self.author
        )
    }
}

/// Returns the videos found by searching for `query`
pub fn search(cfg: &Config, query: &str) -> Result<Vec<Video>, Box<dyn Error>> {
    let url = format!("{}/api/v1/search?q={}", cfg.instance, query);
    let search_result_string: String = ureq::get(&url)
        .set("User-Agent", "Mozilla/5.0")
        .call()?
        .into_string()?;

    Ok(from_str(&search_result_string)?)
}

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

/// Selects a video
pub fn select_video(cfg: &Config, videos: &[Video]) -> String {
    let mut video_number = String::new();

    print!(
        "{}\nItem: {}",
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    );
    io::stdout().flush().expect("Could not flush output");
    io::stdin()
        .read_line(&mut video_number)
        .expect("Could not read input");

    let selected_video = &videos[video_number
        .trim()
        .parse::<usize>()
        .expect("Could not parse input")
        - 1];
    format!("{}/watch?v={}", cfg.instance, &selected_video.video_id)
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

// TODO: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        // Only get one result
        let mut cfg = Config::default();
        cfg.results = 1;

        let results = search(&cfg, "Max Stirner Complete Works Audio Book").unwrap();
        let top_result = &results.first().unwrap();
        assert_eq!(top_result.video_id, "MmdB8R9sm4Y".to_string());
        assert_eq!(
            top_result.title,
            "Max Stirner Complete Works Audio Book".to_string()
        );
    }
}
