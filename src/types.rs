use crate::config::Config;
use crate::timestamp::sec_to_hms;
use serde::Deserialize;
use std::fmt;
use std::process::Command;
use termion::{color, style};

/// Video type that API responses are parsed into
/// The Invidious API is documented [here](https://github.com/iv-org/documentation/blob/master/API.md).
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Content {
    #[serde(rename_all = "camelCase")]
    Video {
        title: String,
        video_id: String,
        author: String,
        description: String,
        published: i64,
        length_seconds: i32,
        view_count: i64,
    },
    #[serde(rename_all = "camelCase")]
    Channel {
        author: String,
        sub_count: i32,
        video_count: i32,
        description: String,
    },
    #[serde(rename_all = "camelCase")]
    Playlist {
        title: String,
        playlist_id: String,
        author: String,
        video_count: i32,
        // videos: Vec<Video>,
    },
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Video {
                length_seconds,
                author,
                title,
                ..
            } => {
                let timestamp = sec_to_hms(length_seconds);
                write!(f, "\"{}\" ({}) - {}", title, timestamp, author)
            }
            Content::Channel { author, .. } => {
                write!(f, "CHANNEL: {}", author)
            }
            Content::Playlist { title, author, .. } => {
                write!(f, "PLAYLIST: \"{}\" - {}", title, author)
            }
        }
    }
}

// TODO: play, download, subscribe traits
// pub fn download(&self, _cfg: Config) {
//     unimplemented!()
// }

impl Content {
    pub fn play(&self, cfg: Config) {
        match self {
            Content::Video { video_id, .. } => {
                println!("Opening video, please wait...");
                Command::new(&cfg.player)
                    .args(&cfg.player_args)
                    .arg(format!("{}/watch?v={}", &cfg.instance, video_id))
                    .output()
                    .expect("Couldn't display a video");
            }
            _ => unimplemented!(),
        }
    }
}

/// A collection of things Videos, Playlists, Channels
pub struct Collection(pub Vec<Content>);

/// Collections display in a table
impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,
            "{}{}│Item\t Title\n├─────────────────────────────────────────────────────────────────────{}",
            style::Bold,
            color::Fg(color::Yellow),
            style::Reset
        )?;
        for (i, item) in self.0.iter().enumerate() {
            write!(f, "{}│", color::Fg(color::Yellow))?;

            if i % 2 == 0 {
                writeln!(f, "{}{}\t{}", color::Fg(color::Green), i + 1, item)?;
            } else {
                writeln!(f, "{}{}\t{}", color::Fg(color::Blue), i + 1, item)?;
            }
        }

        writeln!(
            f,
            "{}{}└─────────────────────────────────────────────────────────────────────{}",
            style::Bold,
            color::Fg(color::Yellow),
            style::Reset
        )
    }
}
