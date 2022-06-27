use crate::prom::MetricScraper;
use crate::ui::{ActiveWidget, GraphWidget, InteractiveWidget, MetricsWidget, SearchWidget};
use crossterm::event::KeyCode;

pub struct App {
    pub metric_scraper: MetricScraper,
    pub search_widget: SearchWidget,
    pub metrics_widget: MetricsWidget,
    pub graph_widget: GraphWidget,
    pub active_widget: ActiveWidget,
}

impl App {
    pub fn new(endpoint: String, scrape_interval: u64) -> Self {
        let metric_scraper = MetricScraper::new(endpoint, scrape_interval);
        let metric_history = metric_scraper.get_history();
        Self {
            search_widget: SearchWidget::new(false, vec![]),
            metrics_widget: MetricsWidget::new(true, metric_history),
            graph_widget: GraphWidget::new(false),
            active_widget: ActiveWidget::Metrics,
            metric_scraper,
        }
    }

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
