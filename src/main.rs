use rsnic::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rsnic",
    about = "RuSt Simple Neat Invidious Client",
    author = "Spencer B."
)]
enum Opt {
    #[structopt(about = "Search for and play a video")]
    Play { query: String },

    #[structopt(about = "Search for and download a video")]
    Download { query: String },

    #[structopt(about = "Search for and subscribe to a channel")]
    Subscribe { query: String },

    #[structopt(about = "List your subscriptions' videos")]
    List { query: String },
}

fn main() {
    let cfg: rsnic::Config = confy::load("rsnic").expect("Error loading configuration.");
    let opt = Opt::from_args();
    let mut videos: Vec<Video> = Vec::new();

    // Search
    match opt {
        Opt::Play { query } => {
            videos.append(
                &mut search::search(&cfg, &query)
                    .expect("An error occurred, no videos were found."),
            );
        }
        _ => (),
    }

    // Open
    // match opt { Download => download(), player => play() }...
    println!("{}", VideoCollection(&videos[..cfg.results as usize]));
    let selected_url = prompt::select_video(&cfg, &videos);
    play(&cfg, &selected_url);
}
