pub trait RadioStation {
    /// If a song different from the previous one is playing return it, otherwise None.
    fn get_new_song() -> Option<String>;
}
