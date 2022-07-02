use std::{collections::HashSet, error::Error};

use crate::prom::MetricScraper;
use tui::widgets::ListState;

pub enum ElementInFocus {
    MetricHeaders,
    LabelsView,
}

enum Direction {
    Up,
    Down,
}

pub struct App<'a> {
    pub endpoint: &'a str,
    pub scrape_interval: u64,
    pub metric_scraper: MetricScraper,

    pub focus: ElementInFocus,
    pub metric_list_state: ListState,
    pub labels_list_state: ListState,
    pub selected_metric: Option<String>,
    pub selected_label: Option<String>,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(endpoint: &'a str, scrape_interval: u64, metric_scraper: MetricScraper) -> App<'a> {
        let mut app = App {
            endpoint,
            scrape_interval,
            metric_scraper,
            focus: ElementInFocus::MetricHeaders,
            metric_list_state: ListState::default(),
            labels_list_state: ListState::default(),
            selected_metric: None,
            selected_label: None,
            should_quit: false,
        };
        // initialize lists state to index 0
        app.metric_list_state.select(Some(0));
        app.labels_list_state.select(Some(0));
        app
    }

    fn change_selected_metric(&mut self, direction: Direction) -> Result<bool, Box<dyn Error>> {
        let metrics_headers = self
            .metric_scraper
            .get_history_lock()?
            .get_metrics_headers();
        let metrics_headers_len = metrics_headers.len();
        update_list_state_with_direction(
            direction,
            &mut self.metric_list_state,
            metrics_headers_len,
        );
        let selected_index = self
            .metric_list_state
            .selected()
            .expect("a selected metric item");
        let next_selected_metric = metrics_headers.get(selected_index).map(|o| o.clone());
        let different = self.selected_metric != next_selected_metric;
        self.selected_metric = next_selected_metric;

        // reset labels state
        if different {
            self.labels_list_state.select(Some(0));
            self.selected_label = None
        }
        return Ok(different);
    }

    fn change_selected_labels(&mut self, direction: Direction) -> Result<bool, Box<dyn Error>> {
        let selected_metric = self.selected_metric.clone().expect("metric to be selected");
        if let Some(metric) = self
            .metric_scraper
            .get_history_lock()?
            .get_metric(&selected_metric)
        {
            let labels: Vec<String> = metric.time_series.iter().map(|(k, _)| k.clone()).collect();
            let labels_len = labels.len();
            update_list_state_with_direction(direction, &mut self.labels_list_state, labels_len);
            let selected_index = self
                .labels_list_state
                .selected()
                .expect("a selected labels item");
            let next_selected_label = labels.get(selected_index).map(|o| o.clone());
            let different = self.selected_label != next_selected_label;
            self.selected_label = next_selected_label;
            return Ok(different);
        }
        Ok(false)
    }

    pub fn on_down(&mut self) -> Result<(), Box<dyn Error>> {
        let direction = Direction::Down;
        match self.focus {
            ElementInFocus::MetricHeaders => {
                self.change_selected_metric(direction)?;
            }
            ElementInFocus::LabelsView => {
                self.change_selected_labels(direction)?;
            }
        }
        Ok(())
    }

    pub fn on_up(&mut self) -> Result<(), Box<dyn Error>> {
        let direction = Direction::Up;
        match self.focus {
            ElementInFocus::MetricHeaders => {
                self.change_selected_metric(direction)?;
            }
            ElementInFocus::LabelsView => {
                self.change_selected_labels(direction)?;
            }
        }
        Ok(())
    }

    pub fn on_tab(&mut self) -> Result<(), Box<dyn Error>> {
        self.focus = match self.focus {
            ElementInFocus::MetricHeaders => ElementInFocus::LabelsView,
            ElementInFocus::LabelsView => ElementInFocus::MetricHeaders,
        };
        Ok(())
    }
}

fn update_list_state_with_direction(direction: Direction, state: &mut ListState, list_len: usize) {
    match direction {
        Direction::Down => {
            if let Some(selected) = state.selected() {
                if selected >= list_len - 1 {
                    state.select(Some(0));
                } else {
                    state.select(Some(selected + 1));
                }
            }
        }
        Direction::Up => {
            if let Some(selected) = state.selected() {
                if selected > 0 {
                    state.select(Some(selected - 1));
                } else {
                    state.select(Some(list_len - 1));
                }
            }
        }
    }
}
