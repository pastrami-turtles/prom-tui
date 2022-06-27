use super::{
    model::MetricHistory,
    parser::{decode_single_scrape_metric, split_metric_lines},
};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task;

type MetricHistoryArc = Arc<RwLock<MetricHistory>>;
pub struct MetricScraper {
    metrics_history: MetricHistoryArc,
}

impl MetricScraper {
    pub fn new(url: String, scrape_interval: u64) -> Self {
        let metric_history = MetricHistoryArc::new(RwLock::new(MetricHistory::new()));

        {
            let history = Arc::clone(&metric_history);
            task::spawn(async move {
                scrape_metric_endpoint(&url, &history, scrape_interval).await;
            });
        }
        Self {
            metrics_history: metric_history,
        }
    }

    pub fn get_history(&self) -> Arc<std::sync::RwLock<MetricHistory>> {
        self.metrics_history.clone()
    }
}

async fn scrape_metric_endpoint(url: &str, history: &MetricHistoryArc, scrape_interval: u64) {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(scrape_interval);
    let mut must_scrape = true;

    loop {
        // scrape and update history
        if must_scrape {
            let splitted_metrics = get_splitted_metrics_from_endpoint(&url).await;
            update_history_with_new_scrape(history, splitted_metrics);
            // set must_scrape to false to avoid scraping again until the next tick
            must_scrape = false;
        }

        // if time has elapsed since last tick, allow to scrape the endpoint again and update history
        if last_tick.elapsed() >= tick_rate {
            must_scrape = true;
            // reset last tick
            last_tick = Instant::now();
        }
        //TODO ad signal to stop the loop when the app quit.
    }
}

fn update_history_with_new_scrape(history: &MetricHistoryArc, splitted_metrics: Vec<Vec<String>>) {
    let mut history_guard = history.write().unwrap();
    let timestamp = get_timestamp_unix_epoch();
    for part in splitted_metrics {
        let single_scrape_metric = decode_single_scrape_metric(part, timestamp);
        let metric_to_update_option = history_guard.metrics.get_mut(&single_scrape_metric.name);
        match metric_to_update_option {
            Some(metric_to_update) => {
                log::debug!("updating metric: {}", metric_to_update.details.name);
                metric_to_update.update_time_series(single_scrape_metric.value_per_labels);
            }
            None => {
                let metric = single_scrape_metric.into_metric();
                log::debug!(
                    "add metric '{}' for the first time to the history.",
                    metric.details.name
                );
                history_guard
                    .metrics
                    .insert(metric.details.name.clone(), metric);
            }
        }
    }
}

fn get_timestamp_unix_epoch() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    timestamp
}

async fn get_splitted_metrics_from_endpoint(url: &str) -> Vec<Vec<String>> {
    let resp = reqwest::get(url)
        .await
        .expect("a ok response from scraping endpoint!")
        .text()
        .await
        .expect("a text response from scraping endpoint!");
    let lines = resp
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    return split_metric_lines(lines);
}

#[cfg(test)]
mod tests {
    use crate::prom::{parser::split_metric_lines, test_data::generate_metric_lines};

    use super::*;

    #[test]
    fn test_update_history_with_new_scrape() {
        // initialize data structure
        let metric_history = MetricHistoryArc::new(RwLock::new(MetricHistory::new()));
        // simulate first scrape
        let lines = split_metric_lines(generate_metric_lines());
        let history = Arc::clone(&metric_history);
        update_and_assert(history, lines, 1);

        // simulate second scrape
        let lines = split_metric_lines(generate_metric_lines());
        let history = Arc::clone(&metric_history);
        update_and_assert(history, lines, 2);
    }

    fn update_and_assert(
        history: MetricHistoryArc,
        lines: Vec<Vec<String>>,
        expected_length: usize,
    ) {
        // update history
        update_history_with_new_scrape(&history, lines);

        // assert results
        let history_read_guard = history
            .read()
            .map_err(|err| anyhow::anyhow!("failed to aquire lock of metric history: {}", err));
        history_read_guard
            .expect("to access the history")
            .metrics
            .iter()
            .for_each(|m| {
                m.1.time_series.values().for_each(|time_series| {
                    assert_eq!(time_series.samples.len(), expected_length);
                });
            });
    }
}
