use crate::model::StatefulTree;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::Tree;

pub struct MetricsWidget<'a> {
    pub store: &'a mut StatefulTree,
    active: bool,
}

impl<'a> MetricsWidget<'a> {
    pub fn new(active: bool, store: &'a mut StatefulTree) -> Self {
        MetricsWidget {
            active: active,
            store: store,
        }
    }
}

impl crate::ui::InteractiveWidget for MetricsWidget<'_> {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: &Rect) {
        let tree = Tree::new(self.store.items.as_ref())
            .block(
                Block::default()
                    .title(super::style::create_styled_title("Metrics", self.active))
                    .borders(Borders::ALL),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Gray)
                    .fg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            );
        f.render_stateful_widget(tree, *area, &mut self.store.state.to_owned());
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn handle_input(&mut self, key_code: crossterm::event::KeyCode) {
        match key_code {
            KeyCode::Down => self.store.next(),
            KeyCode::Up => self.store.previous(),
            KeyCode::Left => self.store.close(),
            KeyCode::Right => self.store.open(),
            _ => {}
        }
    }
}
