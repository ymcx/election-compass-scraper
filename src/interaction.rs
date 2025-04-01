use thirtyfour::{By, WebDriver, WebElement, prelude::ElementQueryable};

async fn click(driver: &WebDriver, by: By, wait: bool) -> bool {
    let query = if wait {
        driver.query(by)
    } else {
        driver.query(by).nowait()
    };

    if let Ok(element) = query.first().await {
        return element.click().await.is_ok();
    }

    false
}

pub async fn click_gender_checkbox(driver: &WebDriver, gender: &str) -> bool {
    if gender.is_empty() {
        return true;
    }

    let element = format!("//input[@value='{gender}']");
    click(driver, By::XPath(element), true).await
}

pub async fn click_gender_button(driver: &WebDriver) -> bool {
    let element = "//button[@aria-label='Sukupuoli']";
    click(driver, By::XPath(element), true).await
}

pub async fn click_show_more(driver: &WebDriver, continuously: bool) -> bool {
    let element = "//button[@aria-label='Näytä lisää']";
    if !continuously {
        return click(driver, By::XPath(element), true).await;
    }

    while click(driver, By::XPath(element), false).await {}
    true
}

pub async fn click_accept_cookies(driver: &WebDriver) -> bool {
    let element = "//button[@aria-label='Vain välttämättömät']";
    click(driver, By::XPath(element), true).await
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
    while let Err(e) = driver.goto(url).await {
        eprintln!("{e}");
    }
}
