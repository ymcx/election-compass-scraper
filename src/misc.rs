use crate::constants::{self, Elections};
use rand::Rng;
use std::{error::Error, ops::Range};
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    process::{Child, Command},
    time::Duration,
};

pub async fn driver() -> Result<(Child, WebDriver, String), Box<dyn Error>> {
    let port = rand::rng().random_range(1024..65536);
    let directory = format!("/tmp/scraper-{port}");

    let child = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .arg("--silent")
        .spawn()?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut capabilities = DesiredCapabilities::chrome();
    capabilities.add_arg("--headless")?;
    capabilities.add_arg(&format!("--user-data-dir={directory}"))?;
    let driver = WebDriver::new(format!("http://localhost:{port}"), capabilities).await?;

    Ok((child, driver, directory))
}

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
    arg(2).parse().unwrap_or(4)
}

pub fn urls(range: &Vec<Range<usize>>, url: &str) -> Vec<String> {
    range
        .iter()
        .flat_map(|range| range.clone().map(|i| format!("{url}{i}")))
        .collect()
}
