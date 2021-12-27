use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Mirror {
    base_url: String,
    download_url: String,
    search_url: String,
}

impl Mirror {
    fn new(base_url: &str, download_ext: &str, search_ext: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            download_url: base_url.to_string() + &download_ext.to_string(),
            search_url: base_url.to_string() + &search_ext.to_string(),
        }
    }
}

#[derive(Debug)]
enum Mirrors {
    Kitsu,
    Chimu,
}

impl Default for Mirrors {
    fn default() -> Self {
        Self::Kitsu
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct OsuMapset {
    set_id: usize,
    artist: String,
    title: String,
    creator: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_mirror = Mirrors::default();
    let mirror = match arg_mirror {
        Mirrors::Kitsu => Mirror::new("https://kitsu.moe/", "/d", "api/search"),
        Mirrors::Chimu => Mirror::new("https://api.chimu.moe/v1", "/download", "/search"),
    };

    let client = reqwest::Client::new();

    let res: Vec<OsuMapset> = client
        .get(format!("{}", mirror.search_url))
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", res);
    Ok(())
}
