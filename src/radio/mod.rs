pub mod data;
pub mod song_func;

pub trait Radio {
    /// Get the currently playing song.
    fn get_current_song(&self) -> Option<String>;
}

pub struct RadioStation {
    pub name: &'static str,
    url: &'static str,
    get_song: fn(&str) -> Option<String>,
}

impl Radio for RadioStation {
    fn get_current_song(&self) -> Option<String> {
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
