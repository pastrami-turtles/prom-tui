use clap::{command, Arg, Command, ValueHint};

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn build() -> Command<'static> {
    command!()
        .name("PROM TUI")
        .arg(
            Arg::new("Endpoint")
                .short('e')
                .long("endpoint")
                .env("PROM_ENDPOINT")
                .value_hint(ValueHint::Url)
                .value_name("ENDPOINT")
                .global(true)
                .takes_value(true)
                .help("Prometheus endpoint to scrape")
                .long_help("The Prometheus endpoint used to scrape metrics from.")
                .default_value("http://localhost:8080/metrics")
        )
        .arg(
            Arg::new("Port")
                .short('p')
                .long("port")
                .env("PROM_PORT")
                .value_hint(ValueHint::Other)
                .value_name("PORT")
                .global(false)
                .takes_value(true)
                .use_value_delimiter(false)
                .help("Prometheus endpoint's port number")
                .long_help("The port number used in the default prometheus endpoint. Example: http://localhost:<PORT>/metrics")
                .validator(|v| v.to_string().parse::<u16>())
        )
        .arg(
            Arg::new("Logging")
                .short('l')
                .long("logging")
                .value_name("LOG_LEVEL")
                .global(false)
                .takes_value(true)
                .use_value_delimiter(false)
                .help("Set the logging level")
                .long_help("Set the logging level to one of these values: DEBUG,ERROR,WARN,INFO")
                .validator(|v| match v {
                    "info" | "INFO" | "debug" | "DEBUG" | "error" | "ERROR" | "warn" | "WARN" => Ok(()),
                    _ => Err("the value should match the following: DEBUG,ERROR,WARN,INFO")
                })
        )
}

#[test]
fn verify() {
    build().debug_assert();
}
