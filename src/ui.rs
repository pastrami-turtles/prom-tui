use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::{Tree};

pub fn render<B: Backend>(f: &mut Frame<B>, store: &mut crate::model::StatefulTree) {
    let size = f.size();

    let items = Tree::new(store.items.clone())
    .block(Block::default().title("Metrics").borders(Borders::ALL))
    .highlight_style(
        Style::default()
            .bg(Color::Gray)
            .fg(Color::White)         
            .add_modifier(Modifier::ITALIC),
    );

    f.render_stateful_widget(items, size, &mut store.state);
}
