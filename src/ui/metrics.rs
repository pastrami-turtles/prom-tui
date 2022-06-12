use tui_tree_widget::Tree;

use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};

use tui_tree_widget::TreeItem;

pub fn create_metrics_widged<'a>(items: &'a Vec<TreeItem>) -> Tree<'a> {
    return Tree::new(items.as_ref())
        .block(Block::default().title("Metrics").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Gray)
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC),
        );
}
