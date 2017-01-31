// There has to be a way better, more canonial way to do this, but untill
// someone points it out, this works.

// TODO: Create functions for other radiostations

/// Get the song for 1live stations
/// The song is the only thing in html and can be returned without changes
pub fn radio_1live(html: &str) -> String {
    html.into()
}
