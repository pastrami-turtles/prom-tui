use regex::Regex;

mod cli;
mod interactive;
mod prom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO in the future, this should be not provided by the user but embedded in the binary
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting the application!");

    log::info!("Reading cli inputs");
    let matches = cli::build().get_matches();
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
