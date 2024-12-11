mod challenge;
mod error;
mod input;
mod select;

use std::sync::{Arc, Mutex};

use clap::{value_parser, Parser};
use itertools::Itertools;

use crate::challenge::solve_all;
use crate::error::Error;
use crate::input::{Download, InputFiles};
use crate::select::MultiChallengeSelector;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = value_parser!(MultiChallengeSelector))]
    parts: Vec<MultiChallengeSelector>,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {}", err);
    }
}

async fn run() -> Result<(), Error> {
    let args = Args::parse();
    let challenges = args.parts.into_iter().flatten().collect_vec();

    let input = InputFiles::new("input")?;
    {
        let downloader = Download::new(&input).await?;
        downloader.download_missing(&challenges).await?;
    }

    let input = Arc::new(Mutex::new(input));
    let solutions = solve_all(challenges, input.clone()).await;

    for solution in solutions {
        println!("\t{}", solution)
    }

    Ok(())
}
