mod constants;
mod driver;
mod interaction;
mod io;
mod scrape;

#[tokio::main]
async fn main() {
    let election = io::get_election();
    let threads = io::get_threads();

    println!("{}\n[{}]", constants::TAG, election.file);

    let candidates = scrape::scrape(&election.urls, election.questions, threads).await;
    if let Err(error) = io::save(&election.headers, &candidates, &election.file).await {
        io::print_error(&error);
    }
}
