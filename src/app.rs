use crate::ui::ActiveWidget;
use crate::ui::InteractiveWidget;
use crossterm::event::KeyCode;

pub struct App<'a> {
    pub search_widget: crate::ui::SearchWidget,
    pub metrics_widget: crate::ui::MetricsWidget<'a>,
    pub graph_widget: crate::ui::GraphWidget,
    pub active_widget: ActiveWidget,
}

impl<'a> App<'a> {
    pub fn dispatch_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Tab => {
                self.next_component();
            }
            KeyCode::BackTab => {
                self.previous_component();
            }
            _ => match self.active_widget {
                ActiveWidget::Search => self.search_widget.handle_input(key_code),
                ActiveWidget::Metrics => self.metrics_widget.handle_input(key_code),
                ActiveWidget::Graph => self.graph_widget.handle_input(key_code),
            },
        }
    }

    fn next_component(&mut self) {
        match self.active_widget {
            ActiveWidget::Search => {
                self.active_widget = ActiveWidget::Metrics;
                self.search_widget.set_active(false);
                self.metrics_widget.set_active(true);
            }
            ActiveWidget::Metrics => {
                self.active_widget = ActiveWidget::Graph;
                self.metrics_widget.set_active(false);
                self.graph_widget.set_active(true);
            }
            ActiveWidget::Graph => {
                self.active_widget = ActiveWidget::Search;
                self.graph_widget.set_active(false);
                self.search_widget.set_active(true);
            }
        }
    }

    fn previous_component(&mut self) {
        match self.active_widget {
            ActiveWidget::Search => {
                self.active_widget = ActiveWidget::Graph;
                self.search_widget.set_active(false);
                self.graph_widget.set_active(true);
            }
            ActiveWidget::Metrics => {
                self.active_widget = ActiveWidget::Search;
                self.metrics_widget.set_active(false);
                self.search_widget.set_active(true);
            }
            ActiveWidget::Graph => {
                self.active_widget = ActiveWidget::Metrics;
                self.graph_widget.set_active(false);
                self.metrics_widget.set_active(true);
            }
        }
    }
}
