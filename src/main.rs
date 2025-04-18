mod constants;
mod driver;
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
    let _ = misc::save(&elections.headers, &candidates, &elections.file)
        .await
        .map_err(|e| misc::print_error(&e));
}
