use super::parse;
use super::Metric;
use reqwest;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn query(url: &str) -> Vec<Metric> {
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    let lines = resp
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    return parse(lines, now);
}
