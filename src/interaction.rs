use crate::io;
use thirtyfour::{
    By, WebDriver, WebElement, extensions::query::ElementQuery, prelude::ElementQueryable,
};

async fn click(query: &ElementQuery) -> bool {
    match query.first().await {
        Ok(element) => element.click().await.is_ok(),
        Err(_) => false,
    }
}

pub async fn click_gender_checkbox(driver: &WebDriver, gender: &str) -> bool {
    if gender.is_empty() {
        return true;
    }

    let element = format!("//input[@value='{gender}']");
    let query = driver.query(By::XPath(element));
    click(&query).await
}

pub async fn click_gender_button(driver: &WebDriver) -> bool {
    let element = "//button[@aria-label='Sukupuoli']";
    let query = driver.query(By::XPath(element));
    click(&query).await
}

pub async fn click_show_more(driver: &WebDriver, continuous: bool) -> bool {
    let element = "//button[@aria-label='Näytä lisää']";
    if !continuous {
        let query = driver.query(By::XPath(element));
        return click(&query).await;
    }

    let query = driver.query(By::XPath(element)).nowait();
    while click(&query).await {}
    true
}

pub async fn click_accept_cookies(driver: &WebDriver) -> bool {
    let element = "//button[@aria-label='Vain välttämättömät']";
    let query = driver.query(By::XPath(element));
    click(&query).await
}

async fn text(query: &ElementQuery) -> String {
    match query.first().await {
        Ok(element) => element.text().await.unwrap_or_default(),
        Err(_) => String::default(),
    }
}

pub async fn text_name(driver: &WebDriver) -> String {
    let element = "sc-xyPcs";
    let query = driver.query(By::ClassName(element));
    text(&query).await
}

async fn elements(query: &ElementQuery) -> Vec<WebElement> {
    query.all_from_selector().await.unwrap_or_default()
}

pub async fn elements_a(driver: &WebDriver) -> Vec<WebElement> {
    let element = "a";
    let query = driver.query(By::Tag(element));
    elements(&query).await
}

pub async fn elements_buttons(driver: &WebDriver) -> Vec<WebElement> {
    let element = "sc-heIBZE";
    let query = driver.query(By::ClassName(element));
    elements(&query).await
}

pub async fn elements_questions(driver: &WebDriver) -> Vec<WebElement> {
    let element = "sc-bRilDX";
    let query = driver.query(By::ClassName(element));
    elements(&query).await
}

pub async fn elements_info(driver: &WebDriver) -> (Vec<WebElement>, Vec<WebElement>) {
    let element = ("sc-fxLEUo", "sc-cDCfkV");
    let query = (
        driver.query(By::ClassName(element.0)),
        driver.query(By::ClassName(element.1)),
    );
    (elements(&query.0).await, elements(&query.1).await)
}

pub async fn elements_options(question: &WebElement) -> Vec<WebElement> {
    let element = "sc-kuCIbt";
    let query = question.query(By::ClassName(element));
    elements(&query).await
}

pub async fn goto(driver: &WebDriver, url: &str) {
    while let Err(e) = driver.goto(url).await {
        io::print_error(&Box::new(e).into());
    }
}
