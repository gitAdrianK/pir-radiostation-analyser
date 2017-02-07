extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;

mod radio;
mod util;


use radio::data::RADIOSTATIONS;
use radio::Radio;
use radio::song::Song;
use std::collections::LinkedList;
use std::thread;
use std::time::Duration;
use util::html_generator::write_html_from_json;
use util::logger::{log, log_at};
use util::song_map::SongMap;

const DIRECTORY: &'static str = "data";
const SUBDIRECTORY: &'static str = "radio";

/// Radiostation analyzer
/// Collects songs played on radiostations and displays them sorted by
/// times played
fn main() {
    // Start analyzing
    run_radio_analyser();
    // Write html displaying info
    write_html_from_json(DIRECTORY, SUBDIRECTORY);
}

/// Analyzes stations
/// Gets the song played about every minute and saves them to json
fn run_radio_analyser() {
    log("Start Analyzing Radio stations");
    for station in RADIOSTATIONS {
        thread::spawn(move || {
            // Try to load existing file
            let mut map = SongMap::load_from_file(
                &format!("{}\\{}", DIRECTORY, SUBDIRECTORY),
                station.shorthand,
            );
            // Save the last 5 songs played
            let mut last_songs: LinkedList<Song> = LinkedList::new();
            loop {
                // If theres a song playing and it's not contained in the
                // last 5 songs log it and add it to the list and save change
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
                            map.save_to_file(
                                &format!("{}\\{}", DIRECTORY, SUBDIRECTORY),
                                station.shorthand,
                            );
                        }
                    },
                    None => {},
                }
                // Wait about a minute
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
