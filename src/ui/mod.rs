use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

mod graph;
pub use self::graph::GraphWidget;
mod metrics;
pub use self::metrics::MetricsWidget;
mod metrics_state;
mod search;
pub use self::search::SearchWidget;
mod style;

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    //store: &mut crate::model::StatefulTree,
    app: &mut crate::app::App,
) {
    let header_content = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.size());

    app.search_widget.render(f, &header_content[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(header_content[1]);
    // TODO
    app.metrics_widget.update_state();
    app.metrics_widget.render(f, &chunks[0]);
    app.graph_widget.render(f, &chunks[1]);
}

pub enum ActiveWidget {
    Search,
    Metrics,
    Graph,
}

pub trait InteractiveWidget {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: &Rect);
    fn set_active(&mut self, active: bool);
    fn handle_input(&mut self, key_code: KeyCode);
}
