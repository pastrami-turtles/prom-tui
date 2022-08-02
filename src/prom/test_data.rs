pub fn generate_metric_lines() -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(String::from("# HELP metric_1 Description of the metric"));
    lines.push(String::from("# TYPE metric_1 gauge"));
    lines.push(String::from("metric_1{shard=\"0\"} 10.0"));
    lines.push(String::from("# HELP metric_2 Description"));
    lines.push(String::from("# TYPE metric_2 counter"));
    lines.push(String::from("metric_2{shard=\"0\",label1=\"test1\"} 5"));
    lines.push(String::from("# HELP incoming_requests Incoming Requests"));
    lines.push(String::from("# TYPE incoming_requests counter"));
    lines.push(String::from("incoming_requests 10"));
    lines.push(String::from("# HELP connected_clients Connected Clients"));
    lines.push(String::from("# TYPE connected_clients gauge"));
    lines.push(String::from("connected_clients 3"));
    lines.push(String::from("# HELP response_time Response Times"));
    lines.push(String::from("# TYPE response_time histogram"));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"0.005\"} 3",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"0.01\"} 4",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"0.025\"} 13",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"0.05\"} 25",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"0.1\"} 57",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"0.25\"} 148",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"0.5\"} 319",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"production\",le=\"+Inf\"} 6563",
    ));
    lines.push(String::from(
        "response_time_sum{env=\"production\"} 32899.06535799631",
    ));
    lines.push(String::from("response_time_count{env=\"production\"} 6563"));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"0.005\"} 4",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"0.01\"} 4",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"0.025\"} 13",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"0.05\"} 31",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"0.1\"} 56",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"0.25\"} 168",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"0.5\"} 338",
    ));
    lines.push(String::from(
        "response_time_bucket{env=\"testing\",le=\"+Inf\"} 6451",
    ));
    lines.push(String::from(
        "response_time_sum{env=\"testing\"} 32157.055112958977",
    ));
    lines.push(String::from("response_time_count{env=\"testing\"} 6451"));
    return lines;
}
