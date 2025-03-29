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

async fn driver() -> Result<(Child, WebDriver), Box<dyn Error>> {
    let port = rand::rng().random_range(1024..65536);
    let child = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .arg("--log-level=OFF")
        .spawn()?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut capabilities = DesiredCapabilities::chrome();
    capabilities.set_headless()?;
    let driver = WebDriver::new(format!("http://localhost:{port}"), capabilities).await?;

    Ok((child, driver))
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

fn threads() -> usize {
    std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::default())
        .parse()
        .unwrap_or(4)
}

fn urls(range: &Vec<Range<u16>>, url: &str) -> Vec<String> {
    range
        .iter()
        .flat_map(|range| range.clone().map(|i| format!("{url}{i}")))
        .collect()
}

#[tokio::main]
async fn main() {
    let elections = constants::municipal_elections_2025();
    let urls = urls(&elections.range, &elections.url);
    let threads = threads();

    save(&elections.headers, &elections.file, false)
        .await
        .map_err(|e| eprintln!("{e}"))
        .ok();

    futures::stream::iter(urls)
        .map(|url| {
            let file = elections.file.clone();
            async move {
                let mut driver = loop {
                    if let Ok(driver) = driver().await {
                        break driver;
                    }
                };

                let content = scrape::municipality(&driver.1, &url, elections.questions).await;
                save(&content.join("\n"), &file, true)
                    .await
                    .map_err(|e| eprintln!("{e}"))
                    .ok();

                driver.1.quit().await.map_err(|e| eprintln!("{e}")).ok();
                driver.0.kill().await.map_err(|e| eprintln!("{e}")).ok();
            }
        })
        .buffer_unordered(threads)
        .collect::<Vec<_>>()
        .await;
}
