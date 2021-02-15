use std::io;
use std::io::Write;
use serde_json::from_str;
use serde::Deserialize;
use termion::{color, style};
use std::process::Command;
use std::fmt;

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
        write!(f, "\"{}\"", self.title)
    }
}

/// Returns the videos found by searching for `query`
pub fn search(url: &String) -> Result<Vec<Video>, ureq::Error> {
    // TODO: pass config containing URL?
    let search_result_string: String = ureq::get(&url)
        .set("User-Agent", "Mozilla/5.0") .call()?.into_string()?;

    let videos: Vec<Video> = from_str(&search_result_string).unwrap();
    Ok(videos)
}

/// Displays a list of videos
pub fn print_videos(videos: &Vec<Video>) {
    // TODO: option to turn off color
    println!("{}{}Item\t Title{}", style::Bold, color::Fg(color::Yellow), style::Reset);
    for (i, video) in videos.iter().enumerate() {
        if i % 2 == 0 {
            println!("{}{}\t{}", color::Fg(color::Green), i+1, video);
        }
        else {
            println!("{}{}\t{}", color::Fg(color::Blue), i+1, video);
        }
    }

}

/// Selects a video
pub fn select_video(videos: &Vec<Video>) -> String {
    let mut video_number = String::new();

    print!("{}\nItem: {}", color::Fg(color::Yellow), color::Fg(color::Reset));
    io::stdout().flush().expect("Could not flush output");
    io::stdin().read_line(&mut video_number).expect("Could not read input");

    let selected_video = &videos[video_number.trim().parse::<usize>().expect("Could not parse input") - 1];
    format!("https://invidious.snopyta.org/watch?v={}", &selected_video.video_id)
}

/// Plays a video in the video player
pub fn play(url: &String) {
    // TODO: specify player in config or use xdg
    println!("Opening video, please wait...");
    Command::new("mpv").arg(url).output().expect("Could not open MPV");
}

// TODO: tests
