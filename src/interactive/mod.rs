use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use tokio::{
    sync::{broadcast, mpsc},
    task,
};

use std::{
    io,
    time::{Duration, Instant},
};

use tui::{backend::CrosstermBackend, Terminal};

use crate::{interactive::app::App, prom::MetricScraper};
mod app;
mod ui;

enum Event<I> {
    Input(I),
    Tick,
}

pub async fn show(endpoint: String, scrape_interval: u64) -> Result<(), Box<dyn Error>> {
    let metric_scraper = MetricScraper::new(endpoint.clone(), scrape_interval.clone());
    let mut app = App::new(&endpoint, scrape_interval, metric_scraper);
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let mut app = app_old::App::new(endpoint.clone(), 10);

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
        terminal.draw(|f| ui::draw(f, &mut app).expect("failed to draw ui"))?;

        match rx.recv().await {
            Some(Event::Input(event)) => match event.code {
                KeyCode::Char('q') => {
                    log::info!("Shuting down...");
                    if let Err(e) = notify_shutdown.send(()) {
                        log::error!("Error sending shutdown signal: {}", e);
                    }
                    break;
                }
                KeyCode::Down => app.on_down()?,
                KeyCode::Up => app.on_up()?,
                KeyCode::Tab | KeyCode::BackTab => app.on_tab()?,
                _ => {} //app.dispatch_input(event.code),
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
