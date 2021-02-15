mod lib;

use crate::lib::*;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Really Simple Invidious Client")
        .version("0.1.0")
        .about("Spook-free YouTube video viewing")
        .author("Spencer B.")
        .subcommand(
            SubCommand::with_name("search")
                .about("Search for a specific video")
                .arg(
                    Arg::with_name("query")
                        .help("What to search for")
                        .required(true),
                ),
        )
        .get_matches();

    let cfg: Config = confy::load("rsnic").expect("Error loading configuration.");

    if let Some(ref matches) = matches.subcommand_matches("search")
    {
        let query = matches.value_of("query").unwrap();
        let videos: Vec<Video> =
            search(&cfg, query).expect("An error occurred, no videos were found.");
        print_videos(&cfg, &videos).expect("Could not display videos");

        let selected_url = select_video(&cfg, &videos);
        play(&cfg, &selected_url);
    }
}
