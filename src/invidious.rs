use serde::Deserialize;
use termion::{color, style};
use std::fmt;

trait ContentItem{}

// TODO: ContentItem trait, impl Display for Vec<T: ContentItem>
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

pub struct VideoCollection<'a>(pub &'a Vec<Video>);

impl fmt::Display for VideoCollection<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,
            "{}{}│Item\t Title\n├─────────────────────────────────────────────────────────────────────{}",
            style::Bold,
            color::Fg(color::Yellow),
            style::Reset
        )?;
        for (i, video) in self.0.iter().enumerate() {
            write!(f, "{}│", color::Fg(color::Yellow))?;

            if i % 2 == 0 {
                writeln!(f, "{}{}\t{}", color::Fg(color::Green), i + 1, video)?;
            } else {
                writeln!(f, "{}{}\t{}", color::Fg(color::Blue), i + 1, video)?;
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

pub struct Playlist;
pub struct Channel;
