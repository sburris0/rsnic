mod lib;

use crate::lib::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rsnic",
    about = "RuSt Simple Neat Invidious Client",
    author = "Spencer B."
)]
enum Opt {
    Play { query: String },
    Download { query: String },
    Subscribe { query: String },
    List { query: String },
}

fn main() {
    let cfg: Config = confy::load("rsnic").expect("Error loading configuration.");
    let opt = Opt::from_args();
    let mut videos: Vec<Video> = Vec::new();

    // Search
    match opt {
        Opt::Play { query } => {
            videos.append(
                &mut search(&cfg, &query).expect("An error occurred, no videos were found."),
            );
        }
        _ => (),
    }

    // Open
    // match opt { Download => download(), player => play() }...
    print_videos(&cfg, &videos).expect("Could not display videos");
    let selected_url = select_video(&cfg, &videos);
    play(&cfg, &selected_url);
}
