extern crate hyper;
extern crate time;

mod radio;
mod util;

use std::fs;
use std::thread;
use std::time::Duration;

use radio::Radio;
use radio::data::RADIOSTATIONS;
use util::logger::log;

fn main() {
    // Create log directory in case it doesn't exist
    let _ = fs::create_dir("log");
    log("Start Analyzing Radio stations");
    for station in RADIOSTATIONS {
        thread::spawn(move || {
            let mut last_song = String::new();
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

pub fn radio_antenne(html: &str) -> Option<String> {
    if html.contains("\"total\":1") {
        return None;
    }
    // General suffix for all fields
    let suffix = "\",";
    // Response contains two results, songs is saved in type PE_E
    // It is not ensured that the PE_E is the always the first/second part of the response
    let type_ident = "\"type\":\"PE_E\"";
    let (_, html) = html.split_at(html.find(type_ident).unwrap() + type_ident.len());
    // Artist
    let artist_prefix = "\"artistName\":\"";
    let a_pos = html.find(artist_prefix).unwrap();
    let (_, artist) = html.split_at(a_pos + artist_prefix.len());
    let a_pos = artist.find(suffix).unwrap();
    let (artist , _) = artist.split_at(a_pos);
    // Song
    let song_prefix = "\"name\":\"";
    let s_pos = html.find(song_prefix).unwrap();
    let (_, song) = html.split_at(s_pos + song_prefix.len());
    let s_pos = song.find(suffix).unwrap();
    let (song , _) = song.split_at(s_pos);
    Some(format!("{} - {}", artist, song))
}
