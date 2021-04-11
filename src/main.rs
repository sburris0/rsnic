use rsnic::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rsnic",
    about = "RuSt Simple Neat Invidious Client",
    author = "Spencer B."
)]
struct Opt {
    #[structopt(required = true)]
    query: String,
}

fn main() {
    let cfg: rsnic::Config = confy::load("rsnic").expect("Error loading configuration.");
    let opt = Opt::from_args();

    // Search and display results
    let results = search::search(&cfg, &opt.query).expect("No videos found.");
    println!("{}", Collection(results.clone()));

    // Open
    loop {
        if let Ok((_cmd, index)) = prompt::prompt() {
            // index-1 to match non-zero indexed videos in list
            let selected = &results[index - 1];
            match selected {
                // TODO: match opt { Download => download(), player => play() }...
                Content::Video { .. } => selected.play(cfg),
                _ => (),
            }
            break;
        }
    }
}
