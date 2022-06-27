use std::sync::{Arc, RwLockReadGuard};

use crate::prom::{Metric, MetricHistory};
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
    metric_history: Arc<std::sync::RwLock<MetricHistory>>,
}

impl MetricsWidget {
    pub fn new(active: bool, metric_history: Arc<std::sync::RwLock<MetricHistory>>) -> Self {
        let mut state = StatefulTree::new();

        // select first element at start
        //state.next();

        MetricsWidget {
            state,
            active,
            metric_history,
        }
    }

    // TODO
    pub fn update_state(&mut self) {
        let history = self
            .metric_history
            .read()
            .map_err(|err| anyhow::anyhow!("failed to aquire lock of metric history: {}", err))
            .unwrap();
        if history.is_empty() {
            return;
        }
        let metrics: Vec<Metric> = history.metrics.values().cloned().collect();
        self.state = StatefulTree::with_items(&metrics);
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
