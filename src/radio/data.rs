use radio::RadioStation;
use radio::song_func::radio_1live;

pub const RADIOSTATIONS: &'static [RadioStation] = &[
    // TODO: Add other stations
    RadioStation {
        name: "1LIVE",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_1live.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "1LIVE diGGi",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_1live_diggi.txt",
        get_song: radio_1live,
    },
];
