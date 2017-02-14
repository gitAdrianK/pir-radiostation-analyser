use hyper::Client;
use std::io::prelude::*;
use util::logger::log;

/// Scrapes a website
/// The adress may not be https, only http works
pub fn scrape(url: &str, name: &str) -> Option<String> {
    let client = Client::new();
    let mut response = match client.get(url).send() {
        Ok(resp) => resp,
        Err(e) => {
            log(&format!("Error {}, FAILED scraping from {}", e, name));
            return None
        },
    };
    if response.status != ::hyper::Ok {
        log(&format!("Error {}, FAILED scraping from {}", response.status, name));
        return None;
    }
    let mut s = String::new();
    let _ = response.read_to_string(&mut s);
    //log(&format!("Scrapped from {}", name));
    Some(s)
}
