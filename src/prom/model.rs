use std::collections::HashMap;

use super::parser::extract_labels_key_and_map;

pub struct MetricHistory {
    pub metrics: HashMap<String, Metric>,
}

impl MetricHistory {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.metrics.len() == 0
    }

    pub fn get_metrics_headers(&self) -> Vec<String> {
        self.metrics.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn get_metric(&self, metric_name: &str) -> Option<&Metric> {
        self.metrics.get(metric_name)
    }
}

pub struct SingleScrapeMetric {
    pub name: String,
    pub docstring: String,
    pub value_per_labels: HashMap<String, Sample>,
}

impl SingleScrapeMetric {
    pub fn into_metric(self) -> Metric {
        let mut metric = Metric {
            details: MetricDetails {
                name: self.name,
                docstring: self.docstring,
            },
            time_series: HashMap::new(),
        };
        self.value_per_labels
            .into_iter()
            .for_each(|(labels, sample)| {
                add_time_series_into_metric(labels, &mut metric.time_series, sample);
            });
        metric
    }
}

#[derive(Clone)]
pub struct Metric {
    pub details: MetricDetails,
    pub time_series: HashMap<String, TimeSeries>,
}

#[derive(Clone)]
pub struct MetricDetails {
    pub name: String,
    pub docstring: String,
}

impl Metric {
    pub fn update_time_series(&mut self, value_per_labels: HashMap<String, Sample>) {
        value_per_labels.into_iter().for_each(|(key, value)| {
            if self.time_series.contains_key(&key) {
                self.time_series
                    .get_mut(&key)
                    .expect("should contain the value")
                    .samples
                    .push(value);
            } else {
                add_time_series_into_metric(key, &mut self.time_series, value);
            }
        })
    }
}

#[derive(Clone)]
pub struct TimeSeries {
    pub labels: HashMap<String, String>,
    pub samples: Vec<Sample>,
}

#[derive(Clone)]
pub enum Sample {
    GaugeSample(SingleValueSample),
    CounterSample(SingleValueSample),
    HistogramSample(HistogramSample),
}

#[derive(Clone)]
pub struct SingleValueSample {
    pub timestamp: u64,
    pub value: f64,
}

#[derive(Clone)]
pub struct HistogramSample {
    pub timestamp: u32,
    pub values: Vec<f64>,
}

fn add_time_series_into_metric(
    labels: String,
    time_series: &mut HashMap<String, TimeSeries>,
    sample: Sample,
) {
    let mut labels_map = HashMap::new();
    let key;
    if labels.contains("=") {
        (labels_map, key) = extract_labels_key_and_map(Some(labels));
    } else {
        key = labels;
        labels_map.insert("key".to_string(), "value".to_string());
    }

    time_series.insert(
        key,
        TimeSeries {
            labels: labels_map,
            samples: vec![sample],
        },
    );
}

#[cfg(test)]
mod tests {
    use crate::prom::{
        parser::{decode_single_scrape_metric, split_metric_lines},
        test_data::generate_metric_lines,
    };

    use super::*;

    #[test]
    // TODO eventually at some point this test can be removed. As the logic is tested from the metric scraper.
    fn test_convert_single_scrape_metric_into_metric_and_update_metric() {
        use std::time::{SystemTime, UNIX_EPOCH};

        // simulate first scrape
        let lines = split_metric_lines(generate_metric_lines());
        let mut metrics: Vec<Metric> = Vec::new();
        for part in lines {
            let single_scrape_metric = decode_single_scrape_metric(
                part,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            let name_to_test = single_scrape_metric.name.clone();
            let labels_to_test = match single_scrape_metric.value_per_labels.keys().next() {
                Some(key) => key.clone(),
                None => String::new(),
            };
            let metric = single_scrape_metric.into_metric();
            assert_eq!(metric.details.name, name_to_test);
            assert_eq!(metric.time_series.contains_key(&labels_to_test), true);
            metrics.push(metric);
        }
        // simulate second scrape
        let lines = split_metric_lines(generate_metric_lines());
        for part in lines {
            let single_scrape_metric = decode_single_scrape_metric(
                part,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            // update existing metrics
            let metric_to_update_option = metrics
                .iter_mut()
                .find(|m| m.details.name == single_scrape_metric.name);
            match metric_to_update_option {
                Some(metric_to_update) => {
                    metric_to_update.update_time_series(single_scrape_metric.value_per_labels);
                    metric_to_update
                        .time_series
                        .values()
                        .for_each(|time_series| {
                            assert_eq!(time_series.samples.len(), 2);
                        });
                }
                None => {
                    panic!("no additional metric should be added");
                }
            }
        }
    }
}
