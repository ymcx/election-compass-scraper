# Election compass scraper

Scrapes candidates' responses from Yle's election compass (vaalikone). The results for each election are saved to a file in the data directory.

<br>

```shell
$ election-compass-scraper --help
Usage: election-compass-scraper [OPTIONS]

Options:
  -e, --election <ELECTION>
  -y, --year <YEAR>
  -j, --jobs <JOBS>
  -h, --help                 Print help
$ election-compass-scraper --election=municipal --year=2025 -j4
         __          __  __
  ___  / /__  _____/ /_/_/___  ____     _________  ________  ____  ____  __________   ___________________  ____  ___  _____
 / _ \/ / _ \/ ___/ __/ / __ \/ __ \   / ___/ __ \/ __  __ \/ __ \/ __ \/ ___/ ___/  / ___/ ___/ ___/ __ \/ __ \/ _ \/ ___/
/ ___/ / ___/ /__/ /_/ / /_/ / / / /  / /__/ /_/ / / / / / / /_/ / /_/ /\__ \\__ \   \__ \ /__/ /  / /_/ / /_/ / ___/ /
\___/_/\___/\___/\__/_/\____/_/ /_/   \___/\____/_/ /_/ /_/ ____/\__/_//____/____/  /____/___/_/   \__/_/ ____/\___/_/
                                                         \_/                                           \_/
URL     [https://vaalit.yle.fi/vaalikone/kuntavaalit2025/]
THREADS [4]

[########-----------]
```
