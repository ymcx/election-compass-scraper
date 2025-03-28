use crate::interaction;
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

async fn candidate_info(driver: &WebDriver, fields: usize) -> String {
    let name = interaction::element(driver, By::ClassName("sc-xyPcs")).await;
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

    let mut info = vec![name];
    for (i, key) in keys.iter().enumerate().take(fields) {
        let text = key.text().await.unwrap_or_default();
        if extract.contains(&text.as_str()) {
            info.push(vals[i].text().await.unwrap_or_default());
        } else {
            info.push(String::default());
        }
    }

    info.join(";")
}

async fn candidate_answers(driver: &WebDriver, fields: usize) -> String {
    let mut answers: Vec<String> = Vec::new();
    let elements = interaction::elements(driver, By::ClassName("sc-bRilDX")).await;
    for element in elements.iter().take(fields) {
        let options = element
            .query(By::ClassName("sc-kuCIbt"))
            .all_from_selector()
            .await
            .unwrap_or_default();

        let mut selected = String::default();
        for (i, option) in options.iter().enumerate() {
            if option.attr("imgurl").await.unwrap_or_default().is_some() {
                selected = i.to_string();
                break;
            }
        }

        answers.push(selected);
    }

    answers.join(";")
}

async fn candidate(driver: &WebDriver, url: &str, gender: &str, fields: (usize, usize)) -> String {
    let url = format!("https://vaalit.yle.fi{url}");
    driver.goto(url).await.unwrap();
    interaction::click(driver, By::XPath("//button[@aria-label='Näytä lisää']")).await;

    let info = candidate_info(driver, fields.0).await;
    let answers = candidate_answers(driver, fields.1).await;

    format!("{info};{gender};{answers}")
}

pub async fn municipality(driver: &WebDriver, url: &str, fields: (usize, usize)) -> Vec<String> {
    driver.goto(url).await.unwrap();
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
            let candidate = candidate(driver, &link, gender, fields).await;
            municipality.push(candidate);
        }
    }

    municipality
}
