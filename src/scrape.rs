use crate::{driver::Driver, interaction};
use futures::StreamExt;
use std::error::Error;
use thirtyfour::WebDriver;

async fn candidate_urls_gender(
    driver: &WebDriver,
    gender: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    interaction::click_gender_checkbox(driver, gender).await;
    let urls = candidate_urls(driver).await;
    interaction::click_gender_checkbox(driver, gender).await;

    urls
}

async fn candidate_urls(driver: &WebDriver) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls: Vec<String> = Vec::new();
    for a in interaction::elements_a(driver).await {
        let href = a.attr("href").await.unwrap_or_default().unwrap_or_default();
        if href.contains("/ehdokkaat/") {
            urls.push(href);
        }
    }

    let mut amount = 0;
    for button in interaction::elements_buttons(driver).await {
        let text = button.text().await.unwrap_or_default();
        if text.contains("Näytä (") {
            amount = text
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap_or_default();
            break;
        }
    }

    if urls.len() != amount {
        let message = format!(
            "Amount of scraped urls ({}) doesn't match the amount of candidates present ({})",
            urls.len(),
            amount
        );
        return Err(message.into());
    }

    Ok(urls)
}

async fn candidate_info(driver: &WebDriver) -> String {
    let (keys, vals) = interaction::elements_info(driver).await;
    let extract = [
        "Puolue",
        "Kotikunta",
        "Koulutus",
        "Syntymävuosi",
        "Äidinkieli",
        "Kielitaito",
    ];

    let mut info = vec![String::default(); extract.len()];
    for (keyindex, key) in keys.iter().enumerate() {
        let keytext = key.text().await.unwrap_or_default();
        if let Some(extractindex) = extract.iter().position(|&i| i == keytext) {
            let valtext = vals[keyindex].text().await.unwrap_or_default();
            info[extractindex] = valtext;
        }
    }

    info.join(";")
}

async fn candidate_answers(driver: &WebDriver, questions: usize) -> String {
    let mut answers = vec![String::default(); questions];
    for (q, question) in interaction::elements_questions(driver)
        .await
        .iter()
        .take(questions)
        .enumerate()
    {
        for (o, option) in interaction::elements_options(question)
            .await
            .iter()
            .enumerate()
        {
            if option.attr("imgurl").await.unwrap_or_default().is_none() {
                continue;
            }

            answers[q] = o.to_string();
            break;
        }
    }

    answers.join(";")
}

async fn candidate(
    driver: &WebDriver,
    url_relative: &str,
    gender: &str,
    questions: usize,
) -> Result<String, Box<dyn Error>> {
    let url = format!("https://vaalit.yle.fi{url_relative}");
    interaction::goto(driver, &url).await;
    interaction::click_show_more(driver, false).await;

    let name = interaction::text_name(driver).await;
    if name.is_empty() {
        let message = format!("Scraping of {url} was unsuccessful");
        return Err(message.into());
    }

    let info = candidate_info(driver).await;
    let answers = candidate_answers(driver, questions).await;
    let candidate = format!("{name};{info};{gender};{answers}");

    Ok(candidate)
}

async fn municipality(url: &str, questions: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let drivers = Driver::new().await;
    let driver = drivers.driver().ok_or("Driver is None")?;

    interaction::goto(driver, url).await;
    interaction::click_accept_cookies(driver).await;
    interaction::click_show_more(driver, true).await;
    interaction::click_gender_button(driver).await;

    let genders = ["", "female", "male", "other"];
    let mut urls = (
        candidate_urls_gender(driver, genders[0]).await?,
        candidate_urls_gender(driver, genders[1]).await?,
        candidate_urls_gender(driver, genders[2]).await?,
        candidate_urls_gender(driver, genders[3]).await?,
    );
    urls.0
        .retain(|u| !urls.1.contains(u) && !urls.2.contains(u) && !urls.3.contains(u));

    let mut municipality: Vec<String> = Vec::new();
    for (i, urls) in [&urls.0, &urls.1, &urls.2, &urls.3].into_iter().enumerate() {
        for url in urls {
            let candidate = loop {
                match candidate(driver, url, genders[i], questions).await {
                    Ok(candidate) => break candidate,
                    Err(e) => eprintln!("{e}"),
                }
            };
            municipality.push(candidate);
        }
    }

    Ok(municipality)
}

pub async fn scrape(urls: &Vec<String>, questions: usize, threads: usize) -> Vec<String> {
    let mut candidates = futures::stream::iter(urls)
        .map(|url| async move {
            loop {
                match municipality(url, questions).await {
                    Ok(candidates) => break candidates,
                    Err(e) => eprintln!("{e}"),
                }
            }
        })
        .buffer_unordered(threads)
        .collect::<Vec<Vec<_>>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    candidates.sort();
    candidates
}
