use super::Metric;
use super::MetricDetails;
use super::Sample;
use super::SingleValueSample;
use super::TimeSeries;
use std::collections::HashMap;

pub fn parse(lines: Vec<String>, timestamp: u64) -> Vec<Metric> {
    let parts = split_metric_lines(lines);

    let mut metrics: Vec<Metric> = Vec::new();

    for part in parts {
        let metric = decode_metric(part, timestamp);
        metrics.push(metric);
    }

    return metrics;
}

fn decode_metric(lines: Vec<String>, timestamp: u64) -> Metric {
    let (name, docstring) = extract_name_docstring(lines[0].clone()).unwrap();
    let metric_type = extract_type(lines[1].clone()).unwrap();

    let mut metric = Metric {
        details: MetricDetails {
            name: name,
            docstring: docstring,
        },
        time_series: HashMap::new(),
    };

    match metric_type.as_str() {
        "gauge" => {
            for line in lines.iter().skip(2) {
                let labels = extract_labels(&line);
                let value = extract_value(&line);
                metric.time_series.insert(
                    labels.clone(),
                    TimeSeries {
                        labels: decode_labels(labels.clone()),
                        samples: vec![Sample::GaugeSample(SingleValueSample {
                            timestamp: timestamp,
                            value: value,
                        })],
                    },
                );
            }
        }
        "counter" => {
            for line in lines.iter().skip(2) {
                let labels = extract_labels(&line);
                let value = extract_value(&line);
                metric.time_series.insert(
                    labels.clone(),
                    TimeSeries {
                        labels: decode_labels(labels.clone()),
                        samples: vec![Sample::CounterSample(SingleValueSample {
                            timestamp: timestamp,
                            value: value,
                        })],
                    },
                );
            }
        }
        "histogram" => {
            // TODO
        }
        _ => {}
    }

    return metric;
}

fn split_metric_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut metrics: Vec<Vec<String>> = Vec::new();
    let mut metric_lines: Vec<String> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if metric_lines.len() != 0
            && (index + 1 == lines.len() || lines[index + 1].starts_with("# HELP"))
        {
            metric_lines.push(line.to_string());
            metrics.push(metric_lines);
            metric_lines = Vec::new();
            continue;
        }
        metric_lines.push(line.to_string());
    }

    return metrics;
}

fn extract_name_docstring(line: String) -> Option<(String, String)> {
    let name_desc: String = line.chars().skip(7).take(line.len() - 6).collect();
    let name_desc = name_desc
        .match_indices(" ")
        .nth(0)
        .map(|(index, _)| name_desc.split_at(index))
        .map(|(name, desc)| (String::from(name), String::from(desc.trim())));
    return name_desc;
}

fn extract_type(line: String) -> Option<String> {
    let metric_type = line
        .match_indices(" ")
        .nth(2)
        .map(|(index, _)| line.split_at(index))
        .map(|(_, metric_type)| String::from(metric_type.trim()));
    return metric_type;
}

fn extract_labels(line: &String) -> String {
    let mut labels = line
        .match_indices("{")
        .nth(0)
        .map(|(index, _)| line.split_at(index + 1))
        .map(|(_, right)| String::from(right))
        .unwrap();
    labels = labels
        .match_indices("}")
        .nth(0)
        .map(|(index, _)| labels.split_at(index))
        .map(|(left, _)| String::from(left))
        .unwrap();
    return labels;
}

fn decode_labels(labels: String) -> HashMap<String, String> {
    let parts: Vec<String> = labels.split(",").map(|s| s.to_string()).collect();
    let mut labels = HashMap::new();
    for label in parts {
        let key_value: Vec<String> = label.split("=").map(|s| s.to_string()).collect();
        let value = key_value[1].clone().replace("\"", "");
        labels.insert(key_value[0].clone(), value);
    }
    return labels;
}

fn extract_value(line: &String) -> f64 {
    line.split_whitespace()
        .last()
        .unwrap()
        .parse::<f64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_labels() {
        let labels = decode_labels(String::from("key1=\"value1\",key2=\"0\""));
        assert_eq!(labels.keys().count(), 2);
        assert_eq!(labels.get("key1").unwrap(), "value1");
        assert_eq!(labels.get("key2").unwrap(), "0");
    }

    #[test]
    fn test_extract_name_docstring() {
        let line = String::from("# HELP metric_1 Description of the metric");
        let name_desc = extract_name_docstring(line);
        match name_desc {
            Some((name, description)) => {
                assert_eq!(name, "metric_1");
                assert_eq!(description, "Description of the metric");
            }
            None => panic!("Failed to extract name and description"),
        }
    }

    #[test]
    fn test_extract_type() {
        let line = String::from("# TYPE vectorized_pandaproxy_request_latency histogram");
        let metric_type = extract_type(line);
        match metric_type {
            Some(metric_type) => {
                assert_eq!(metric_type, "histogram");
            }
            None => panic!("Failed to extract metric type"),
        }
    }

    #[test]
    fn test_split_metric_lines() {
        let lines = generate_metric_lines();
        let splitted_lines = split_metric_lines(lines);
        assert_eq!(splitted_lines.len(), 2);
        assert_eq!(splitted_lines[0].len(), 3);
        assert_eq!(splitted_lines[1].len(), 3);
    }

    #[test]
    fn test_parse() {
        let lines = generate_metric_lines();
        let result = self::parse(lines, 1654892036);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].details.name, String::from("metric_1"));
        assert_eq!(
            result[0].details.docstring,
            String::from("Description of the metric")
        );
        assert_eq!(result[0].time_series.contains_key("shard=\"0\""), true);
        assert_eq!(result[1].details.name, String::from("metric_2"));
        assert_eq!(result[1].details.docstring, String::from("Description"));
        let samples = &result[1]
            .time_series
            .get("shard=\"0\",label1=\"test1\"")
            .unwrap()
            .samples[0];
        match samples {
            Sample::CounterSample(sample) => {
                assert_eq!(sample.value, 5.0)
            }
            _ => panic!("Wrong sample type, expected CounterSample"),
        }
    }

    fn generate_metric_lines() -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(String::from("# HELP metric_1 Description of the metric"));
        lines.push(String::from("# TYPE metric_1 gauge"));
        lines.push(String::from("metric_1{shard=\"0\"} 10.000007"));
        lines.push(String::from("# HELP metric_2 Description"));
        lines.push(String::from("# TYPE metric_2 counter"));
        lines.push(String::from("metric_2{shard=\"0\",label1=\"test1\"} 5"));
        return lines;
    }
}
