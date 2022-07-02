use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Row, Table, TableState},
    Frame,
};

use crate::prom::{Metric, Sample};
use chrono::prelude::*;

use super::graph_data::GraphData;

pub fn draw<B>(
    f: &mut Frame<B>,
    chunk_right: Rect,
    chunk_left: Rect,
    metric: &Metric,
    selected_label: &str,
) where
    B: Backend,
{
    if let Some(graph_data) = GraphData::parse(metric, selected_label) {
        draw_graph(f, chunk_right, &graph_data);
    }
    draw_table(f, chunk_left, metric, &selected_label);
}

#[allow(clippy::cast_precision_loss)]
fn draw_table<B>(f: &mut Frame<B>, area: Rect, metric: &Metric, selected_label: &str)
where
    B: Backend,
{
    let samples = &metric
        .time_series
        .get(selected_label)
        .expect("values for selected label")
        .samples;
    let mut title = format!("History ({})", samples.len());

    let rows = samples.iter().map(|entry| {
        let (timestamp, value) = match entry {
            Sample::GaugeSample(single_value) => (single_value.timestamp, single_value.value),
            Sample::CounterSample(single_value) => (single_value.timestamp, single_value.value),
            _ => unimplemented!(),
        };
        let time = Local.timestamp(timestamp as i64, 0).to_rfc2822();
        Row::new(vec![time, value.to_string()])
    });

    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title(title))
        .header(
            Row::new(vec!["Time", "Value"]).style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Length(50),
            Constraint::Length(15),
            Constraint::Percentage(100),
        ]);

    let mut state = TableState::default();
    state.select(Some(samples.len() - 1));

    f.render_stateful_widget(t, area, &mut state);
}

fn draw_graph<B>(f: &mut Frame<B>, area: Rect, points: &GraphData)
where
    B: Backend,
{
    let datasets = vec![Dataset::default()
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::LightGreen))
        .graph_type(GraphType::Line)
        .data(&points.data)];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Graph").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .labels(vec![
                    Span::raw(points.first_time.format("%H:%M:%S").to_string()),
                    Span::raw(points.last_time.format("%H:%M:%S").to_string()),
                ])
                .bounds([points.x_min, points.x_max]),
        )
        .y_axis(
            Axis::default()
                .labels(vec![
                    Span::raw(points.y_min.to_string()),
                    Span::raw(points.y_max.to_string()),
                ])
                .bounds([points.y_min, points.y_max]),
        );
    f.render_widget(chart, area);
}
