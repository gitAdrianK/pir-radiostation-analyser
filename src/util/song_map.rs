use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use serde_json;

use radio::song::Song;
use util::logger::log;

#[derive(Debug, Serialize, Deserialize)]
pub struct SongMap(HashMap<Song, u8>);

impl SongMap {
    fn new() -> Self {
        SongMap(HashMap::new())
    }

    fn insert(&mut self, key: Song, value: u8) {
        self.0.insert(key, value);
    }

    pub fn insert_song(&mut self, song: Song) {
        let counter = self.0.entry(song).or_insert(0);
        *counter += 1;
    }

    pub fn save_to_file(&self, dir: &str, name: &str) {
        fn save_error(dir: &str, name: &str) {
            log(&format!("ERROR: Could not save {}\\{}.json",dir, name));
        }

        let _ = fs::create_dir_all(dir);
        let to = match File::create(format!("{}\\{}.json",dir, name)) {
            Ok(file) => file,
            Err(_) => {
                save_error(dir, name);
                return
            }
        };
        let mut to = BufWriter::new(to);
        let mut string_map: HashMap<String, u8> = HashMap::new();
        for (key, value) in self.0.iter() {
            string_map.insert(serde_json::to_string(key).unwrap(), *value);
        }
        // TODO: Hashmap contains non String value, this panics
        // Only HashMap<String, T: Serializable> supported
        match serde_json::to_writer_pretty(&mut to, &string_map) {
            Err(_) => {
                save_error(dir, name);
                return
            }
            _ => {},
        }
        let _ = to.write(b"\n");
        let _ = to.flush();
    }

    pub fn load_from_file(dir: &str, name: &'static str) -> Self {
        fn load_error(dir: &str, name: &'static str) {
            log(&format!("ERROR: Could not load {}\\{}.json",dir, name));
        }

        let mut song_map = SongMap::new();
        let from = match File::open(format!("{}\\{}.json",dir, name)) {
            Ok(file) => file,
            Err(_) => {
                load_error(dir, name);
                return song_map
            }
        };
        let from = BufReader::new(from);
        let string_map: HashMap<String, u8> = match serde_json::from_reader(from) {
            Ok(map) => map,
            Err(_) => {
                load_error(dir, name);
                return song_map
            }
        };
        for (key, value) in string_map.iter() {
            song_map.insert(serde_json::from_str(key).unwrap(), *value);
        }
        song_map
    }
}
