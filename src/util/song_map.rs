use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use serde_json;

use radio::song::Song;
//use util::logger::log;

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
        //fn save_error(dir: &str, name: &str) {
        //    log(&format!("ERROR: Could not save {}\\{}.json",dir, name));
        //}
        let _ = fs::create_dir_all(dir);
        let to = match File::create(format!("{}\\{}.json",dir, name)) {
            Ok(file) => file,
            Err(_) => {
                //save_error(dir, name);
                return
            }
        };
        let mut to = BufWriter::new(to);
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
        match serde_json::to_writer_pretty(&mut to, &string_vec) {
            Err(_) => {
                //save_error(dir, name);
                return
            }
            _ => {},
        }
        let _ = to.write(b"\n");
        let _ = to.flush();
    }

    pub fn load_from_file(dir: &str, name: &'static str) -> Self {
        //fn load_error(dir: &str, name: &'static str) {
        //    log(&format!("ERROR: Could not load {}\\{}.json",dir, name));
        //}

        let mut song_map = SongMap::new();
        let from = match File::open(format!("{}\\{}.json",dir, name)) {
            Ok(file) => file,
            Err(_) => {
                //load_error(dir, name);
                return song_map
            }
        };
        let from = BufReader::new(from);
        let string_vec: Vec<SongMapHelper> = match serde_json::from_reader(from) {
            Ok(vec) => vec,
            Err(_) => {
                //load_error(dir, name);
                return song_map
            }
        };
        for song_helper in string_vec {
            song_map.insert( Song {
                artist: song_helper.artist,
                title: song_helper.title
            }, song_helper.count);
        }
        song_map
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SongMapHelper {
    artist: String,
    title: String,
    count: u8
}
