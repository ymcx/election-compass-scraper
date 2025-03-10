import csv
import headers
from typing import List
from selenium.webdriver import Chrome, ChromeOptions
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions
from selenium.webdriver.support.ui import WebDriverWait

ELECTIONS = headers.COUNTY_ELECTIONS_2025


def create_driver() -> Chrome:
    """Create and configure a headless Chrome driver."""
    options = ChromeOptions()
    options.add_argument("--headless")

    return Chrome(options)


def click_element(driver: Chrome, by: str, value: str) -> None:
    """Click an element after waiting for it to be clickable."""
    WebDriverWait(driver, 5).until(
        expected_conditions.element_to_be_clickable((by, value))
    ).click()


def find_element(driver: Chrome, by: str, value: str) -> str:
    """Returns the found element or an empty string."""
    try:
        return driver.find_element(by, value).text
    except Exception:
        return ""


def expand_list(driver: Chrome) -> None:
    """Keep clicking 'Show more' until the whole list is visible."""
    while True:
        try:
            click_element(driver, By.XPATH, "//button[@aria-label='Näytä lisää']")
        except Exception:
            break


def get_candidate_urls_gender(driver: Chrome, gender: str) -> List[str]:
    """Collect all candidate URLs in the current page, filtered by gender."""
    click_element(driver, By.XPATH, f"//input[@value='{gender}']")
    candidates = get_candidate_urls(driver)
    click_element(driver, By.XPATH, f"//input[@value='{gender}']")

    return candidates


def get_candidate_urls(driver: Chrome) -> List[str]:
    """Collect all candidate URLs in the current page."""
    candidates = []
    links = driver.find_elements(By.TAG_NAME, "a")
    for link in links:
        href = link.get_attribute("href")
        if href and ELECTIONS.URL in href:
            candidates.append(href)

    return candidates


def get_candidate_info(driver: Chrome) -> List[str]:
    """Parse candidate's information."""
    keys = driver.find_elements(By.CLASS_NAME, "sc-fxLEUo.iIOaPI")
    values = driver.find_elements(By.CLASS_NAME, "sc-cDCfkV.yFgtA")
    extract = ["Kotikunta", "Koulutus", "Syntymävuosi", "Äidinkieli"]
    data = {
        key.text: values[i].text for i, key in enumerate(keys) if key.text in extract
    }

    name = find_element(driver, By.CLASS_NAME, "sc-xyPcs.eWLytF")
    party = find_element(driver, By.CLASS_NAME, "sc-cdoHnr.ebTVhW.sc-fUubzJ.sc-isewAz.gtdlFp.cbPEsB")
    municipality = data.get(extract[0], "")
    education = data.get(extract[1], "")
    year_of_birth = data.get(extract[2], "")
    language = data.get(extract[3], "")

    return [name, party, municipality, education, year_of_birth, language]


def get_candidate_answers(driver: Chrome) -> List[str]:
    """Parse candidate's answers to questions."""
    answers = []
    questions = driver.find_elements(By.CLASS_NAME, "sc-bRilDX.sc-lcBlzg.ddoxjp.ZCKmG")[
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

    return answers


def get_candidate(driver: Chrome, url: str, gender: str) -> List[str]:
    """Scrape individual candidate details."""
    print(url)

    driver.get(url)
    click_element(driver, By.XPATH, "//button[@aria-label='Näytä lisää']")

    info = get_candidate_info(driver)
    answers = get_candidate_answers(driver)

    return info + [gender] + answers


def get_municipality(driver: Chrome, url: str) -> List[List[str]]:
    """Process a single municipality page and all its candidates."""
    driver.get(url)
    expand_list(driver)
    click_element(driver, By.XPATH, "//button[@aria-label='Sukupuoli']")

    candidate_urls_f = get_candidate_urls_gender(driver, "female")
    candidate_urls_m = get_candidate_urls_gender(driver, "male")
    candidate_urls_o = get_candidate_urls_gender(driver, "other")
    candidate_urls_n = list(
        set(get_candidate_urls(driver))
        - set(candidate_urls_f)
        - set(candidate_urls_m)
        - set(candidate_urls_o)
    )

    candidates_f = [get_candidate(driver, url, "female") for url in candidate_urls_f]
    candidates_m = [get_candidate(driver, url, "male") for url in candidate_urls_m]
    candidates_o = [get_candidate(driver, url, "other") for url in candidate_urls_o]
    candidates_n = [get_candidate(driver, url, "") for url in candidate_urls_n]

    return candidates_f + candidates_m + candidates_o + candidates_n


def save(contents: List[List[str]], mode: str) -> None:
    """Saves the given contents to a file in CSV format."""
    file = open(ELECTIONS.FILE, mode)
    csv.writer(file).writerows(contents)
    file.close()


def main() -> None:
    """Main scraping workflow."""
    driver = create_driver()
    driver.get(ELECTIONS.URL)
    click_element(driver, By.XPATH, "//button[@aria-label='Vain välttämättömät']")

    save([ELECTIONS.FIELDS], "w")

    urls = [f"{ELECTIONS.URL}{i}" for i in ELECTIONS.RANGE]
    for i, url in enumerate(urls):
        progress = f"{round(i / len(urls) * 100, 2)}%"
        print(progress)

        fields = get_municipality(driver, url)
        save(fields, "a")


main()
