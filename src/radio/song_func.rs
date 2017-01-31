// There has to be a way better, more canonical way to do this, but until
// someone points it out, this works.

// TODO: Create functions for other radio stations

/// Get the song for 1live and wdr stations
/// The song is the only thing in HTML and can be returned without changes
pub fn radio_1live(html: &str) -> Option<String> {
    // Songs are divided by "-", do not contain "KiRaka" or "WDR"
    let filter = html.to_lowercase();
    if filter.contains("kiraka")
        || filter.contains("wdr")
        || filter.contains("1live")
        || filter.contains("cosmoradio")
        || filter.contains("mittagsecho")
        || filter.contains("wir sind der westen")
        || filter.contains("echo des tages")
    {
        return None;
    }
    Some(html.into())
}

/// Song is after field "song_now"
pub fn radio_ndr(html: &str) -> Option<String> {
    let prefix = "song_now\":  \"";
    let suffix = "\",\n\"song_previous";
    let s_pos = html.find(prefix).unwrap();
    let (_, song) = html.split_at(s_pos + prefix.len());
    let s_pos = song.find(suffix).unwrap();
    let (song , _) = song.split_at(s_pos);
    Some(song.trim().into())
}

pub fn radio_antenne(html: &str) -> Option<String> {
    if html.contains("\"total\":1") {
        return None;
    }
    // General suffix for all fields
    let suffix = "\",";
    // Response contains two results, songs is saved in type PE_E
    // It is not ensured that the PE_E is the always the first/second part of the response
    let type_ident = "\"type\":\"PE_E\"";
    let (_, html) = html.split_at(html.find(type_ident).unwrap() + type_ident.len());
    // Artist
    let artist_prefix = "\"artistName\":\"";
    let a_pos = html.find(artist_prefix).unwrap();
    let (_, artist) = html.split_at(a_pos + artist_prefix.len());
    let a_pos = artist.find(suffix).unwrap();
    let (artist , _) = artist.split_at(a_pos);
    // Song
    let song_prefix = "\"name\":\"";
    let s_pos = html.find(song_prefix).unwrap();
    let (_, song) = html.split_at(s_pos + song_prefix.len());
    let s_pos = song.find(suffix).unwrap();
    let (song , _) = song.split_at(s_pos);
    Some(format!("{} - {}", artist, song))
}
