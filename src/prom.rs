use reqwest;

pub fn query(url: &str) -> Vec<String> {
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
    return resp
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
}
