use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use std::error::Error;

use crate::interactive::app::{App, ElementInFocus};



pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
  let chunks = Layout::default()
      .constraints([Constraint::Length(2 + 3), Constraint::Min(8)].as_ref())
      .split(f.size());
  draw_info_header(f, chunks[0], app);
  // draw_main(f, chunks[1], app)?;
  Ok(())
}

fn draw_info_header<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let endpoint = format!("Metrics endpoint: {}", app.endpoint);
    let scrape_interval = format!("Scraping interval: {}s", app.scrape_interval);
    let mut text = vec![Spans::from(endpoint), Spans::from(scrape_interval)];

    // if let Some(err) = app.mqtt_thread.has_connection_err().unwrap() {
    //     text.push(Spans::from(Span::styled(
    //         format!("MQTT Connection Error: {}", err),
    //         Style::default()
    //             .fg(Color::Red)
    //             .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
    //     )));
    // }

    if let Some(selected_metric) = &app.selected_metric {
        text.push(Spans::from(format!("Selected metric: {}", selected_metric)));
    }

    let title = format!("PROM TUI {}", env!("CARGO_PKG_VERSION"));
    let block = Block::default().borders(Borders::ALL).title(title);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

// fn draw_main<B>(f: &mut Frame<B>, area: Rect, app: &mut App) -> Result<(), Box<dyn Error>>
// where
//     B: Backend,
// {
//     let history = app.metric_scraper.get_history_lock()?;
//     let tree_items = history.to_tte();

//     // Move opened_topics over to TreeState
//     app.topic_overview_state.close_all();
//     for topic in &app.opened_topics {
//         app.topic_overview_state
//             .open(history.get_tree_identifier(topic).unwrap_or_default());
//     }

//     // Ensure selected topic is selected index
//     app.topic_overview_state.select(
//         app.selected_topic
//             .as_ref()
//             .and_then(|selected_topic| history.get_tree_identifier(selected_topic))
//             .unwrap_or_default(),
//     );

//     #[allow(clippy::option_if_let_else)]
//     let overview_area = if let Some(selected_topic) = &app.selected_topic {
//         if let Some(topic_history) = history.get(selected_topic) {
//             let chunks = Layout::default()
//                 .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
//                 .direction(Direction::Horizontal)
//                 .split(area);

//             draw_details(
//                 f,
//                 chunks[1],
//                 topic_history,
//                 matches!(app.focus, ElementInFocus::JsonPayload),
//                 &mut app.json_view_state,
//             );

//             chunks[0]
//         } else {
//             area
//         }
//     } else {
//         area
//     };

//     draw_overview(
//         f,
//         overview_area,
//         &tree_items,
//         matches!(app.focus, ElementInFocus::TopicOverview),
//         &mut app.topic_overview_state,
//     );
//     Ok(())
// }