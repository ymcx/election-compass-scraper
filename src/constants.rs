use std::ops::Range;

const COMMON: &[&str] = &[
    "name",
    "party",
    "municipality",
    "education",
    "year of birth",
    "language",
    "language skills",
    "gender",
];

pub const TAG: &str = "        __          __  __
  ___  / /__  _____/ /_/_/___  ____     _________  ________  ____  ____  __________   ___________________  ____  ___  _____
 / _ \\/ / _ \\/ ___/ __/ / __ \\/ __ \\   / ___/ __ \\/ __  __ \\/ __ \\/ __ \\/ ___/ ___/  / ___/ ___/ ___/ __ \\/ __ \\/ _ \\/ ___/
/ ___/ / ___/ /__/ /_/ / /_/ / / / /  / /__/ /_/ / / / / / / /_/ / /_/ /\\__ \\\\__ \\   \\__ \\ /__/ /  / /_/ / /_/ / ___/ /
\\___/_/\\___/\\___/\\__/_/\\____/_/ /_/   \\___/\\____/_/ /_/ /_/ ____/\\__/_//____/____/  /____/___/_/   \\__/_/ ____/\\___/_/
                                                         \\_/                                           \\_/";

pub struct Election {
    pub file: String,
    pub urls: Vec<String>,
    pub headers: String,
    pub questions: usize,
}

impl Election {
    fn new(
        file: &str,
        url: &str,
        range: &[Range<usize>],
        common: &[&str],
        questions: &[&str],
    ) -> Self {
        Self {
            file: file.to_string(),
            urls: range
                .iter()
                .flat_map(|range| range.clone().map(|i| format!("{url}{i}")))
                .collect(),
            headers: [common, questions]
                .concat()
                .iter()
                .map(|&i| i.to_string())
                .collect::<Vec<_>>()
                .join(";"),
            questions: questions.len(),
        }
    }

    pub fn get(election: usize) -> Self {
        match election {
            1 => Self::county_2025(),
            _ => Self::municipal_2025(),
        }
    }

    fn municipal_2025() -> Self {
        const FILE: &str = "data/MUNICIPAL_ELECTIONS_2025.csv";
        const URL: &str = "https://vaalit.yle.fi/vaalikone/kuntavaalit2025/";
        const RANGE: &[Range<usize>] = &[1..263, 284..314];
        const QUESTIONS: &[&str] = &[
            "my municipality should set a maximum size for elementary school teaching groups.",
            "it is justifiable to close down small schools to cut costs.",
            "in my municipality, pupils should have the opportunity to study swedish in finnish-speaking schools before the sixth grade in elementary schools.",
            "my municipality should financially support the home care of children under the age of 3.",
            "my municipality should attract residents with a baby allowance.",
            "schools and kindergartens should serve less meat.",
            "schools should maintain christian traditions.",
            "if the municipality's income and expenses must be balanced, it should be done by cutting expenses rather than raising taxes.",
            "capital gains should not be subject to municipal taxation.",
            "the municipality should primarily buy services from companies that it owns.",
            "municipalities should help to finance regional flights.",
            "remote support in looking for jobs is sufficient for unemployed people in small towns.",
            "if necessary, the municipality should expropriate private land in order to obtain land for construction.",
            "my municipality should avoid clear-cutting in the forest areas it owns.",
            "the municipality should prioritise construction over preserving the local nature.",
            "my municipality should attract more industrial jobs, even if it harms the environment.",
            "my municipality must strive to achieve carbon neutrality before finland's national target year of 2035.",
            "even sparsely populated areas of my municipality should have access to public transport to reach services.",
            "immigrants strengthen the vitality of my municipality.",
            "my municipality must refuse to accept quota refugees.",
            "my municipality should finance events that encourage interaction between different cultures.",
            "my municipality should reduce spending on immigrant integration services.",
            "municipalities should compensate for the state cuts in theatre funding.",
            "libraries should be self-service so that my municipality can save on staff salary costs.",
            "my municipality should show support for gender and sexual minorities by flying a pride flag.",
        ];
        Self::new(FILE, URL, RANGE, COMMON, QUESTIONS)
    }

    fn county_2025() -> Self {
        const FILE: &str = "data/COUNTY_ELECTIONS_2025.csv";
        const URL: &str = "https://vaalit.yle.fi/vaalikone/aluevaalit2025/";
        const RANGE: &[Range<usize>] = &[263..284];
        const QUESTIONS: &[&str] = &[
            "in my wellbeing services county, i should be able to get non-urgent treatment in primary health care within two weeks.",
            "customer fees for public healthcare should be reduced.",
            "it is right that public money is spent to support people's use of private healthcare.",
            "one round-the-clock specialist hospital with emergency services will be enough in my wellbeing services county in the future.",
            "i am prepared to compromise on the social and health services of my home municipality if it is in the interest of the entire wellbeing services county.",
            "my wellbeing services county should attract doctors to health centers with financial incentives, even if that means cutting spending elsewhere.",
            "my wellbeing services county should attract care staff from abroad.",
            "my wellbeing services county must ensure that caregivers have a good command of finnish or swedish.",
            "my wellbeing services county should also offer undocumented people healthcare that is not required by law.",
            "this wellbeing services county should stop providing free contraception for young people.",
            "my wellbeing services county should train social and healthcare workers about the diversity of sexual orientation and gender.",
            "it is right that even small hospitals in swedish-speaking areas be allowed maintain 24-hour special medical care services due to linguistic rights.",
            "young people should be able to access mental health services without an appointment in my wellbeing services county.",
            "we should add more beds to 24-hour service housing for the elderly, even if it costs more.",
            "in eldercare, home visits can be replaced by remote services.",
            "adult children should take more responsibility for caring for their elderly parents than they do now.",
            "we must accept that one's place of residence affects the availability of basic social and healthcare services.",
            "this wellbeing services county should reduce its purchase of child protection services from large companies.",
            "drug users should have the opportunity to exchange dirty needles for clean ones in all the municipalities in my wellbeing services county.",
            "the current fire brigades must be maintained, even in remote areas.",
            "wellbeing services counties should be given the right to collect tax.",
            "the number of wellbeing services counties should be reduced from the current 21.",
            "well-being areas must make budget cuts within the required timeframe, even if that means that services must be quickly reduced.",
            "no-one should be allowed to hold triple roles as mp, municipal councillor and regional councillor at the same time.",
            "my wellbeing services county must take into account the reduction of climate emissions in all its decisions.",
        ];
        Self::new(FILE, URL, RANGE, COMMON, QUESTIONS)
    }
}
