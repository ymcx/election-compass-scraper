use tokio::process::{Child, Command};

pub struct DriverProcess {
    process: Option<Child>,
    port: usize,
}

impl DriverProcess {
    pub fn new(port: usize) -> Self {
        let process = Command::new("chromedriver")
            .arg(format!("--port={port}"))
            .arg("--silent")
            .spawn()
            .ok();

        Self { process, port }
    }
}

impl Drop for DriverProcess {
    fn drop(&mut self) {
        let directory = format!("/tmp/scraper-{}", self.port);
        if let Some(mut process) = self.process.take() {
            tokio::spawn(async move {
                process.kill().await.ok();
                std::fs::remove_dir_all(directory).ok();
            });
        }
    }
}
