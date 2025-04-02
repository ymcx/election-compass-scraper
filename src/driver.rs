use rand::Rng;
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio::{
    process::{Child, Command},
    time::Duration,
};

pub struct Driver {
    process: Option<Child>,
    driver: Option<WebDriver>,
    port: usize,
}

impl Driver {
    pub async fn new() -> Self {
        let port = rand::rng().random_range(1024..65536);
        let process = Command::new("chromedriver")
            .arg(format!("--port={port}"))
            .arg("--silent")
            .spawn()
            .ok();

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut capabilities = DesiredCapabilities::chrome();
        let _ = capabilities.add_arg("--headless");
        let _ = capabilities.add_arg(&format!("--user-data-dir=/tmp/scraper-{port}"));
        let driver = WebDriver::new(format!("http://localhost:{port}"), capabilities)
            .await
            .ok();

        Self {
            process,
            driver,
            port,
        }
    }

    pub fn driver(&self) -> Option<&WebDriver> {
        self.driver.as_ref()
    }

    pub async fn drop(&mut self) {
        let process = self.process.take();
        let driver = self.driver.take();
        let directory = format!("/tmp/scraper-{}", self.port);
        let _ = std::fs::remove_dir_all(directory);

        if let Some(driver) = driver {
            let _ = driver.quit().await;
        }
        if let Some(mut process) = process {
            let _ = process.kill().await;
        }
    }
}
