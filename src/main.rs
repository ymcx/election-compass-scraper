mod constants;
mod interaction;
mod misc;
mod scrape;

#[tokio::main]
async fn main() {
    let elections = misc::elections();
    let threads = misc::threads();
    let urls = misc::urls(&elections.range, &elections.url);

    println!("Scraping {} with {} threads", elections.url, threads);

    if let Err(e) = misc::save(&elections.headers, &elections.file, false).await {
        eprintln!("{e}");
    }

    scrape::process_urls(&urls, &elections.file, elections.questions, threads).await;
}
