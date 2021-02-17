use crate::types::Video;
use std::io::{self, Write};
use termion::color;

/// Selects a video
pub fn select_video<'a>(videos: &'a [Video]) -> &'a Video {
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

    &videos[video_number
        .trim()
        .parse::<usize>()
        .expect("Could not parse input")
        - 1]
}
