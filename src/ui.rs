use crate::prom::Metric;
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, store: &mut crate::model::MetricStore) {
    let size = f.size();

    let items: Vec<ListItem> = store
        .items
        .iter()
        .map(|metric| match metric {
            Metric::CounterMetric(metric) => ListItem::new(metric.name.as_ref()),
            Metric::GaugeMetric(metric) => ListItem::new(metric.name.as_ref()),
            Metric::HistogramMetric(metric) => ListItem::new(metric.name.as_ref()),
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Metrics").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        );

    f.render_stateful_widget(list, size, &mut store.state);
}
