use std::{collections::HashSet, error::Error};

use crate::prom::MetricScraper;
use tui_tree_widget::{flatten, TreeState};

use crate::interactive::list;


pub enum ElementInFocus {
  MetricHeaders,
  LabelsView,
}

enum Direction {
  Up,
  Down,
}

enum CursorMove {
  Absolute(usize),
  Relative(Direction),
}

pub struct App<'a> {
  pub endpoint: &'a str,
  pub scrape_interval: u64,
  pub metric_scraper: MetricScraper,

  pub focus: ElementInFocus,
  pub labels_view_state: TreeState,
  pub opened_metrics: HashSet<String>,
  pub selected_metric: Option<String>,
  pub should_quit: bool,
  pub metric_overview_state: TreeState,
  pub metric_stateful_list: list::StatefulList<String>,
}

impl<'a> App <'a> {
  pub fn new(endpoint: &'a str, scrape_interval: u64, metric_scraper: MetricScraper) -> App<'a> {
      App {
          endpoint,
          scrape_interval,
          metric_scraper,
          focus: ElementInFocus::MetricHeaders,
          labels_view_state: TreeState::default(),
          opened_metrics: HashSet::new(),
          selected_metric: None,
          should_quit: false,
          metric_overview_state: TreeState::default(),
          metric_stateful_list: list::StatefulList::with_items(vec![]),
      }
  }

  fn change_selected_metric(&mut self, cursor_move: CursorMove) -> Result<bool, Box<dyn Error>> {
    let metrics_headers = self.metric_scraper.get_history_lock()?.get_metrics_headers();
    self.metric_stateful_list = list::StatefulList::with_items(metrics_headers.clone());
    let current_index = self
    .selected_metric
    .as_ref()
    .and_then(|selected_metric| metrics_headers.iter().position(|o| o == selected_metric));
    let new_index = match cursor_move {
      CursorMove::Absolute(index) => index,
      CursorMove::Relative(direction) => current_index.map_or_else(
          || match direction {
              Direction::Down => 0,
              Direction::Up => usize::MAX,
          },
          |current_index| match direction {
              Direction::Up => current_index.overflowing_sub(1).0,
              Direction::Down => current_index.saturating_add(1) % metrics_headers.len(),
          },
      ),
    }
    .min(metrics_headers.len().saturating_sub(1));
    let next_selected_metric = metrics_headers.get(new_index).map(|o| o.clone());
    let different = self.selected_metric != next_selected_metric;
    self.selected_metric = next_selected_metric;
    Ok(different)
  }

  pub fn on_down(&mut self) -> Result<(), Box<dyn Error>> {
    let direction = Direction::Down;
    match self.focus {
        ElementInFocus::MetricHeaders => {
            self.change_selected_metric(CursorMove::Relative(direction))?;
        }
        _ => {}
    }
    Ok(())
  }

  pub fn on_up(&mut self) -> Result<(), Box<dyn Error>> {
    let direction = Direction::Up;
    match self.focus {
        ElementInFocus::MetricHeaders => {
            self.change_selected_metric(CursorMove::Relative(direction))?;
        }
        _ => {}
    }
    Ok(())
  }
}