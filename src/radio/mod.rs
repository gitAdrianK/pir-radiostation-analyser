pub mod data;
pub mod song;

use radio::song::Song;

pub trait Radio {
    /// Get the currently playing song.
    fn get_current_song(&self) -> Option<Song>;
}

pub struct RadioStation {
    pub name: &'static str,
    url: &'static str,
    get_song: fn(&str) -> Option<Song>,
}

impl Radio for RadioStation {
    fn get_current_song(&self) -> Option<Song> {
        use util::http_scraper::scrape;
        let html = match scrape(self.url, self.name) {
            Some(html) => html,
            None => return None,
        };
        let song = match (self.get_song)(&html) {
            Some(song) => {
                if song.is_empty() {
                    return None;
                }
                song
            },
            None => return None,
        };
        Some(song)
    }
}
