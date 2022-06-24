use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
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
use tokio::sync::{broadcast, mpsc};
use tokio::task;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod cli;
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

    // setup terminal
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut metrics: Vec<Metric> = prom::query(endpoint.borrow()).await;
    let mut app = app::App::new(&mut metrics);

    // Set up an input loop using TUI and Crossterm
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);
    let (notify_shutdown, _) = broadcast::channel(1);
    let mut notify_shutdown_rx1 = notify_shutdown.subscribe();
    let (tx, mut rx) = mpsc::channel(1);
    log::info!("Spawning input loop...");
    task::spawn(async move {
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout).expect("that poll works") {
                if let CEvent::Key(key) = event::read().expect("that can read events") {
                    if let Err(e) = tx.send(Event::Input(key)).await {
                        log::error!("Error sending event: {}", e);
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                tokio::select! {
                    sended = tx.send(Event::Tick) => {
                        if let Err(e) = sended {
                            log::error!("Error sending tick: {}", e);
                        }
                    }
                    _ = notify_shutdown_rx1.recv() => {
                        log::info!("Received shutdown signal");
                        drop(tx);
                        break;
                    }
                }
                last_tick = Instant::now();
            }
        }
    });

    //render loop, which calls terminal.draw() on every iteration.
    log::info!("Starting render loop...");
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        match rx.recv().await {
            Some(Event::Input(event)) => match event.code {
                KeyCode::Char('q') => {
                    log::info!("Shuting down...");
                    if let Err(e) = notify_shutdown.send(()) {
                        log::error!("Error sending shutdown signal: {}", e);
                    }
                    break;
                }
                _ => app.dispatch_input(event.code),
            },
            Some(Event::Tick) => {}
            None => {}
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
