use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

pub struct GraphWidget {
    active: bool,
}

impl GraphWidget {
    pub fn new(active: bool) -> Self {
        GraphWidget { active: active }
    }
}

impl crate::ui::InteractiveWidget for GraphWidget {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: &Rect) {
        f.render_widget(
            Block::default()
                .title(super::style::create_styled_title("Graph", self.active))
                .borders(Borders::ALL),
            *area,
        );
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn handle_input(&mut self, key_code: KeyCode) {
        match key_code {
            _ => {}
        }
    }
}
