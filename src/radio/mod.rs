pub mod data;
pub mod song_func;

pub trait Radio {
    /// Get the currently playing song.
    fn get_current_song(&self) -> Option<String>;
}

pub struct RadioStation {
    pub name: &'static str,
    url: &'static str,
    get_song: fn(&str) -> String,
}

impl Radio for RadioStation {
    fn get_current_song(&self) -> Option<String> {
        use util::http_scraper::scrape;
        match scrape(self.url, self.name) {
            Some(html) => {
                let song = (self.get_song)(&html);
                if song.is_empty() {
                    None
                } else {
                    Some(song)
                }
            },
            None => None,
        }
    }
}
