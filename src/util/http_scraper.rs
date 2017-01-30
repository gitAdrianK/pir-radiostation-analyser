use hyper::{Client, Ok};
use util::logger::log;
use std::io::prelude::*;

pub fn  scrape(url: &str) -> Option<String> {
    let client = Client::new();
    let mut response = client.get(url).send().unwrap();
    if response.status != Ok {
        log("log\\log", &format!("Error {}, FAILED scraping from {}", response.status, url));
        return None;
    }
    let mut s = String::new();
    // TODO: Find out why this method is not found
    let _ = response.read_to_string(&mut s);
    log("log\\log", &format!("Scrapped from {}", url));
    Some(s)
}
