use std::collections::HashMap;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;

use reqwest::{self, Url};
use tokio::fs::{read_to_string, write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::input::{Error, Result};

const BASE_URI: &'static str = "https://adventofcode.com/2024";
const SESSION_FILE: &'static str = ".session";

pub struct Input {
    client: reqwest::Client,
    session: Option<String>,
    cache: HashMap<usize, String>,
}

impl Input {
    pub async fn new() -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            session: None,
            cache: HashMap::new(),
        }
    }

    pub async fn get_input(&mut self, day: usize) -> Result<&str> {
        if let Some(input) = self.cache.get(&day) {
            return Ok(input)
        }

        let input = self.read_or_download_input(day).await?;
        self.cache.insert(day, input);

        Ok(self.cache.get(&day).expect("cached input"))
    }

    async fn read_or_download_input(&mut self, day: usize) -> Result<String> {
        let result = self.read_input(day).await;

        if let Err(Error::NoInputFile(_)) = result {
            self.download_input(day).await?;
            self.read_input(day)
        } else {
            result
        }
    }

    async fn download_input(&mut self, day: usize) -> Result<()> {
        if self.session.is_none() {
            self.load_session().await?;
        }

        let session = self.session.as_ref().ok_or(Error::NoSessionFile)?;

        let mut url = Url::parse(BASE_URI)?;
        url.path_segments_mut()?
            .push("day")
            .push(&day.to_string())
            .push("input");

        let response = self
            .client
            .get(url)
            .header("Cookie", format!("session={session}"))
            .send()
            .await?
            .bytes()
            .await?;

        write(input_path(day), &response).await?;

        Ok(())
    }

    async fn load_session(&mut self) -> Result<()> {
        use std::io::ErrorKind::*;

        let session = read_to_string(SESSION_FILE).await
            .map_err(|err| match err.kind() {
                NotFound => Error::NoSessionFile,
                _ => Error::IOError(err),
            })
            .map(|session| session.trim().to_string())?;

        self.session = Some(session);

        Ok(())
    }

    async fn read_input(&self, day: usize) -> Result<String> {
        use std::io::ErrorKind::*;

        let path = input_path(day);
        read_to_string(&path).await
            .map_err(|err| match err.kind() {
                NotFound => Error::NoInputFile(path),
                _ => Error::IOError(err),
            })
            .map(|session| session.trim().to_string())
    }
}

fn input_path(day: usize) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(".");
    path.push("input");
    path.push(format!("day{}", day));

    path
}
