use crate::constants::Election;
use std::{env, error::Error, io::prelude::*, thread};
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

pub fn get_election() -> Election {
    let mut args = env::args();
    args.next();
    let election = args.next().unwrap_or_default().parse().unwrap_or_default();

    Election::get(election)
}

pub fn get_threads() -> usize {
    thread::available_parallelism().unwrap().get()
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
