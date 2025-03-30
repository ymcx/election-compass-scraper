use constants::Elections;
use futures::StreamExt;
use rand::Rng;
use std::{error::Error, ops::Range, time::Duration};
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    process::{Child, Command},
};

mod constants;
mod interaction;
mod scrape;

async fn driver() -> Result<(Child, WebDriver, String), Box<dyn Error>> {
    let port = rand::rng().random_range(1024..=u16::MAX);
    let directory = format!("/tmp/scraper-{port}");

    let child = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .arg("--log-level=OFF")
        .spawn()?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut capabilities = DesiredCapabilities::chrome();
    capabilities.add_arg("--headless")?;
    capabilities.add_arg(&format!("--user-data-dir={directory}"))?;
    let driver = WebDriver::new(format!("http://localhost:{port}"), capabilities).await?;

    Ok((child, driver, directory))
}

async fn save(content: &str, file: &str, append: bool) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .truncate(!append)
        .open(file)
        .await?;

    let content = format!("{content}\n");
    file.write(content.as_bytes()).await?;

    Ok(())
}

fn elections(argument: &str) -> Elections {
    match argument.to_uppercase().as_str() {
        "COUNTY25" => constants::county_elections_2025(),
        "EURO24" => panic!("Election data doesn't exist"),
        "MUNICIPAL25" => constants::municipal_elections_2025(),
        "PARLIAMENTARY23" => panic!("Election data doesn't exist"),
        "PRESIDENTIAL24" => panic!("Election data doesn't exist"),

        "COUNTY" => elections("COUNTY25"),
        "EURO" => elections("EURO24"),
        "MUNICIPAL" => elections("MUNICIPAL25"),
        "PARLIAMENTARY" => elections("PARLIAMENTARY23"),
        "PRESIDENTIAL" => elections("PRESIDENTIAL24"),

        _ => elections("MUNICIPAL"),
    }
}

fn threads(argument: &str) -> usize {
    argument.parse().unwrap_or(4)
}

fn urls(range: &Vec<Range<u16>>, url: &str) -> Vec<String> {
    range
        .iter()
        .flat_map(|range| range.clone().map(|i| format!("{url}{i}")))
        .collect()
}

async fn process_url(url: &str, file: &str, questions: usize) -> Result<(), Box<dyn Error>> {
    let (mut child, driver, directory) = driver().await?;

    let content = scrape::municipality(&driver, url, questions).await;
    save(&content.join("\n"), file, true).await?;

    driver.quit().await?;
    child.kill().await?;
    std::fs::remove_dir_all(directory)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let elections = elections(arguments.get(1).map(String::as_str).unwrap_or_default());
    let threads = threads(arguments.get(2).map(String::as_str).unwrap_or_default());
    let urls = urls(&elections.range, &elections.url);

    println!("Scraping {} with {} threads", elections.url, threads);

    save(&elections.headers, &elections.file, false)
        .await
        .map_err(|e| eprintln!("{e}"))
        .ok();

    futures::stream::iter(urls)
        .map(|url| {
            let file = elections.file.clone();
            let questions = elections.questions;
            async move {
                loop {
                    match process_url(&url, &file, questions).await {
                        Ok(_) => break,
                        Err(e) => eprintln!("{e}"),
                    }
                }
            }
        })
        .buffer_unordered(threads)
        .collect::<Vec<_>>()
        .await;
}
