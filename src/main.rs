use anyhow::{anyhow, Result};
use log::LevelFilter;
use simplelog::{ConfigBuilder, WriteLogger};
use std::{env, fs::OpenOptions, time::Instant};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;

    let start = Instant::now();

    for url in env::args().skip(1) {
        log::info!(">>> HTTP GET {url}");
        match reqwest::get(&url).await {
            Ok(response) => {
                let status = response.status();
                log::info!("Status: {status}");

                match response.text().await {
                    Ok(body) => {
                        log::info!("Body is {} bytes", body.len());
                    }
                    Err(err) => {
                        log::error!("Error getting response body: {err:?}");
                    }
                }
            }
            Err(err) => {
                log::error!("HTTP GET error: {err:?}");
            }
        }
    }

    let duration = start.elapsed();
    log::info!("Elapsed: {}mS", duration.as_millis());

    Ok(())
}

fn init_logger() -> Result<()> {
    // This assumes we are running from the project target/{release,debug} directory...
    let log_file_path = env::current_exe()?
        .parent()
        .ok_or(anyhow!("exe has no parent"))?
        .parent()
        .ok_or(anyhow!("exe has no grand-parent"))?
        .parent()
        .ok_or(anyhow!("exe has no great-grand-parent"))?
        .join("conntest.log");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)?;

    let log_config = ConfigBuilder::new().set_time_format_rfc3339().build();

    WriteLogger::init(LevelFilter::Info, log_config, log_file)?;

    Ok(())
}
