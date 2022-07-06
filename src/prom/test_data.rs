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
    return lines;
}
