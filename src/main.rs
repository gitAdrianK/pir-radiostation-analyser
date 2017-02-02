extern crate hyper;
extern crate time;

mod radio;
mod util;

use std::thread;
use std::time::Duration;

use radio::Radio;
use radio::data::RADIOSTATIONS;
use radio::song::Song;
use util::logger::{log, log_at};

fn main() {
    log("Start Analyzing Radio stations");
    for station in RADIOSTATIONS {
        thread::spawn(move || {
            let mut last_song = Song::empty();
            loop {
                // TODO: Save song in hash-map
                // TODO: Serialize hash-map
                match station.get_current_song() {
                    Some(song) => {
                        if  last_song != song {
                            log_at(
                                &format!("log\\{}", station.shorthand),
                                station.shorthand,
                                &format!("{}: {}", station.name, song),
                            );
                            println!("{}: {}", station.name, song);
                            last_song = song;
                        }
                    },
                    None => {},
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
    log("Stop Analyzing Radio stations");
}
