use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

mod graph;
mod metrics;
mod search;

pub fn render<B: Backend>(f: &mut Frame<B>, store: &mut crate::model::StatefulTree) {
    let header_content = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.size());

    let search_widget = search::create_search_widget();
    f.render_widget(search_widget, header_content[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(header_content[1]);

    let metrics_widget = metrics::create_metrics_widged(&store.items);
    f.render_stateful_widget(metrics_widget, chunks[0], &mut store.state);

    let graph_widget = graph::create_graph_widget();
    f.render_widget(graph_widget, chunks[1]);
}
