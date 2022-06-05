use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io, time::{Duration, Instant}};
use tui::{backend::CrosstermBackend, Terminal};

mod model;
mod prom;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    let default_endpoint = &"http://localhost:8080".to_string();
    let query = args.get(1).unwrap_or_else(|| default_endpoint);
    let lines: Vec<String> = prom::query(query);


    let metric_names: Vec<String> = lines.iter()
        .filter(|line| line.starts_with("# HELP "))
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            parts[2].to_string()
        })
        .collect();
    let mut events = model::MetricStore::new(metric_names);

    // select first element at start
    events.next();

    loop {
        terminal.draw(|f| ui::render(f, &mut events))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => events.next(),
                    KeyCode::Up => events.previous(),
                    _ => {}
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
