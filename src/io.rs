use crate::constants::Election;
use clap::Parser;
use std::{cmp, error::Error, io::prelude::*};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn save(
    headers: &str,
    candidates: &Vec<String>,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::default();
    buffer.push_str(headers);
    buffer.push('\n');
    for candidate in candidates {
        buffer.push_str(candidate);
        buffer.push('\n');
    }

    let mut file = File::create(filename).await?;
    file.write_all(buffer.as_bytes()).await?;

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    election: Option<String>,
    #[arg(short, long)]
    jobs: Option<usize>,
    #[arg(short, long)]
    year: Option<usize>,
}

pub fn args() -> (Election, usize) {
    let args = Args::parse();
    let election = Election::get(args.election, args.year);
    let jobs = args.jobs.unwrap_or(cmp::min(num_cpus::get() / 2, 10));

    (election, jobs)
}

pub fn print_error(error: &Box<dyn Error>) {
    print!(" [{}]", error);
    let _ = std::io::stdout().flush();
}

pub fn print_progress(current: usize, length: usize) {
    print!("\x1B[2K\r");
    print!("[{:-<19}]", "#".repeat(current * 20 / length));
    let _ = std::io::stdout().flush();
}
