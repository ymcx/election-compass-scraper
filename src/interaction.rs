use thirtyfour::{By, WebDriver, WebElement, prelude::ElementQueryable};

pub async fn click(driver: &WebDriver, by: By) -> bool {
    if let Ok(element) = driver.query(by).first().await {
        return element.click().await.is_ok();
    }

    false
}

pub async fn element(driver: &WebDriver, by: By) -> String {
    if let Ok(element) = driver.query(by).first().await {
        return element.text().await.unwrap_or_default();
    }

    String::default()
}

pub async fn elements(driver: &WebDriver, by: By) -> Vec<WebElement> {
    driver
        .query(by)
        .all_from_selector()
        .await
        .unwrap_or_default()
}

pub async fn goto(driver: &WebDriver, url: &str) {
    while driver.goto(url).await.is_err() {
        eprintln!("Couldn't open {url}");
    }
}
