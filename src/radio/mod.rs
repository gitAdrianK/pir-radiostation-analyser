pub mod data;
pub mod song;

use radio::song::Song;

/// A radio must be able to provide the currently played song
pub trait Radio {
    /// Get the currently playing song.
    fn get_current_song(&self) -> Option<Song>;
}

/// Representation of a radiostation
pub struct RadioStation {
    /// Name
    pub name: &'static str,
    /// Shorthand
    pub shorthand: &'static str,
    /// URL to a site containing the currently playing song
    url: &'static str,
    /// Function to get the currently playing song
    get_song: fn(&str) -> Option<Song>,
}

impl Radio for RadioStation {
    fn get_current_song(&self) -> Option<Song> {
        use util::http_scraper::scrape;
        // Get html from url
        let html = match scrape(self.url, self.name) {
            Some(html) => html,
            None => return None,
        };
        // Get song
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
