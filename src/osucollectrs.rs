use std::{
    error::Error,
    path::{Path, PathBuf},
};

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorResponse {
    name: String,
    description: String,
    beatmapsets: Vec<BeatmapSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatmapSet {
    id: usize,
    beatmaps: Vec<Beatmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Beatmap {
    id: usize,
}

pub enum Mirror {
    Chimu,
    Kitsu,
}

impl Mirror {
    fn base_url(&self) -> Url {
        match self {
            Self::Chimu => Url::parse("https://api.chimu.moe/v1/").unwrap(),
            Self::Kitsu => Url::parse("https://kitsu.moe/api/").unwrap(),
        }
    }
    fn dl_url(&self) -> Url {
        match self {
            Self::Chimu => self.base_url().join("download/").unwrap(),
            Self::Kitsu => self.base_url().join("d/").unwrap(),
        }
    }
    fn search_url(&self) -> Url {
        match self {
            Self::Chimu => self.base_url().join("search/").unwrap(),
            Self::Kitsu => self.base_url().join("search/").unwrap(),
        }
    }
}

pub struct OsuCollectrs {
    client: Client,
    collector_url: Url,
    dl_url: Url,
    search_url: Url,
    dl_path: PathBuf,
}

impl OsuCollectrs {
    pub fn new(client: Client, mirror: Mirror) -> Self {
        Self {
            client,
            collector_url: Url::parse("https://osucollector.com/api/collections/").unwrap(),
            dl_url: mirror.dl_url(),
            search_url: mirror.search_url(),
            dl_path: Path::new("./maps").to_owned(),
        }
    }

    pub async fn get_collection(&self, id: usize) -> Result<CollectorResponse, Box<dyn Error>> {
        let url = self.collector_url.join(&id.to_string())?;
        let resp: CollectorResponse = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    pub async fn dl_beatmap(&self, id: usize) -> Result<(), Box<dyn Error>> {
        if !self.dl_path.exists() {
            std::fs::create_dir(&self.dl_path)?;
        }

        let url = self.dl_url.join(&id.to_string())?;
        let resp = self.client.get(url).send().await?;
        let content_disposition = resp
            .headers()
            .get("content-disposition")
            .unwrap()
            .to_str()?;

        // Expected format of content-disposition: `attachment; filename="FILENAME"`.
        let filename = &content_disposition[22..content_disposition.len() - 1]
            .trim()
            .to_owned();

        let beatmap_bytes = resp.bytes().await?;
        if Path::exists(&self.dl_path.join(filename)) {
            tokio::fs::remove_file(&self.dl_path.join(filename)).await?;
        }
        tokio::fs::write(self.dl_path.join(filename), beatmap_bytes).await?;
        Ok(())
    }

    pub async fn run(&self, id: usize) -> Result<(), Box<dyn Error>> {
        let collection = self.get_collection(id).await?;
        println!("# {}", collection.name);
        println!("{}", collection.description);
        println!("---");

        for beatmap_set in collection.beatmapsets {
            println!("{}", beatmap_set.id);
            self.dl_beatmap(beatmap_set.id).await?;
        }
        Ok(())
    }

    pub fn search(&self, query: &str) {
        todo!()
    }
}
