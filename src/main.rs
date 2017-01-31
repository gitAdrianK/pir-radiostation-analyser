extern crate hyper;
extern crate time;

mod radio;
mod util;

use std::fs;
use std::thread;
use std::time::Duration;

use radio::Radio;
use radio::data::RADIOSTATIONS;
use radio::song::Song;
use util::logger::log;

fn main() {
    // Create log directory in case it doesn't exist
    let _ = fs::create_dir("log");
    log("Start Analyzing Radio stations");
    for station in RADIOSTATIONS {
        thread::spawn(move || {
            let mut last_song = Song::empty();
            loop {
                // TODO: Save song in hash-map
                // TODO: Serialize hash-map
                let song = station.get_current_song();
                match song {
                    Some(song) => {
                        if  last_song != song {
                            log(&format!("{}: Song changed", station.name));
                            println!("{}: {}", station.name, song);
                            last_song = song;
                        }
                    },
                    None => {},
                }
                thread::sleep(Duration::from_secs(30));
            }
        });
    }
    println!("-- Press any key to end program --");
    // stdin blocks mains thread, so we don't need to join the other threads
    std::io::stdin()
                .read_line(&mut String::new())
                .expect("something went seriously wrong :O");
    log("Stop Analyzing Radio stations");
}
