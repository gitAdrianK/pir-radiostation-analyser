// There has to be a way better, more canonical way to do this, but until
// someone points it out, this works.

// TODO: Create functions for other radio stations

/// Get the song for 1live and wdr stations
/// The song is the only thing in HTML and can be returned without changes
pub fn radio_1live(html: &str) -> String {
    // Songs are divided by "-", do not contain "KiRaka" or "WDR"
    let filter = html.to_lowercase();
    if filter.contains("kiraka")
        || filter.contains("wdr")
        || filter.contains("1live")
        || filter.contains("cosmoradio")
        || filter.contains("mittagsecho")
        || filter.contains("wir sind der westen")
    {
        return String::new();
    }
    html.into()
}

/// Song is after field "song_now"
pub fn radio_ndr(html: &str) -> String {
    let prefix = "song_now\":  \"";
    let suffix = "\",\n\"song_previous";
    let s_pos = html.find(prefix).unwrap();
    let (_, song) = html.split_at(s_pos + prefix.len());
    let s_pos = song.find(suffix).unwrap();
    let (song , _) = song.split_at(s_pos);
    song.trim().into()
}
