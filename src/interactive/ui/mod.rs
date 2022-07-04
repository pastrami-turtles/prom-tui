use std::error::Error;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap};
use tui::Frame;

use crate::interactive::app::{App, ElementInFocus};
use crate::prom::Metric;

mod graph_data;
mod history;
mod search;
mod style;

const fn focus_color(has_focus: bool) -> Color {
    if has_focus {
        Color::LightGreen
    } else {
        Color::Gray
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    let chunks = Layout::default()
        .constraints([Constraint::Length(2 + 3), Constraint::Min(8)].as_ref())
        .split(f.size());
    draw_info_header(f, chunks[0], app);
    draw_main(f, chunks[1], app)?;
    Ok(())
}

fn draw_info_header<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let endpoint = format!("Metrics endpoint: {}", app.endpoint);
    let scrape_interval = format!("Scraping interval: {}s", app.scrape_interval);
    let mut text = vec![Spans::from(endpoint), Spans::from(scrape_interval)];

    let has_error = app
        .metric_scraper
        .get_has_error_lock()
        .expect("to get has_error_lock");
    if *has_error {
        text.push(Spans::from(Span::styled(
            format!("Prom-tui is not able to scrape the metrics endpoint provided. Please check that the endpoint provided is reachable."),
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
        )));
    }

    if let Some(selected_metric) = &app.selected_metric {
        text.push(Spans::from(format!("Selected metric: {}", selected_metric)));
    }

    let title = format!("PROM TUI {}", env!("CARGO_PKG_VERSION"));
    let block = Block::default().borders(Borders::ALL).title(title);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_main<B>(f: &mut Frame<B>, area: Rect, app: &mut App) -> Result<(), Box<dyn Error>>
where
    B: Backend,
{
    let metric_headers = app.metric_scraper.get_history_lock()?.get_metrics_headers();

    #[allow(clippy::option_if_let_else)]
    let metric_headers_area = if let Some(selected_metric) = &app.selected_metric {
        if let Some(metric) = app
            .metric_scraper
            .get_history_lock()?
            .get_metric(selected_metric)
        {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
                .direction(Direction::Horizontal)
                .split(area);

            let chunks_left = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Vertical)
                .split(chunks[0]);

            draw_details(
                f,
                chunks[1],
                chunks_left[1],
                metric,
                matches!(app.focus, ElementInFocus::LabelsView),
                &mut app.labels_list_state,
                &app.selected_label,
            );
            chunks_left[0]
        } else {
            area
        }
    } else {
        area
    };

    draw_list(
        f,
        metric_headers_area,
        &metric_headers,
        matches!(app.focus, ElementInFocus::MetricHeaders),
        &&app.selected_metric,
        &mut app.metric_list_state,
        "Metrics",
    );

    Ok(())
}

fn draw_list<B>(
    f: &mut Frame<B>,
    area: Rect,
    items: &[String],
    has_focus: bool,
    selected_label_option: &Option<String>,
    state: &mut ListState,
    title_prefix: &str,
) where
    B: Backend,
{
    if let Some(selected_label) = selected_label_option {
        // if the list is updated we need to be sure that the state index is still point to the correct item
        let current_index = items.iter().position(|a| a == selected_label).expect("index to be found");
        let state_index = state.selected().expect("state index to be present");
        if state_index != current_index {
            state.select(Some(current_index))
        }
    }

    let title = format!("{} ({})", title_prefix, items.len());
    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(title)
        .border_type(BorderType::Plain);
    let list_item: Vec<ListItem> = items
        .iter()
        .map(|header| {
            ListItem::new(Spans::from(vec![Span::styled(
                header.clone(),
                Style::default(),
            )]))
        })
        .collect();
    let focus_color = focus_color(has_focus);
    let list = List::new(list_item).block(list_block).highlight_style(
        Style::default()
            .bg(focus_color)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    f.render_stateful_widget(list, area, state);
}

fn draw_details<B>(
    f: &mut Frame<B>,
    chunk_right: Rect,
    chunk_left: Rect,
    metric: &Metric,
    is_in_focus: bool,
    labels_state: &mut ListState,
    selected_label_option: &Option<String>,
) where
    B: Backend,
{
    let time_series_keys: Vec<String> = metric.time_series.iter().map(|(k, _)| k.clone()).collect();
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(25), Constraint::Min(16)].as_ref())
        .split(chunk_right);
    draw_list(
        f,
        chunks[0],
        &time_series_keys,
        is_in_focus,
        selected_label_option,
        labels_state,
        "Labels",
    );
    if let Some(selected_label) = selected_label_option {
        history::draw(f, chunks[1], chunk_left, metric, selected_label);
    }
}
