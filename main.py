import concurrent.futures
import csv
import json
from typing import List, Tuple

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions
from selenium.webdriver.support.ui import WebDriverWait


class Config:
    BASE_URL = "https://vaalit.yle.fi/vaalikone/kuntavaalit2025/"
    FILE = "candidates.csv"
    MAX_QUESTIONS = 25
    MUNICIPALITIES = 313
    THREADS = 8
    TIMEOUT = 5


def create_driver() -> webdriver.Chrome:
    """Create and configure a headless Chrome driver."""
    options = webdriver.ChromeOptions()
    options.add_argument("--headless")
    return webdriver.Chrome(options)


def click_element(driver: webdriver.Chrome, locator: Tuple[str, str]) -> None:
    """Click an element after waiting for it to be clickable."""
    WebDriverWait(driver, Config.TIMEOUT).until(
        expected_conditions.element_to_be_clickable(locator)
    ).click()


def expand_candidate_list(driver: webdriver.Chrome) -> None:
    """Keep clicking 'Show more' until all candidates are loaded."""
    while True:
        try:
            click_element(driver, (By.XPATH, "//button[@aria-label='Näytä lisää']"))
        except Exception:
            break


def get_candidate_urls(driver: webdriver.Chrome, gender: str) -> List[Tuple[str, str]]:
    """Collect candidate URLs for a specific gender."""
    click_element(driver, (By.XPATH, f"//input[@value='{gender}']"))

    candidates = []
    for link in driver.find_elements(By.TAG_NAME, "a"):
        href = link.get_attribute("href")
        if href and Config.BASE_URL in href:
            candidates.append((href, gender))

    click_element(driver, (By.XPATH, f"//input[@value='{gender}']"))
    return candidates


def parse_candidate_info(driver: webdriver.Chrome, gender: str) -> List[str]:
    script = driver.find_element(By.XPATH, '//script[@type="application/ld+json"]')
    attribute = script.get_attribute("innerHTML")
    if not attribute:
        return []

    data = json.loads(attribute)
    person_data = next(
        (item for item in data["@graph"] if item.get("@type") == "Person"), {}
    )

    keys = driver.find_elements(By.CLASS_NAME, "sc-fxLEUo.iIOaPI")
    values = driver.find_elements(By.CLASS_NAME, "sc-cDCfkV.yFgtA")

    fields_to_extract = ["Kotikunta", "Koulutus", "Syntymävuosi", "Äidinkieli"]
    extracted_data = {
        key.text: values[index].text
        for index, key in enumerate(keys)
        if key.text in fields_to_extract
    }

    return [
        person_data.get("givenName", ""),
        person_data.get("familyName", ""),
        person_data.get("affiliation", ""),
        extracted_data.get("Kotikunta", ""),
        extracted_data.get("Koulutus", ""),
        extracted_data.get("Syntymävuosi", ""),
        extracted_data.get("Äidinkieli", ""),
        gender,
    ]


def parse_candidate_answers(driver: webdriver.Chrome) -> List[str]:
    """Parse candidate's answers to questions."""
    answers = []
    questions = driver.find_elements(By.CLASS_NAME, "sc-bRilDX.sc-lcBlzg.ddoxjp.ZCKmG")[
        : Config.MAX_QUESTIONS
    ]

    for question in questions:
        options = question.find_elements(By.CLASS_NAME, "sc-kuCIbt")
        selected = next(
            (str(i) for i, opt in enumerate(options) if opt.get_attribute("imgurl")),
            "-1",
        )
        answers.append(selected)

    return answers


def scrape_candidate(driver: webdriver.Chrome, url: str, gender: str) -> List[str]:
    """Scrape individual candidate details."""
    driver.get(url)
    click_element(driver, (By.XPATH, "//button[@aria-label='Näytä lisää']"))

    info = parse_candidate_info(driver, gender)
    answers = parse_candidate_answers(driver)
    return info + answers


def process_municipality(url: str) -> List[List[str]]:
    """Process a single municipality page and all its candidates."""
    print(f"Processing {url}")

    driver = create_driver()
    driver.get(url)

    click_element(driver, (By.XPATH, "//button[@aria-label='Vain välttämättömät']"))
    expand_candidate_list(driver)
    click_element(driver, (By.XPATH, "//button[@aria-label='Sukupuoli']"))

    candidate_urls = []
    for gender in ["female", "male", "other"]:
        candidate_urls += get_candidate_urls(driver, gender)

    candidates = [
        scrape_candidate(driver, url, gender) for url, gender in candidate_urls
    ]

    driver.quit()

    return candidates


def main() -> None:
    """Main scraping workflow."""
    municipality_urls = [
        f"{Config.BASE_URL}{i + 1}" for i in range(Config.MUNICIPALITIES)
    ]

    with concurrent.futures.ThreadPoolExecutor(max_workers=Config.THREADS) as executor:
        futures = [
            executor.submit(process_municipality, url) for url in municipality_urls
        ]

        with open(Config.FILE, "w", newline="", encoding="utf-8") as file:
            writer = csv.writer(file)

            for future in concurrent.futures.as_completed(futures):
                writer.writerows(future.result())


if __name__ == "__main__":
    main()
