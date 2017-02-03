extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate time;

mod radio;
mod util;

use std::collections::LinkedList;
use std::thread;
use std::time::Duration;

use radio::Radio;
use radio::data::RADIOSTATIONS;
use radio::song::Song;
use util::song_map::SongMap;
use util::logger::{log, log_at};

fn main() {
    log("Start Analyzing Radio stations");
    run_radio_analyser();
    println!("-- Press any key to end program --");
    // stdin blocks mains thread, so we don't need to join the other threads
    std::io::stdin()
                .read_line(&mut String::new())
                .expect("something went seriously wrong :O");
    log("Stop Analyzing Radio stations");
}

fn run_radio_analyser() {
    for station in RADIOSTATIONS {
        thread::spawn(move || {
            let mut map = SongMap::load_from_file(
                "radio",
                station.shorthand,
            );
            let mut last_songs: LinkedList<Song> = LinkedList::new();
            loop {
                match station.get_current_song() {
                    Some(song) => {
                        if !last_songs.contains(&song) {
                            log_at(
                                &format!("log\\{}", station.shorthand),
                                station.shorthand,
                                &format!("{}: {}", station.name, song),
                            );
                            println!("{}: {}", station.name, song);
                            map.insert_song(song.clone());
                            if last_songs.len() == 5 {
                                last_songs.pop_back();
                            }
                            last_songs.push_front(song);
                        }
                    },
                    None => {},
                }
                map.save_to_file(
                    "radio",
                    station.shorthand,
                );
                thread::sleep(Duration::from_secs(60));
            }
        });
    }
}
