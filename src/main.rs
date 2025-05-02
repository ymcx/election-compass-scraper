mod constants;
mod driver;
mod interaction;
mod io;
mod scrape;

#[tokio::main]
async fn main() {
    let (election, threads) = io::args();

    println!(
        "{}\nFILE\t[{}]\nTHREADS\t[{}]\n",
        constants::TAG,
        election.file,
        threads
    );

    let candidates = scrape::scrape(&election.urls, election.questions, threads).await;
    let _ = io::save(&election.headers, &candidates, &election.file)
        .await
        .map_err(|e| io::print_error(&e));
}
