mod lib;

use clap::{App, Arg, SubCommand};
use crate::lib::*;

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
        // TODO: move search url to search()
        let req_url = format!("https://invidious.snopyta.org/api/v1/search?q={}", query);
        let videos: Vec<Video> = search(&req_url).expect("An error occurred, no videos were found.");
        print_videos(&videos);

        let selected_url = select_video(&videos);
        play(&selected_url);
    }
}
