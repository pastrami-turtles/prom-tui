use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct SearchWidget {
    active: bool,
    pub input: Vec<char>,
}

impl SearchWidget {
    pub fn new(active: bool, input: Vec<char>) -> Self {
        SearchWidget {
            active: active,
            input: input,
        }
    }
}

impl crate::ui::InteractiveWidget for SearchWidget {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: &Rect) {
        let component = Paragraph::new(Text::from(self.input.iter().collect::<String>())).block(
            Block::default()
                .borders(Borders::ALL)
                .title(super::style::create_styled_title("Search", self.active)),
        );
        f.render_widget(component, *area);
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn handle_input(&mut self, key_code: crossterm::event::KeyCode) {
        match key_code {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Esc => {
                self.input.clear();
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }
}
