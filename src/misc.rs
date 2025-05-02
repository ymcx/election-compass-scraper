use crate::constants::{self, Elections};
use std::cmp;
use std::io::prelude::*;
use std::{error::Error, ops::Range};
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

fn arg(i: usize) -> String {
    std::env::args()
        .collect::<Vec<_>>()
        .get(i)
        .unwrap_or(&String::default())
        .to_string()
}

pub fn elections() -> Elections {
    match arg(1).to_uppercase().as_str() {
        "COUNTY25" => constants::county_elections_2025(),
        "EUROPEAN24" => panic!("Election data doesn't exist"),
        "MUNICIPAL25" => constants::municipal_elections_2025(),
        "PARLIAMENTARY23" => panic!("Election data doesn't exist"),
        "PRESIDENTIAL24" => panic!("Election data doesn't exist"),

        "COUNTY" => constants::county_elections_2025(),
        "EUROPEAN" => panic!("Election data doesn't exist"),
        "MUNICIPAL" => constants::municipal_elections_2025(),
        "PARLIAMENTARY" => panic!("Election data doesn't exist"),
        "PRESIDENTIAL" => panic!("Election data doesn't exist"),

        _ => constants::municipal_elections_2025(),
    }
}

pub fn threads() -> usize {
    let cores = num_cpus::get();
    let default = cmp::min(cores / 2, 10);

    arg(2).parse().unwrap_or(default)
}

pub fn urls(range: &Vec<Range<usize>>, url: &str) -> Vec<String> {
    range
        .iter()
        .flat_map(|range| range.clone().map(|i| format!("{url}{i}")))
        .collect()
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
