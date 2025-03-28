use std::collections::HashMap;
use thirtyfour::{By, WebDriver, prelude::ElementQueryable};

use crate::interaction::{click, element, elements};

async fn get_candidate_urls_gender(driver: &WebDriver, gender: &str) -> Vec<String> {
    click(driver, By::XPath(format!("//input[@value='{}']", gender))).await;
    let candidates = get_candidate_urls(driver).await;
    click(driver, By::XPath(format!("//input[@value='{}']", gender))).await;

    candidates
}

async fn get_candidate_urls(driver: &WebDriver) -> Vec<String> {
    loop {
        let mut candidates: Vec<String> = Vec::new();
        let links = elements(driver, By::Tag("a")).await;
        for link in links {
            let href = link.attr("href").await;
            if let Ok(h) = href {
                let m = h.unwrap();
                let jee = m;
                if jee.contains("ehdokkaat") {
                    candidates.push(format!("https://vaalit.yle.fi{}", jee));
                }
            }
        }
        return candidates;
    }
}

async fn get_candidate_info(driver: &WebDriver) -> String {
    let keys = elements(driver, By::ClassName("sc-fxLEUo")).await;
    let values = elements(driver, By::ClassName("sc-cDCfkV")).await;
    let extract = ["Kotikunta", "Koulutus", "Syntymävuosi", "Äidinkieli"];
    let mut data: HashMap<String, String> = HashMap::new();
    for (i, key) in keys.iter().enumerate() {
        let text: String = key.text().await.unwrap_or_default();
        let text: &str = text.as_str();
        for e in extract {
            if text.contains(e) {
                let val = values[i].text().await.unwrap_or_default();
                data.insert(text.to_string(), val);
            }
        }
    }

    let name = element(driver, By::ClassName("sc-xyPcs")).await;
    let party = element(driver, By::ClassName("sc-cdoHnr")).await;
    let munic = data.get(extract[0]).unwrap_or(&"".to_string()).to_string();
    let educa = data.get(extract[1]).unwrap_or(&"".to_string()).to_string();
    let year_ = data.get(extract[2]).unwrap_or(&"".to_string()).to_string();
    let langu = data.get(extract[3]).unwrap_or(&"".to_string()).to_string();

    let mut vec: Vec<String> = Vec::new();
    vec.push(name);
    vec.push(party);
    vec.push(munic);
    vec.push(educa);
    vec.push(year_);
    vec.push(langu);
    vec.join(",")
}

async fn get_candidate_answers(driver: &WebDriver) -> String {
    let mut answers: Vec<String> = Vec::new();

    for _ in 0..3 {
        answers = Vec::new();
        let questions = driver
            .query(By::ClassName("sc-bRilDX"))
            .all_from_selector()
            .await
            .unwrap_or_default();
        for (i, question) in questions.iter().enumerate() {
            if i == 25 {
                break;
            }

            let options = question
                .query(By::ClassName("sc-kuCIbt"))
                .all_from_selector()
                .await
                .unwrap_or_default();
            let mut selected = String::from("");
            for (k, option) in options.iter().enumerate() {
                if let Some(_) = option.attr("imgurl").await.unwrap_or_default() {
                    selected = k.to_string();
                }
            }
            answers.push(selected);
        }

        if answers.len() == 25 {
            break;
        }
    }

    answers.join(",")
}

async fn get_candidate(driver: &WebDriver, url: &str, gender: &str) -> String {
    let _ = driver.goto(url).await;
    click(driver, By::XPath("//button[@aria-label='Näytä lisää']")).await;
    let info = get_candidate_info(driver).await;
    let answers = get_candidate_answers(driver).await;

    let mut vec: Vec<String> = Vec::new();
    vec.push(info);
    vec.push(gender.to_string());
    vec.push(answers);
    vec.join(",")
}

pub async fn get_municipality(driver: &WebDriver, url: &str) -> Vec<String> {
    let _ = driver.goto(url).await;
    while click(driver, By::XPath("//button[@aria-label='Näytä lisää']")).await {}
    click(driver, By::XPath("//button[@aria-label='Sukupuoli']")).await;

    let candidate_urls_f = get_candidate_urls_gender(driver, "female").await;
    let candidate_urls_m = get_candidate_urls_gender(driver, "male").await;
    let candidate_urls_o = get_candidate_urls_gender(driver, "other").await;
    // neither

    let mut cf: Vec<String> = Vec::new();
    for url in candidate_urls_f {
        let c = get_candidate(driver, url.as_str(), "female").await;
        cf.push(c);
    }
    let mut cm: Vec<String> = Vec::new();
    for url in candidate_urls_m {
        let c = get_candidate(driver, url.as_str(), "male").await;
        cm.push(c);
    }
    let mut co: Vec<String> = Vec::new();
    for url in candidate_urls_o {
        let c = get_candidate(driver, url.as_str(), "other").await;
        co.push(c);
    }

    cf.append(&mut cm);
    cf.append(&mut co);
    cf
}

pub async fn accept_cookies(driver: &WebDriver) {
    let _ = driver.goto("https://yle.fi/").await;
    click(
        &driver,
        By::XPath("//button[@aria-label='Vain välttämättömät']"),
    )
    .await;
}
