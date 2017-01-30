extern crate hyper;
extern crate time;

mod radio;
mod util;

use util::logger::log;
use util::http_scraper::scrape;

fn main() {
    // Tmp testing
    log("log\\log", "Start Analysing Radiostations");
    let s = scrape("http://www.wdr.de/radio/radiotext/streamtitle_1live.txt");
    match s {
        Some(msg) => println!("{:?}", msg),
        None => println!("Nothing"),
    }
}
