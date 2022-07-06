use crate::logging::app_config;
use regex::Regex;

mod cli;
mod interactive;
mod logging;
mod prom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read cli arguments
    let matches = cli::build().get_matches();

    // initialize the logger
    log4rs::init_config(app_config("log.out", matches.value_of("Logging"))).unwrap();
    log::info!("Starting the application!");

    let regex = Regex::new(":(\\d{2,5})/").unwrap();
    let port_option = matches.value_of("Port");
    let endpoint_option = matches.value_of("Endpoint");
    let endpoint = match port_option {
        Some(port) => endpoint_option
            .map(|e| regex.replace(e, format!(":{port}/", port = port)))
            .unwrap()
            .to_string(),
        None => endpoint_option.unwrap().to_string(),
    };
    let scrape_interval = matches
        .value_of("Scrape-Interval")
        .expect("scrape interval value to be available")
        .parse::<u64>()
        .expect("scrape interval value to be parsable to u64");
    log::info!("Reading metrics from endpoint: {}", endpoint);
    log::info!("Scraping interval is: {}s", scrape_interval);

    // start dashboard
    log::info!("Showing the dashboard");
    interactive::show(endpoint.clone(), scrape_interval).await?;
    Ok(())
}
