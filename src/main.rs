use regex::Regex;

mod app;
mod cli;
mod interactive;
mod prom;
mod ui;
enum Event<I> {
    Input(I),
    Tick,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize the logger
    //TODO in the future, this should be not provided by the user but embedded in the binary
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting the application!");

    // read cli arguments
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
    log::info!("Reading metrics from endpoint: {}", endpoint);

    interactive::show(endpoint.clone(), 10).await;
    Ok(())
}
