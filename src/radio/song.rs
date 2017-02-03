use util::get_inbetween;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub struct Song {
    pub artist: String,
    pub title: String,
}

impl Song {
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
        // Could be replace with a regex <smth>magazin
        || filter.contains("europamagazin")
        || filter.contains("meinungsmagazin")
        || filter.contains("martin winkelheide")
    {
        return None;
    }
    match html.find(" - ") {
        Some(pos) => {
            let (artist, title) = html.split_at(pos);
            return Some(
                Song {
                    artist: artist.trim().into(),
                    title: title.replace(" - ", "").trim().into()
                }
            );
        },
        None => {},
    }
    /// WDR 2 "von"
    match html.find(" von ") {
        Some(pos) => {
            let (title, artist) = html.split_at(pos);
            Some(
                Song {
                    artist: artist.replace(" von ", "").trim().into(),
                    title: title.trim().into()
                }
            )
        },
        None => None,
    }
}

/// Song is after field "song_now"
pub fn radio_ndr(html: &str) -> Option<Song> {
    let song = match get_inbetween(html, "song_now\":  \"", "\",\n\"song_previous") {
        Some(song) => song,
        None => return None,
    };
    match song.find(" - ") {
        Some(pos) => {
            let (artist, title) = song.split_at(pos);
            Some( Song {
                artist: artist.trim().into(),
                title: title.replace(" - ", "").trim().into()
            })
        },
        None => None,
    }
}

pub fn radio_antenne(html: &str) -> Option<Song> {
    if html.contains("\"total\":1") {
        return None;
    }
    // Response contains two results, songs is saved in type PE_E
    // It is not ensured that the PE_E is the always the first/second part of the response
    let html = match get_inbetween(html, "\"type\":\"PE_E\"", ":true") {
        Some(html) => html,
        None => return None,
    };
    // Artist
    let artist = match get_inbetween(&html, "\"artistName\":\"", "\",\"song\"") {
        Some(artist) => artist,
        None => return None,
    };
    // Song
    let song = match get_inbetween(&html, "\"name\":\"", "\",\"artistName\":\"") {
        Some(song) => song,
        None => return None,
    };
    Some(Song{ artist: artist.into(), title: song.into() })
}
