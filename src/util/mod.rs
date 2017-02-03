pub mod logger;
pub mod http_scraper;
pub mod song_map;

/// Given a prefix and a suffix returns everything in-between.
/// If prefix or suffix aren't contained None will be returned.
/// Should the prefix appear before the suffix, or the text between those be empty
/// None will be returned.
pub fn get_inbetween(string: &str, prefix: &str, suffix: &str) -> Option<String> {
    let beginn = match string.find(prefix) {
        Some(pos) => pos + prefix.len(),
        None => return None,
    };
    let end = match string.find(suffix) {
        Some(pos) => pos,
        None => return None,
    };
    if beginn >= end {
        return None;
    }
    unsafe {
        Some(string.slice_unchecked(beginn, end).into())
    }
}
