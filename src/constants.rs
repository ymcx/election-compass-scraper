use std::ops::Range;

const COMMON: [&str; 7] = [
    "Name",
    "Party",
    "Municipality",
    "Education",
    "Year of birth",
    "Language",
    "Gender",
];

#[derive(Clone)]
pub struct Elections {
    pub file: String,
    pub url: String,
    pub range: Vec<Range<u16>>,
    pub headers: Vec<String>,
}

#[allow(dead_code)]
pub fn municipal_elections_2025() -> Elections {
    const FILE: &str = "MUNICIPAL_ELECTIONS_2025.csv";
    const URL: &str = "https://vaalit.yle.fi/vaalikone/kuntavaalit2025/";
    const RANGE: [Range<u16>; 2] = [1..263, 284..314];
    const QUESTIONS: [&str; 25] = [
        "My municipality should set a maximum size for elementary school teaching groups",
        "It is justifiable to close down small schools to cut costs",
        "In my municipality pupils should have the opportunity to study Swedish in Finnish-speaking schools before the sixth grade in elementary schools",
        "My municipality should financially support the home care of children under the age of 3",
        "My municipality should attract residents with a baby allowance",
        "Schools and kindergartens should serve less meat",
        "Schools should maintain Christian traditions",
        "If the municipality's income and expenses must be balanced it should be done by cutting expenses rather than raising taxes",
        "Capital gains should not be subject to municipal taxation",
        "The municipality should primarily buy services from companies that it owns",
        "Municipalities should help to finance regional flights",
        "Remote support in looking for jobs is sufficient for unemployed people in small towns",
        "If necessary the municipality should expropriate private land in order to obtain land for construction",
        "My municipality should avoid clear-cutting in the forest areas it owns",
        "The municipality should prioritise construction over preserving the local nature",
        "My municipality should attract more industrial jobs even if it harms the environment",
        "My municipality must strive to achieve carbon neutrality before Finland's national target year of 2035",
        "Even sparsely populated areas of my municipality should have access to public transport to reach services",
        "Immigrants strengthen the vitality of my municipality",
        "My municipality must refuse to accept quota refugees",
        "My municipality should finance events that encourage interaction between different cultures",
        "My municipality should reduce spending on immigrant integration services",
        "Municipalities should compensate for the state cuts in theatre funding",
        "Libraries should be self-service so that my municipality can save on staff salary costs",
        "My municipality should show support for gender and sexual minorities by flying a Pride flag",
    ];

    Elections {
        file: FILE.to_string(),
        url: URL.to_string(),
        range: RANGE.to_vec(),
        headers: COMMON
            .iter()
            .chain(QUESTIONS.iter())
            .map(|&i| i.to_string())
            .collect(),
    }
}

#[allow(dead_code)]
pub fn county_elections_2025() -> Elections {
    const FILE: &str = "COUNTY_ELECTIONS_2025.csv";
    const URL: &str = "https://vaalit.yle.fi/vaalikone/aluevaalit2025/";
    const RANGE: [Range<u16>; 1] = [263..284];
    const QUESTIONS: [&str; 25] = [
        "In my wellbeing services county I should be able to get non-urgent treatment in primary health care within two weeks",
        "Customer fees for public healthcare should be reduced",
        "It is right that public money is spent to support people's use of private healthcare",
        "One round-the-clock specialist hospital with emergency services will be enough in my wellbeing services county in the future",
        "I am prepared to compromise on the social and health services of my home municipality if it is in the interest of the entire wellbeing services county",
        "My wellbeing services county should attract doctors to health centers with financial incentives even if that means cutting spending elsewhere",
        "My wellbeing services county should attract care staff from abroad",
        "My wellbeing services county must ensure that caregivers have a good command of Finnish or Swedish",
        "My wellbeing services county should also offer undocumented people healthcare that is not required by law",
        "This wellbeing services county should stop providing free contraception for young people",
        "My wellbeing services county should train social and healthcare workers about the diversity of sexual orientation and gender",
        "It is right that even small hospitals in Swedish-speaking areas be allowed maintain 24-hour special medical care services due to linguistic rights",
        "Young people should be able to access mental health services without an appointment in my wellbeing services county",
        "We should add more beds to 24-hour service housing for the elderly even if it costs more",
        "In eldercare home visits can be replaced by remote services",
        "Adult children should take more responsibility for caring for their elderly parents than they do now",
        "We must accept that one's place of residence affects the availability of basic social and healthcare services",
        "This wellbeing services county should reduce its purchase of child protection services from large companies",
        "Drug users should have the opportunity to exchange dirty needles for clean ones in all the municipalities in my wellbeing services county",
        "The current fire brigades must be maintained even in remote areas",
        "Wellbeing services counties should be given the right to collect tax",
        "The number of wellbeing services counties should be reduced from the current 21",
        "Well-being areas must make budget cuts within the required timeframe even if that means that services must be quickly reduced",
        "No-one should be allowed to hold triple roles as MP municipal councillor and regional councillor at the same time",
        "My wellbeing services county must take into account the reduction of climate emissions in all its decisions",
    ];

    Elections {
        file: FILE.to_string(),
        url: URL.to_string(),
        range: RANGE.to_vec(),
        headers: COMMON
            .iter()
            .chain(QUESTIONS.iter())
            .map(|&i| i.to_string())
            .collect(),
    }
}
