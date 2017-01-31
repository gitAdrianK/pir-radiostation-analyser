// There has to be a way better, more canonical way to do this, but until
// someone points it out, this works.

// TODO: Create functions for other radio stations

/// Get the song for 1live stations
/// The song is the only thing in HTML and can be returned without changes
pub fn radio_1live(html: &str) -> String {
    html.into()
}
