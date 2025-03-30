use crate::{interaction, misc};
use futures::StreamExt;
use std::error::Error;
use thirtyfour::{By, WebDriver, prelude::ElementQueryable};

async fn candidate_urls_gender(driver: &WebDriver, gender: &str) -> Vec<String> {
    interaction::click(driver, By::XPath(format!("//input[@value='{gender}']"))).await;
    let urls = candidate_urls(driver).await;
    interaction::click(driver, By::XPath(format!("//input[@value='{gender}']"))).await;

    urls
}

async fn candidate_urls(driver: &WebDriver) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    for element in interaction::elements(driver, By::Tag("a")).await {
        let href = element
            .attr("href")
            .await
            .unwrap_or_default()
            .unwrap_or_default();
        if href.contains("/ehdokkaat/") {
            urls.push(href);
        }
    }

    urls
}

async fn candidate_info(driver: &WebDriver) -> String {
    let keys = interaction::elements(driver, By::ClassName("sc-fxLEUo")).await;
    let vals = interaction::elements(driver, By::ClassName("sc-cDCfkV")).await;
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
    let elements = interaction::elements(driver, By::ClassName("sc-bRilDX")).await;

    for (i, element) in elements.iter().take(questions).enumerate() {
        let options = element
            .query(By::ClassName("sc-kuCIbt"))
            .all_from_selector()
            .await
            .unwrap_or_default();

        for (j, option) in options.iter().enumerate() {
            if option.attr("imgurl").await.unwrap_or_default().is_some() {
                answers[i] = j.to_string();
                break;
            }
        }
    }

    answers.join(";")
}

async fn candidate(
    driver: &WebDriver,
    url: &str,
    gender: &str,
    questions: usize,
) -> Result<String, Box<dyn Error>> {
    interaction::goto(driver, &format!("https://vaalit.yle.fi{url}")).await;
    interaction::click(driver, By::XPath("//button[@aria-label='Näytä lisää']")).await;

    let name = interaction::element(driver, By::ClassName("sc-xyPcs")).await;
    let info = candidate_info(driver).await;
    let answers = candidate_answers(driver, questions).await;
    let candidate = format!("{name};{info};{gender};{answers}");

    if candidate.chars().filter(|&c| c == ';').count() != 32 {
        return Err("Scraping of candidate was unsuccessful".into());
    }

    Ok(candidate)
}

async fn municipality(driver: &WebDriver, url: &str, questions: usize) -> String {
    interaction::goto(driver, url).await;
    interaction::click(
        driver,
        By::XPath("//button[@aria-label='Vain välttämättömät']"),
    )
    .await;
    while interaction::click(driver, By::XPath("//button[@aria-label='Näytä lisää']")).await {}
    interaction::click(driver, By::XPath("//button[@aria-label='Sukupuoli']")).await;

    let links_f = candidate_urls_gender(driver, "female").await;
    let links_m = candidate_urls_gender(driver, "male").await;
    let links_o = candidate_urls_gender(driver, "other").await;
    let mut links_n = candidate_urls(driver).await;
    links_n.retain(|i| !links_f.contains(i) && !links_m.contains(i) && !links_o.contains(i));

    let mut municipality: Vec<String> = Vec::new();
    for (links, gender) in [
        (links_f, "female"),
        (links_m, "male"),
        (links_o, "other"),
        (links_n, ""),
    ] {
        for link in links {
            let candidate = loop {
                match candidate(driver, &link, gender, questions).await {
                    Ok(candidate) => break candidate,
                    Err(e) => eprintln!("{e}"),
                }
            };
            municipality.push(candidate);
        }
    }

    municipality.join("\n")
}

async fn process_url(url: &str, file: &str, questions: usize) -> Result<(), Box<dyn Error>> {
    let (mut child, driver, directory) = misc::driver().await?;

    let content = municipality(&driver, url, questions).await;
    misc::save(&content, file, true).await?;

    driver.quit().await?;
    child.kill().await?;
    std::fs::remove_dir_all(directory)?;

    Ok(())
}

pub async fn process_urls(urls: &Vec<String>, file: &str, questions: usize, threads: usize) {
    futures::stream::iter(urls)
        .map(|url| async move {
            loop {
                match process_url(url, file, questions).await {
                    Ok(_) => break,
                    Err(e) => eprintln!("{e}"),
                }
            }
        })
        .buffer_unordered(threads)
        .collect::<Vec<_>>()
        .await;
}
