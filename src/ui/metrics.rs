use crate::prom::Metric;
use crate::ui::metrics_state::StatefulTree;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::Tree;

pub struct MetricsWidget {
    pub state: StatefulTree,
    active: bool,
}

impl MetricsWidget {
    pub fn new(active: bool, metrics: &Vec<Metric>) -> Self {
        let mut state = StatefulTree::with_items(metrics);

        // select first element at start
        state.next();

        MetricsWidget { active, state }
    }
}

impl crate::ui::InteractiveWidget for MetricsWidget {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: &Rect) {
        let tree = Tree::new(self.state.items.as_ref())
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
        f.render_stateful_widget(tree, *area, &mut self.state.state.to_owned());
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn handle_input(&mut self, key_code: crossterm::event::KeyCode) {
        match key_code {
            KeyCode::Down => self.state.next(),
            KeyCode::Up => self.state.previous(),
            KeyCode::Left => self.state.close(),
            KeyCode::Right => self.state.open(),
            _ => {}
        }
    }
}
