import concurrent.futures
import csv
import headers
from threading import Lock
from typing import List, Tuple
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions
from selenium.webdriver.support.ui import WebDriverWait


class Config:
    THREADS = 8
    TIMEOUT = 5
    ELECTION = headers.COUNTY_ELECTIONS_2025


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
        if href and Config.ELECTION.URL in href:
            candidates.append((href, gender))

    click_element(driver, (By.XPATH, f"//input[@value='{gender}']"))
    return candidates


def parse_candidate_info(driver: webdriver.Chrome, gender: str) -> List[str]:
    keys = driver.find_elements(By.CLASS_NAME, "sc-fxLEUo.iIOaPI")
    values = driver.find_elements(By.CLASS_NAME, "sc-cDCfkV.yFgtA")

    extract = ["Kotikunta", "Koulutus", "Syntymävuosi", "Äidinkieli"]
    data = {
        key.text: values[index].text
        for index, key in enumerate(keys)
        if key.text in extract
    }

    name = driver.find_element(By.CLASS_NAME, "sc-xyPcs.eWLytF").text
    party = driver.find_element(By.CLASS_NAME, "sc-cdoHnr.ebTVhW.sc-fUubzJ.sc-isewAz.gtdlFp.cbPEsB").text

    return [
        name,
        party,
        data.get(extract[0], ""),
        data.get(extract[1], ""),
        data.get(extract[2], ""),
        data.get(extract[3], ""),
        gender,
    ]


def parse_candidate_answers(driver: webdriver.Chrome) -> List[str]:
    """Parse candidate's answers to questions."""
    answers = []
    questions = driver.find_elements(By.CLASS_NAME, "sc-bRilDX.sc-lcBlzg.ddoxjp.ZCKmG")[:len(Config.ELECTION.QUESTIONS)]

    for question in questions:
        options = question.find_elements(By.CLASS_NAME, "sc-kuCIbt")
        selected = next((str(i) for i, opt in enumerate(options) if opt.get_attribute("imgurl")), "-1")
        answers.append(selected)

    return answers


def scrape_candidate(driver: webdriver.Chrome, url: str, gender: str) -> List[str]:
    """Scrape individual candidate details."""
    driver.get(url)
    click_element(driver, (By.XPATH, "//button[@aria-label='Näytä lisää']"))

    info = parse_candidate_info(driver, gender)
    answers = parse_candidate_answers(driver)
    return info + answers


def process_municipality(url: str, lock: Lock) -> None:
    """Process a single municipality page and all its candidates."""
    print(url)

    driver = create_driver()
    driver.get(url)

    click_element(driver, (By.XPATH, "//button[@aria-label='Vain välttämättömät']"))
    expand_candidate_list(driver)
    click_element(driver, (By.XPATH, "//button[@aria-label='Sukupuoli']"))

    candidate_urls = []
    for gender in ["female", "male", "other"]:
        candidate_urls += get_candidate_urls(driver, gender)

    candidates = [scrape_candidate(driver, url, gender) for url, gender in candidate_urls]

    driver.quit()

    save(candidates, "a", lock)


def save(contents: List[List[str]], mode: str, lock: Lock) -> None:
    lock.acquire()

    file = open(Config.ELECTION.FILE, mode)
    csv.writer(file).writerows(contents)
    file.close()

    lock.release()


def main() -> None:
    """Main scraping workflow."""
    lock = Lock()
    save([Config.ELECTION.FIELDS], "w", lock)
    urls = [f"{Config.ELECTION.URL}{i}" for i in Config.ELECTION.RANGE]

    executor = concurrent.futures.ThreadPoolExecutor(Config.THREADS)
    for url in urls:
        executor.submit(process_municipality, url, lock)

    executor.shutdown()


main()
