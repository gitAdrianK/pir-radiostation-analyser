use radio::song::Song;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use util::logger::log;

/// A map containing songs and their times played,
/// Inserting a song already known will increase its counter
#[derive(Debug)]
pub struct SongMap(HashMap<Song, u16>);

impl SongMap {
    /// Creates a new empty songmap
    fn new() -> Self {
        SongMap(HashMap::new())
    }

    /// Inserts a song into the map
    fn insert(&mut self, key: Song, value: u16) {
        self.0.insert(key, value);
    }

    /// Inserts a song into the map
    /// If known increases its counter instead
    pub fn insert_song(&mut self, song: Song) {
        let counter = self.0.entry(song).or_insert(0);
        *counter += 1;
    }

    /// Saves a map to json
    pub fn save_to_file(&self, dir: &str, name: &str) {
        /// Logs possible errors
        fn save_error(dir: &str, name: &str) {
            log(&format!("ERROR: Could not save {}\\{}.json",dir, name));
        }

        // Create file and directory
        let _ = fs::create_dir_all(dir);
        let to = match File::create(format!("{}\\{}.json",dir, name)) {
            Ok(file) => file,
            Err(_) => {
                save_error(dir, name);
                return
            }
        };
        let mut to = BufWriter::new(to);
        // Since the map contains a non-string key we cannot save it to json
        // straight away, convert it to a save-able format
        let mut string_vec: Vec<SongMapHelper> = Vec::new();
        for (key, value) in self.0.iter() {
            string_vec.push(
                SongMapHelper {
                    artist: key.artist.clone(),
                    title: key.title.clone(),
                    count: *value,
                }
            );
        }
        // Write json to file
        match serde_json::to_writer_pretty(&mut to, &string_vec) {
            Err(_) => {
                save_error(dir, name);
                return
            }
            _ => {},
        }
        // Add a newline, otherwise the json might be invalid
        let _ = to.write(b"\n");
        let _ = to.flush();
    }

    /// Loads a map from json
    pub fn load_from_file(dir: &str, name: &'static str) -> Self {
        /// Logs possible errors
        fn load_error(dir: &str, name: &'static str) {
            log(&format!("ERROR: Could not load {}\\{}.json",dir, name));
        }

        // Create an empty map
        let mut song_map = SongMap::new();
        // Try to open
        let from = match File::open(format!("{}\\{}.json",dir, name)) {
            Ok(file) => file,
            Err(_) => {
                load_error(dir, name);
                return song_map
            }
        };
        let from = BufReader::new(from);
        // Save into a vector of helper
        let string_vec: Vec<SongMapHelper> = match serde_json::from_reader(from) {
            Ok(vec) => vec,
            Err(_) => {
                load_error(dir, name);
                return song_map
            }
        };
        // Convert helper to the actual songmap
        for song_helper in string_vec {
            song_map.insert( Song {
                artist: song_helper.artist,
                title: song_helper.title
            }, song_helper.count);
        }
        song_map
    }
}

/// This is a helper so that the songmap may be saved to json with serde
#[derive(Debug, Serialize, Deserialize)]
struct SongMapHelper {
    artist: String,
    title: String,
    count: u16,
}
