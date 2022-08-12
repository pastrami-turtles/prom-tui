use log::error;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Row, Table, TableState, BarChart},
    Frame,
};

use crate::prom::{Metric, MetricType, Sample};
use chrono::prelude::*;

use super::{graph_data::GraphData, histogram_data::{HistogramData}};

pub fn draw<B>(
    f: &mut Frame<B>,
    chunk_right: Rect,
    chunk_left: Rect,
    metric: &Metric,
    selected_label: &str,
) where
    B: Backend,
{
    match metric.details.metric_type {
        MetricType::Histogram => {
            if let Some(histogram_data) = HistogramData::parse(metric, selected_label) {
                draw_histogram_table(f, chunk_left, &histogram_data);
                draw_histogram(f, chunk_right, &histogram_data);
            }
        }
        _ => {
            if let Some(graph_data) = GraphData::parse(metric, selected_label) {
                draw_graph(f, chunk_right, &graph_data);
            }
            draw_table(f, chunk_left, metric, &selected_label);
        }
    }
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
    let title = format!("History ({})", samples.len());

    let rows = samples.iter().map(|entry| {
        let (timestamp, value) = match entry {
            Sample::GaugeSample(single_value) => (single_value.timestamp, single_value.value),
            Sample::CounterSample(single_value) => (single_value.timestamp, single_value.value),
            _ => {
                error!("History table is not implemented for this kind of sample.");
                unimplemented!();
            }
        };
        let time = Local.timestamp(timestamp as i64, 0).to_rfc2822();
        Row::new(vec![time, format!("{:+.4e}", value)])
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
                    Span::raw(format!("{:+.4e}",points.y_min)),
                    Span::raw(format!("{:+.4e}",points.y_max)),
                ])
                .bounds([points.y_min, points.y_max]),
        );
    f.render_widget(chart, area);
}

fn draw_histogram_table<B>(f: &mut Frame<B>, area: Rect, histogram_data: &HistogramData)
where
    B: Backend,
{    let chunks = Layout::default()
    .constraints([Constraint::Percentage(25), Constraint::Min(8)].as_ref())
    .split(area);

    // Draw histogram details
    let title_details = format!("Histogram Details");

    let row_details = [
        Row::new(vec![histogram_data.time.to_rfc2822(), histogram_data.count.to_string(), format!("{:.2}",histogram_data.sum)])
    ];


    let t = Table::new(row_details)
        .block(Block::default().borders(Borders::ALL).title(title_details))
        .header(
            Row::new(vec!["Time", "Count", "Sum"]).style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Length(40),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Percentage(100),
        ]);
    f.render_widget(t, chunks[0]);

    // Draw histogram buckets details
    let title = format!("Histogram Buckets Details");

    let rows = histogram_data.data.iter().map(|entry| {
        Row::new(vec![entry.get_bucket().clone(), entry.get_value().to_string(), format!("{:.2}", entry.get_percentage()), entry.get_inc_per_bucket().to_string(), format!("{:.2}", entry.get_inc_per_bucket_percentage())])
    });

    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title(title))
        .header(
            Row::new(vec!["Bucket", "Count", "Count %", "Inc", "Inc %"]).style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Percentage(100),
        ]);
    f.render_widget(t, chunks[1]);
}

fn draw_histogram<B>(f: &mut Frame<B>, area: Rect, histogram_data: &HistogramData)
where
    B: Backend,
{
    let data: Vec<(&str, u64)>= histogram_data.data.iter().map(|bucket_value|  (bucket_value.get_bucket().as_str(), bucket_value.get_inc_per_bucket())).collect();
    let bar_width = area.width/(data.len()+1) as u16;
    let t = BarChart::default()
        .block(Block::default().title("Histogram").borders(Borders::ALL))
        .data(&data)
        .bar_width(bar_width)
        .bar_style(Style::default().fg(Color::LightGreen))
        .value_style(Style::default().fg(Color::Black).bg(Color::LightGreen));
    f.render_widget(t, area);
}