use std::path::PathBuf;

use itertools::Itertools;
use tokio::fs;
use tokio::task::JoinSet;
use url::Url;

use crate::input::input_files::InputFiles;
use crate::input::{Error, Input, Result};
use crate::select::Challenge;

const BASE_URI: &'static str = "https://adventofcode.com/2024";
const SESSION_FILE: &'static str = ".session";

#[derive(Debug)]
pub struct Download {
    input: InputFiles,
    client: reqwest::Client,
    session: String,
}

impl Download {
    pub async fn new(input: &InputFiles) -> Result<Self> {
        use std::io::ErrorKind::*;

        let client = reqwest::Client::builder().use_rustls_tls().build()?;

        let session = fs::read_to_string(SESSION_FILE)
            .await
            .map(|contents| contents.lines().next().unwrap_or_default().to_string())
            .map_err(|err| match err.kind() {
                NotFound => Error::NoSessionFile,
                _ => Error::IOError(err),
            })?;

        Ok(Self {
            input: input.clone(),
            client,
            session,
        })
    }

    pub async fn download_missing(&self, selection: &[Challenge]) -> Result<()> {
        let missing_days = selection
            .iter()
            .map(|challenge| challenge.day)
            .dedup()
            .filter(|day| !self.input.has_input(*day))
            .collect_vec();

        let mut join_set = JoinSet::new();
        for day in missing_days {
            let dl = InputDownload::new(self, day);
            join_set.spawn(dl.run());
        }

        join_set
            .join_all()
            .await
            .into_iter()
            .collect::<Result<Vec<()>>>()?;

        Ok(())
    }
}

struct InputDownload {
    day: usize,
    filepath: PathBuf,
    session: String,
    client: reqwest::Client,
}

impl InputDownload {
    fn new(parent: &Download, day: usize) -> Self {
        Self {
            day,
            filepath: parent.input.filepath(day),
            session: parent.session.clone(),
            client: parent.client.clone(),
        }
    }

    async fn run(self) -> Result<()> {
        let mut url = Url::parse(BASE_URI)?;
        url.path_segments_mut()?
            .push("day")
            .push(&self.day.to_string())
            .push("input");

        let req = self
            .client
            .get(url)
            .header("Cookie", format!("session={}", self.session))
            .build()?;

        let res = self.client.execute(req).await?.bytes().await?;

        dbg!(&self.filepath);
        fs::write(self.filepath, &res).await?;

        Ok(())
    }
}
