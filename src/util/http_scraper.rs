use hyper::{Client, Ok};
use util::logger::log;
use std::io::prelude::*;

pub fn  scrape(url: &str, name: &str) -> Option<String> {
    let client = Client::new();
    let mut response = client.get(url).send().unwrap();
    if response.status != Ok {
        log("log\\log", &format!("Error {}, FAILED scraping from {}", response.status, name));
        return None;
    }
    let mut s = String::new();
    let _ = response.read_to_string(&mut s);
    log("log\\log", &format!("Scrapped from {}", name));
    Some(s)
}
