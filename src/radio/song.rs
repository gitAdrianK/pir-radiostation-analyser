// There has to be a way better, more canonical way to do this, but until
// someone points it out, this works.

#[derive(Debug, PartialEq)]
pub struct Song {
    artist: String,
    title: String,
}

impl Song {
    pub fn empty() -> Self {
        Song{ artist: "".into(), title: "".into() }
    }

    pub fn is_empty(&self) -> bool {
        self.artist == "" && self.title == ""
    }
}

impl ::std::fmt::Display for Song {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{} -by- {}", self.title, self.artist)
    }
}

/// Get the song for 1live and wdr stations
/// The song is the only thing in HTML and can be returned without changes
pub fn radio_1live(html: &str) -> Option<Song> {
    // Songs are divided by "-", do not contain "KiRaka" or "WDR"
    let filter = html.to_lowercase();
    if filter.contains("kiraka")
        || filter.contains("wdr")
        || filter.contains("1live")
        || filter.contains("cosmoradio")
        || filter.contains("mittagsecho")
        || filter.contains("wir sind der westen")
        || filter.contains("echo des tages")
        || filter.contains("moderation:")
        || filter.contains("europamagazin")
    {
        return None;
    }
    match html.find("-") {
        Some(pos) => {
            let (artist, title) = html.split_at(pos);
            return Some(
                Song{ artist: artist.trim().into(), title: title.replace("-", "").trim().into() }
            );
        },
        None => {},
    }
    /// WDR 2 "von"
    match html.find("von") {
        Some(pos) => {
            let (title, artist) = html.split_at(pos);
            Some(
                Song{ artist: artist.replace("von", "").trim().into(), title: title.trim().into() }
            )
        },
        None => None,
    }
}

//TODO: Rewrite into fewer repeating code line

/// Song is after field "song_now"
pub fn radio_ndr(html: &str) -> Option<Song> {
    let prefix = "song_now\":  \"";
    let suffix = "\",\n\"song_previous";
    let s_pos = html.find(prefix).unwrap();
    let (_, song) = html.split_at(s_pos + prefix.len());
    let s_pos = song.find(suffix).unwrap();
    let (song , _) = song.split_at(s_pos);
    match song.find("-") {
        Some(pos) => {
            let (artist, title) = song.split_at(pos);
            Some(Song{ artist: artist.trim().into(), title: title.replace("-", "").trim().into() })
        },
        None => None,
    }
}

pub fn radio_antenne(html: &str) -> Option<Song> {
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
    Some(Song{ artist: artist.into(), title: song.into() })
}
