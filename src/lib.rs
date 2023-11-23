use clap::Parser;
use chrono::Utc;

pub mod date;
pub mod error;

use crate::error::Error;
use crate::date::{get_bounding_times, parse_and_clamp};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    start: Option<String>,

    #[arg(short, long)]
    end: Option<String>
}

pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    let now = Utc::now();
    let (min, max) = get_bounding_times(&now);

    let start = match cli.start.as_deref() {
        None => Ok(min),
        Some(s) => parse_and_clamp(min, max, s)
    }?;


    let end = match cli.end.as_deref() {
        None => Ok(max),
        Some(s) => parse_and_clamp(min, max, s)
    }?;

    println!("Downloading files between {:?} and {:?}", start, end);

    Ok(())
}
