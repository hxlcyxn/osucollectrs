use serde::{Deserialize, Serialize};

const COLLECTOR_BASE_URL: &str = "https://osucollector.com/api/collections";
const MIRROR_BASE_URL: &str = "https://api.chimu.moe/v1/download";

#[derive(Debug, Serialize, Deserialize)]
struct CollectorResponse {
    name: String,
    description: String,
    beatmapsets: Vec<BeatmapSet>,
}
#[derive(Debug, Serialize, Deserialize)]
struct BeatmapSet {
    id: usize,
    beatmaps: Vec<Beatmap>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Beatmap {
    id: usize,
}

fn print_usage() {
    println!("usage: osucollectrs [id]")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let collector_id = match args.len() {
        1 => {
            print_usage();
            std::process::exit(1)
        }
        2 => args[1].to_owned(),
        _ => {
            println!("Too many arguments!");
            print_usage();
            std::process::exit(1)
        }
    };

    if let Err(_) = &collector_id.parse::<usize>() {
        println!("Invalid osu!collector id! has to be a number");
        print_usage();
        std::process::exit(1)
    }

    let collector_url = format!("{}/{}", COLLECTOR_BASE_URL, collector_id);

    let collector_response = reqwest::get(&collector_url).await?;
    if !collector_response.status().is_success() {
        println!("Could not get osu!collector id! Are you sure it is correct?");
        print_usage();
        std::process::exit(1)
    }
    let collection: CollectorResponse = collector_response.json().await?;

    println!("# {}", &collection.name);
    println!("{}", &collection.description);
    println!("{}", &collection.beatmapsets[0].id);

    let mirror_response = reqwest::get(&format!(
        "{}/{}",
        MIRROR_BASE_URL, collection.beatmapsets[0].id
    ))
    .await?;
    if !mirror_response.status().is_success() {
        println!("Could not get map id! Are you sure it is correct?");
        print_usage();
        std::process::exit(1)
    }

    let re = regex::Regex::new(r#"filename="(.+)""#).unwrap();
    let content_disposition = mirror_response
        .headers()
        .get("content-disposition")
        .unwrap()
        .to_str()
        .unwrap();
    println!("{:?}", re.find(content_disposition));
    let beatmap_bytes = mirror_response.bytes().await?;

    Ok(())
}
