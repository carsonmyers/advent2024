pub mod error;

use std::path::{Path, PathBuf};

use reqwest::{self, Url};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

const BASE_URI: &'static str = "https://adventofcode.com/2022";
const SESSION_FILE: &'static str = ".session";

pub struct Input {
    client: reqwest::Client,
    session: String,
}

impl Input {
    pub async fn new() -> Result<Self> {
        let session = read_file(SESSION_FILE).await?;
        let client = reqwest::Client::new();

        Ok(Input { client, session })
    }

    pub async fn get_input(&self, day: usize) -> Result<Vec<String>> {
        if !input_path(day).as_path().exists() {
            self.download_input(day).await?;
        }

        get_input_from_file(day).await
    }

    pub async fn download_input(&self, day: usize) -> Result<()> {
        let mut url = Url::parse(BASE_URI)?;
        url.path_segments_mut()?
            .push("day")
            .push(&day.to_string())
            .push("input");

        let response = self
            .client
            .get(url)
            .header("Cookie", format!("session={}", self.session))
            .send()
            .await?
            .bytes()
            .await?;

        let mut file = File::create(input_path(day).as_path()).await?;
        file.write_all(response.as_ref()).await?;

        Ok(())
    }
}

async fn get_input_from_file(day: usize) -> Result<Vec<String>> {
    Ok(read_file(input_path(day).as_path())
        .await?
        .lines()
        .map(String::from)
        .collect())
}

async fn read_file<T: AsRef<Path>>(path: T) -> Result<String> {
    let mut file = File::open(path.as_ref()).await?;
    let mut raw_contents = vec![];
    file.read_to_end(&mut raw_contents).await?;

    let contents = String::from_utf8(raw_contents)
        .map_err(|_| Error::Utf8Error(path.as_ref().to_path_buf()))?;

    Ok(contents)
}

fn input_path(day: usize) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(".");
    path.push("input");
    path.push(format!("day{}", day));

    path
}
