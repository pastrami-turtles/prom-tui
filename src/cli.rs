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
                .env("PROMTUI_ENDPOINT")
                .value_hint(ValueHint::Url)
                .value_name("ENDPOINT")
                .global(true)
                .takes_value(true)
                .help("Endpoint to scrape")
                .long_help(
                    "Endpoint to scrape with the prom tui cli.",
                )
                .default_value("http://localhost:8080"),
        )
}

#[test]
fn verify() {
    build().debug_assert();
}
