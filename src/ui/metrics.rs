use std::{borrow::Borrow, sync::Arc};

use crate::prom::{Metric, MetricHistory};
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::{flatten, get_identifier_without_leaf, Tree, TreeItem, TreeState};

pub struct MetricsWidget {
    active: bool,
    metric_history: Arc<std::sync::RwLock<MetricHistory>>,
    ui_state: TreeState,
}

impl MetricsWidget {
    pub fn new(active: bool, metric_history: Arc<std::sync::RwLock<MetricHistory>>) -> Self {
        MetricsWidget {
            active,
            metric_history,
            ui_state: TreeState::default(),
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
    }

    fn tree_items_from_metrics(&self) -> Vec<TreeItem> {
        self.metric_history
            .read()
            .unwrap()
            .metrics
            .iter()
            .map(|kv| {
                let mut metric_leaf = TreeItem::new_leaf(kv.1.details.name.clone());
                for time_series in kv.1.time_series.keys() {
                    metric_leaf.add_child(TreeItem::new_leaf(time_series.clone()));
                }
                metric_leaf
            })
            .collect()
    }

    fn move_up_down(&mut self, down: bool) {
        let items = &self.tree_items_from_metrics();
        let visible = flatten(&self.ui_state.get_all_opened(), items);
        let current_identifier = self.ui_state.selected();
        let current_index = visible
            .iter()
            .position(|o| o.identifier == current_identifier);
        let new_index = current_index.map_or(0, |current_index| {
            if down {
                current_index.saturating_add(1)
            } else {
                current_index.saturating_sub(1)
            }
            .min(visible.len() - 1)
        });
        let new_identifier = visible.get(new_index).unwrap().identifier.clone();
        self.ui_state.select(new_identifier);
    }

    pub fn select_next(&mut self) {
        self.move_up_down(true);
    }

    pub fn select_previous(&mut self) {
        self.move_up_down(false);
    }

    pub fn close_selected(&mut self) {
        let selected = self.ui_state.selected();
        if !self.ui_state.close(&selected) {
            let (head, _) = get_identifier_without_leaf(&selected);
            self.ui_state.select(head);
        }
    }

    pub fn open_selected(&mut self) {
        self.ui_state.open(self.ui_state.selected());
    }
}

impl crate::ui::InteractiveWidget for MetricsWidget {
    fn render<B: Backend>(&self, f: &mut Frame<B>, area: &Rect) {
        let tree = Tree::new(self.tree_items_from_metrics())
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
        f.render_stateful_widget(tree, *area, &mut self.ui_state.to_owned());
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn handle_input(&mut self, key_code: crossterm::event::KeyCode) {
        match key_code {
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Left => self.close_selected(),
            KeyCode::Right => self.open_selected(),
            _ => {}
        }
    }
}
