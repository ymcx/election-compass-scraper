mod constants;
mod driverprocess;
mod interaction;
mod misc;
mod scrape;

#[tokio::main]
async fn main() {
    let elections = misc::elections();
    let threads = misc::threads();
    let urls = misc::urls(&elections.range, &elections.url);

    println!(
        "{}\nURL\t[{}]\nTHREADS\t[{}]\n",
        constants::TAG,
        elections.url,
        threads
    );

    let candidates = scrape::scrape(&urls, elections.questions, threads).await;
    misc::save(&elections.headers, &candidates, &elections.file)
        .await
        .map_err(|e| eprintln!("{e}"))
        .ok();
}
