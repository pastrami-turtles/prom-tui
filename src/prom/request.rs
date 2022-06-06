use super::parse;
use super::Metric;
use reqwest;

pub fn query(url: &str) -> Vec<Metric> {
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
    let lines = resp
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    return parse(lines);
}
