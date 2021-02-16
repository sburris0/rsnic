use rsnic::*;
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
