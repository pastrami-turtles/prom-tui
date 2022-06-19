use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use prom::Metric;
use regex::Regex;
use std::borrow::Borrow;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};
use tui_tree_widget::{TreeItem};

mod cli;
mod model;
mod prom;
mod ui;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // setup terminal
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    let metrics: Vec<Metric> = prom::query(endpoint.borrow());

    let mut tree_items = vec![];
    for metric in metrics {
        let mut metric_leaf = TreeItem::new_leaf(metric.details.name);
        for time_series in metric.time_series.keys() {
            metric_leaf.add_child(TreeItem::new_leaf(time_series.clone()));
        }
        tree_items.push(metric_leaf);
    }

    let mut events = model::StatefulTree::with_items(tree_items);
    
    // select first element at start
    events.next();

    //let mut search_input: Vec<char> = vec![];

    let mut app = app::App {
        search_widget: ui::SearchWidget::new(false, vec![]),
        metrics_widget: ui::MetricsWidget::new(true, &mut events),
        graph_widget: ui::GraphWidget::new(false),
        active_widget: ui::ActiveWidget::Metrics,
    };

    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {
                        app.dispatch_input(key.code);
                    },
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
