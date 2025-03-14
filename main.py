import csv
import headers
from typing import List
from selenium.webdriver import Chrome, ChromeOptions
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions
from selenium.webdriver.support.ui import WebDriverWait
from selenium.common.exceptions import (
    StaleElementReferenceException,
    NoSuchElementException,
    TimeoutException,
)

ELECTIONS = headers.COUNTY_ELECTIONS_2025
DRIVER: Chrome


def create_driver() -> Chrome:
    """Create and configure a headless Chrome driver."""
    options = ChromeOptions()
    options.add_argument("--headless")

    return Chrome(options)


def click_element(by: str, value: str) -> bool:
    """Click an element after waiting for it to be clickable."""
    try:
        WebDriverWait(DRIVER, 5).until(
            expected_conditions.element_to_be_clickable((by, value))
        ).click()

        return True
    except TimeoutException:
        return False


def click_element_once(by: str, value: str) -> None:
    """Try to click an element until it's clicked."""
    for _ in range(3):
        success = click_element(by, value)
        if success:
            break


def click_element_forever(by: str, value: str) -> None:
    """Click an element until it's not clickable anymore."""
    for _ in range(50):
        success = click_element(by, value)
        if not success:
            break


def find_element(by: str, value: str) -> str:
    """Returns the found element or an empty string."""
    try:
        return DRIVER.find_element(by, value).text
    except NoSuchElementException:
        return ""


def get_candidate_urls_gender(gender: str) -> List[str]:
    """Collect all candidate URLs in the current page, filtered by gender."""
    click_element_once(By.XPATH, f"//input[@value='{gender}']")
    candidates = get_candidate_urls()
    click_element_once(By.XPATH, f"//input[@value='{gender}']")

    return candidates


def get_candidate_urls() -> List[str]:
    """Collect all candidate URLs in the current page."""
    while True:
        try:
            candidates = []
            links = DRIVER.find_elements(By.TAG_NAME, "a")
            for link in links:
                href = link.get_attribute("href")
                if href and ELECTIONS.URL in href:
                    candidates.append(href)

            return candidates
        except StaleElementReferenceException as e:
            print(e)


def get_candidate_info() -> List[str]:
    """Parse candidate's information."""
    keys = DRIVER.find_elements(By.CLASS_NAME, "sc-fxLEUo")
    values = DRIVER.find_elements(By.CLASS_NAME, "sc-cDCfkV")
    extract = ["Kotikunta", "Koulutus", "Syntymävuosi", "Äidinkieli"]
    data = {
        key.text: values[i].text for i, key in enumerate(keys) if key.text in extract
    }

    name = find_element(By.CLASS_NAME, "sc-xyPcs")
    party = find_element(By.CLASS_NAME, "sc-cdoHnr")
    municipality = data.get(extract[0], "")
    education = data.get(extract[1], "")
    year_of_birth = data.get(extract[2], "")
    language = data.get(extract[3], "")

    return [name, party, municipality, education, year_of_birth, language]


def get_candidate_answers() -> List[str]:
    """Parse candidate's answers to questions."""
    answers = []

    for _ in range(3):
        answers = []
        questions = DRIVER.find_elements(By.CLASS_NAME, "sc-bRilDX")[
            : len(ELECTIONS.QUESTIONS)
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

        if len(answers) == 25:
            break

    return answers


def get_candidate(url: str, gender: str) -> List[str]:
    """Scrape individual candidate details."""
    print(url)

    DRIVER.get(url)
    click_element_once(By.XPATH, "//button[@aria-label='Näytä lisää']")

    info = get_candidate_info()
    answers = get_candidate_answers()

    return info + [gender] + answers


def get_municipality(url: str) -> List[List[str]]:
    """Process a single municipality page and all its candidates."""
    DRIVER.get(url)
    click_element_forever(By.XPATH, "//button[@aria-label='Näytä lisää']")
    click_element_once(By.XPATH, "//button[@aria-label='Sukupuoli']")

    candidate_urls_f = get_candidate_urls_gender("female")
    candidate_urls_m = get_candidate_urls_gender("male")
    candidate_urls_o = get_candidate_urls_gender("other")
    candidate_urls_n = list(
        set(get_candidate_urls())
        - set(candidate_urls_f)
        - set(candidate_urls_m)
        - set(candidate_urls_o)
    )

    candidates_f = [get_candidate(url, "female") for url in candidate_urls_f]
    candidates_m = [get_candidate(url, "male") for url in candidate_urls_m]
    candidates_o = [get_candidate(url, "other") for url in candidate_urls_o]
    candidates_n = [get_candidate(url, "") for url in candidate_urls_n]

    return candidates_f + candidates_m + candidates_o + candidates_n


def save(contents: List[List[str]], mode: str) -> None:
    """Saves the given contents to a file in CSV format."""
    file = open(ELECTIONS.FILE, mode)
    csv.writer(file).writerows(contents)
    file.close()


def main() -> None:
    """Main scraping workflow."""
    global DRIVER
    DRIVER = create_driver()
    DRIVER.get(ELECTIONS.URL)
    click_element_once(By.XPATH, "//button[@aria-label='Vain välttämättömät']")

    save([ELECTIONS.FIELDS], "w")

    urls = [f"{ELECTIONS.URL}{i}" for i in ELECTIONS.RANGE]
    for i, url in enumerate(urls):
        progress = f"{round(i / len(urls) * 100, 2)}%"
        print(progress)

        fields = get_municipality(url)
        save(fields, "a")


main()
