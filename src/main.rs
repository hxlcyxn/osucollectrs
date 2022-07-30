use std::error::Error;

use osucollectrs::{Mirror, OsuCollectrs};

fn print_usage() {
    println!("usage: osucollectrs [id]")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let collector_id = match args.len() {
        2 => args[1].parse::<usize>().expect("Valid osu!collector id!"),
        _ => {
            print_usage();
            std::process::exit(1)
        }
    };

    let client = reqwest::Client::new();
    let collectr = OsuCollectrs::new(client, Mirror::Chimu);

    if let Err(_) = collectr.run(collector_id).await {
        println!("Are you sure that the id is correct");
        print_usage();
        std::process::exit(1)
    }

    Ok(())
}
