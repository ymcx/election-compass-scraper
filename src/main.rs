use futures::StreamExt;
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

async fn driver(port: u16) -> Result<(Child, WebDriver), Box<dyn Error>> {
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

async fn save(content: &str, file: &str, append: bool) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .truncate(!append)
        .open(file)
        .await
        .unwrap();

    let content = format!("{content}\n");
    file.write(content.as_bytes()).await.unwrap();
}

fn threads() -> usize {
    std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("4"))
        .parse()
        .unwrap()
}

fn urls(range: &Vec<Range<u16>>, baseurl: &str) -> Vec<(String, u16)> {
    range
        .iter()
        .flat_map(|range| {
            range.clone().map(|i| {
                let url = format!("{baseurl}{i}");
                let port = 32768 + i;
                (url, port)
            })
        })
        .collect()
}

#[tokio::main]
async fn main() {
    let elections = constants::municipal_elections_2025();
    let urls = urls(&elections.range, &elections.url);
    let threads = threads();

    save(&elections.headers, &elections.file, false).await;

    futures::stream::iter(urls)
        .map(|(url, port)| {
            let file = elections.file.clone();
            async move {
                let mut driver = loop {
                    if let Ok(driver) = driver(port).await {
                        break driver;
                    }
                };

                let content = scrape::municipality(&driver.1, &url, elections.questions).await;
                save(&content.join("\n"), &file, true).await;

                let _ = driver.1.quit().await;
                let _ = driver.0.kill().await;
            }
        })
        .buffer_unordered(threads)
        .collect::<Vec<_>>()
        .await;
}
