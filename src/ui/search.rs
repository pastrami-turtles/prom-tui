use tui::{
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn create_search_widget() -> Paragraph<'static> {
    let lines = Text::from("");
    let component =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Search"));
    return component;
}
