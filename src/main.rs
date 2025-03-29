use futures::StreamExt;
use std::{ops::Range, time::Duration};
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    process::{Child, Command},
};

mod constants;
mod interaction;
mod scrape;

async fn driver(port: u16) -> (Child, WebDriver) {
    let child = Command::new("geckodriver")
        .arg(format!("--port={port}"))
        .spawn()
        .unwrap();

    tokio::time::sleep(Duration::from_millis(500)).await;

    let mut capabilities = DesiredCapabilities::firefox();
    capabilities.set_headless().unwrap();
    let driver = WebDriver::new(format!("http://localhost:{port}"), capabilities)
        .await
        .unwrap();

    (child, driver)
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
    let arguments: Vec<String> = std::env::args().collect();
    arguments.get(1).unwrap().parse().unwrap()
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
                let mut driver = driver(port).await;

                let content = scrape::municipality(&driver.1, &url, elections.questions).await;
                save(&content.join("\n"), &file, true).await;

                driver.1.quit().await.unwrap();
                driver.0.kill().await.unwrap();
            }
        })
        .buffer_unordered(threads)
        .collect::<Vec<_>>()
        .await;
}
