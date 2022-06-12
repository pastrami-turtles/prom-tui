use tui::widgets::{Block, Borders};

pub fn create_graph_widget() -> Block<'static> {
    return Block::default().title("Graph").borders(Borders::ALL);
}
