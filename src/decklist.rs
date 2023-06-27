pub fn read_from_file(filename: &str) -> Vec<String> {
    use std::fs;
    fs::read_to_string(filename)
        .expect("Invalid filename.")
        .split("\n")
        .map(|t| t.to_string())
        .collect()
}

pub fn read_from_url(url: &str) -> Vec<String> {
    use reqwest::blocking::get;
    get(url)
        .expect("Invalid URL.")
        .text()
        .unwrap()
        .split("\n")
        .map(|t| t.to_string())
        .collect()
}
