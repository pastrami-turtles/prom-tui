use super::GaugeMetric;
use super::CounterMetric;
use super::HistogramMetric;
use super::Metric;

pub fn parse(lines: Vec<String>) -> Vec<Metric> {
    let parts = split_metric_lines(lines);

    let mut metrics: Vec<Metric> = Vec::new();

    for part in parts {
        let metric = decode_metric(part);
        match metric {
            Some(metric) => metrics.push(metric),
            None => continue,
        }
    }

    return metrics;
}

fn decode_metric(lines: Vec<String>) -> Option<Metric> {
    let name_desc = decode_help(lines[0].clone()).unwrap();
    let metric_type = decode_type(lines[1].clone()).unwrap();

    let metric: Option<Metric>;
    match metric_type.as_str() {
        "gauge" => {
            metric = Some(Metric::GaugeMetric(GaugeMetric {
                name: name_desc.0,
                description: name_desc.1,
                value: 0.0,
            }));
        }
        "counter" => {
            metric = Some(Metric::CounterMetric(CounterMetric {
                name: name_desc.0,
                description: name_desc.1,
                value: 0.0,
            }));
        }
        "histogram" => {
            metric = Some(Metric::HistogramMetric(HistogramMetric {
                name: name_desc.0,
                description: name_desc.1,
            }));
        }
        _ => metric = None,
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

fn decode_help(line: String) -> Option<(String, String)> {
    let name_desc: String = line.chars().skip(7).take(line.len() - 6).collect();
    let name_desc = name_desc
        .match_indices(" ")
        .nth(0)
        .map(|(index, _)| name_desc.split_at(index))
        .map(|(name, desc)| (String::from(name), String::from(desc.trim())));
    return name_desc;
}

fn decode_type(line: String) -> Option<String> {
    let metric_type = line
        .match_indices(" ")
        .nth(2)
        .map(|(index, _)| line.split_at(index))
        .map(|(_, metric_type)| String::from(metric_type.trim()));
    return metric_type;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_help() {
        let line = String::from("# HELP metric_1 Description of the metric");
        let name_desc = decode_help(line);
        match name_desc {
            Some((name, description)) => {
                assert_eq!(name, "metric_1");
                assert_eq!(description, "Description of the metric");
            }
            None => panic!("Failed to extract name and description"),
        }
    }

    #[test]
    fn test_decode_type() {
        let line = String::from("# TYPE vectorized_pandaproxy_request_latency histogram");
        let metric_type = decode_type(line);
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

        let result = self::parse(lines);

        assert_eq!(result.len(), 2);
        match &result[0] {
            Metric::GaugeMetric(metric) => {
                assert_eq!(metric.name, String::from("metric_1"));
                assert_eq!(
                    metric.description,
                    String::from("Description of the metric")
                );
            }
            _ => {
                panic!("Wrong metric type");
            }
        }
        match &result[1] {
            Metric::CounterMetric(metric) => {
                assert_eq!(metric.name, String::from("metric_2"));
                assert_eq!(metric.description, String::from("Description"));
            }
            _ => {
                panic!("Wrong metric type");
            }
        }
    }

    fn generate_metric_lines() -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(String::from("# HELP metric_1 Description of the metric"));
        lines.push(String::from("# TYPE metric_1 gauge"));
        lines.push(String::from("metric_1{shard=\"0\"} 10.000007"));
        lines.push(String::from("# HELP metric_2 Description"));
        lines.push(String::from("# TYPE metric_2 counter"));
        lines.push(String::from("metric_2{shard=\"0\"} 5"));
        return lines;
    }
}
