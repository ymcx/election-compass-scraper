import concurrent.futures
import csv
import headers
import random
import sys
from threading import Lock
from typing import List, Tuple
from selenium.webdriver import Chrome, ChromeOptions
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions
from selenium.webdriver.support.ui import WebDriverWait


class Config:
    THREADS = 8
    TIMEOUT = 5
    ELECTION = headers.COUNTY_ELECTIONS_2025


def create_driver() -> Chrome:
    """Create and configure a headless Chrome driver."""
    dir = random.randint(0, sys.maxsize)
    options = ChromeOptions()
    options.add_argument("--headless")
    options.add_argument(f"--user-data-dir=__pycache__/{dir}")

    try:
        return Chrome(options)
    except Exception as e:
        print(e)
        sys.exit(1)


def click_element(driver: Chrome, locator: Tuple[str, str]) -> None:
    """Click an element after waiting for it to be clickable."""
    WebDriverWait(driver, Config.TIMEOUT).until(
        expected_conditions.element_to_be_clickable(locator)
    ).click()


def expand_candidate_list(driver: Chrome) -> None:
    """Keep clicking 'Show more' until all candidates are loaded."""
    while True:
        try:
            click_element(driver, (By.XPATH, "//button[@aria-label='Näytä lisää']"))
        except Exception:
            break


def get_candidate_urls_gender(driver: Chrome, gender: str) -> List[str]:
    """Collect candidate URLs for a specific gender."""
    click_element(driver, (By.XPATH, f"//input[@value='{gender}']"))
    candidates = get_candidate_urls(driver)
    click_element(driver, (By.XPATH, f"//input[@value='{gender}']"))

    return candidates


def get_candidate_urls(driver: Chrome) -> List[str]:
    """Collect candidate URLs."""
    candidates = []
    for link in driver.find_elements(By.TAG_NAME, "a"):
        href = link.get_attribute("href")
        if href and Config.ELECTION.URL in href:
            candidates.append(href)

    return candidates


def parse_candidate_info(driver: Chrome, gender: str) -> List[str]:
    keys = driver.find_elements(By.CLASS_NAME, "sc-fxLEUo.iIOaPI")
    values = driver.find_elements(By.CLASS_NAME, "sc-cDCfkV.yFgtA")
    extract = ["Kotikunta", "Koulutus", "Syntymävuosi", "Äidinkieli"]
    data = {
        key.text: values[i].text for i, key in enumerate(keys) if key.text in extract
    }

    name = driver.find_element(By.CLASS_NAME, "sc-xyPcs.eWLytF").text
    party = driver.find_element(
        By.CLASS_NAME, "sc-cdoHnr.ebTVhW.sc-fUubzJ.sc-isewAz.gtdlFp.cbPEsB"
    ).text
    municipality = data.get(extract[0], "")
    education = data.get(extract[1], "")
    year_of_birth = data.get(extract[2], "")
    language = data.get(extract[3], "")

    return [name, party, municipality, education, year_of_birth, language, gender]


def parse_candidate_answers(driver: Chrome) -> List[str]:
    """Parse candidate's answers to questions."""
    answers = []
    questions = driver.find_elements(By.CLASS_NAME, "sc-bRilDX.sc-lcBlzg.ddoxjp.ZCKmG")[
        : len(Config.ELECTION.QUESTIONS)
    ]
    for question in questions:
        options = question.find_elements(By.CLASS_NAME, "sc-kuCIbt")
        selected = next(
            (
                str(i)
                for i, option in enumerate(options)
                if option.get_attribute("imgurl")
            ),
            "",
        )
        answers.append(selected)

    return answers


def scrape_candidates(driver: Chrome, urls: List[str], gender: str):
    """Scrapes all candidates of the given gender."""
    return [scrape_candidate(driver, url, gender) for url in urls]


def scrape_candidate(driver: Chrome, url: str, gender: str) -> List[str]:
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

    candidate_f = get_candidate_urls_gender(driver, "female")
    candidate_m = get_candidate_urls_gender(driver, "male")
    candidate_o = get_candidate_urls_gender(driver, "other")
    candidate_n = get_candidate_urls(driver)
    candidate_n = list(
        set(candidate_n) - set(candidate_f) - set(candidate_m) - set(candidate_o)
    )

    candidates = (
        scrape_candidates(driver, candidate_f, "female")
        + scrape_candidates(driver, candidate_m, "male")
        + scrape_candidates(driver, candidate_o, "other")
        + scrape_candidates(driver, candidate_n, "")
    )

    driver.quit()
    save(candidates, "a", lock)


def save(contents: List[List[str]], mode: str, lock: Lock) -> None:
    """Saves the given contents to a file in CSV format."""
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
