mod challenge;
mod error;
mod input;

use clap::Parser;

use challenge::ChallengePart;
use error::Error;

use crate::challenge::{run_all_challenges, run_challenge};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    day: Option<usize>,

    #[arg(value_enum)]
    parts: Vec<ChallengePart>,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {}", err);
    }
}

async fn run() -> Result<(), Error> {
    let args = Args::parse();
    let input_svc = input::Input::new().await?;
    let results = match args {
        Args { day: None, .. } => run_all_challenges(&input_svc).await?,
        Args {
            day: Some(d),
            parts,
        } => vec![run_challenge(d, parts, &input_svc).await?],
    };

    println!("results:");
    for result_set in results {
        println!("{:?}", result_set);
    }

    Ok(())
}
