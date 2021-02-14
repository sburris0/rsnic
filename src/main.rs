#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, SubCommand};
use ureq;
use serde_json::from_str;
use std::io;
use std::io::Write;
use std::process::Command;
use termion::{color, style};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Video {
    title: String,
    video_id: String,
    author: String,
    description: String,
    published: i64,
    length_seconds: i32,
}

fn main() {
    let matches = App::new("Really Simple Invidious Client")
        .version("0.1.0")
        .about("Spook-free YouTube video viewing")
        .author("Spencer B.")
        .subcommand(SubCommand::with_name("search")
            .about("Search for a specific video")
            .arg(Arg::with_name("query")
                .help("What to search for")
                .required(true)))
        .get_matches();

    if let Some(query) = matches.subcommand_matches("search").unwrap().value_of("query") {
        let req_url = format!("https://invidious.snopyta.org/api/v1/search?q={}", query);

        let search_result_string: String = ureq::get(&req_url)
            .set("User-Agent", "Mozilla/5.0")
            .call().unwrap()
            .into_string().unwrap();

        let videos: Vec<Video> = from_str(&search_result_string).unwrap();

        println!("{}{}Item\t Title{}", style::Bold, color::Fg(color::Yellow), style::Reset);
        for (i, video) in videos.iter().enumerate() {
            if i % 2 == 0 {
                println!("{}{}\t{}", color::Fg(color::Green), i+1, video.title);
            }
            else {
                println!("{}{}\t{}", color::Fg(color::Blue), i+1, video.title);
            }
        }

        let mut video_number = String::new();
        print!("{}\nItem: {}", color::Fg(color::Yellow), color::Fg(color::Reset));
        io::stdout().flush().expect("Could not flush output");
        io::stdin().read_line(&mut video_number).expect("Could not read input");

        let selected_video = &videos[video_number.trim().parse::<usize>().expect("Could not parse input") - 1];
        let selected_url = format!("https://invidious.snopyta.org/watch?v={}", &selected_video.video_id);

        println!("Opening video, please wait...");
        Command::new("mpv").arg(&selected_url).output().expect("Could not open MPV");
    }
}
