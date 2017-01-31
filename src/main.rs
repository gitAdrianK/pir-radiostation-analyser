extern crate hyper;
extern crate time;

mod radio;
mod util;

use std::thread;
use std::time::Duration;

use radio::Radio;
use radio::data::RADIOSTATIONS;
use util::logger::log;

fn main() {
    log("log\\log", "Start Analysing Radiostations");
    for station in RADIOSTATIONS {
        thread::spawn(move || {
            let mut last_song = "".to_string();
            loop {
                // TODO: Save song in hashmap
                // TODO: Serialize hashmap
                let song = station.get_current_song();
                if last_song != song {
                    println!("{:?}", song);
                    last_song = song;
                }
                thread::sleep(Duration::from_secs(60));
            }
        });
    }
    println!("-- Press any key to end program --");
    // stdin blocks mains thread, so we don't need to join the other threads
    std::io::stdin()
                .read_line(&mut String::new())
                .expect("something went seriously wrong :O");
    log("log\\log", "Stop Analysing Radiostations");
}
