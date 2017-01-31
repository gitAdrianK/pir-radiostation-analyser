pub mod data;
pub mod song_func;

pub trait Radio {
    /// Get the currently playing song.
    fn get_current_song(&self) -> String;
}

pub struct RadioStation {
    name: &'static str,
    url: &'static str,
    get_song: fn(&str) -> String,
}

impl Radio for RadioStation {
    fn get_current_song(&self) -> String {
        use util::http_scraper::scrape;
        match scrape(self.url, self.name) {
            Some(html) => (self.get_song)(&html),
            None => String::new(),
        }
    }
}
