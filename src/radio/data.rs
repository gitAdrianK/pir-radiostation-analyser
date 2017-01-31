use radio::RadioStation;
use radio::song_func::*;

pub const RADIOSTATIONS: &'static [RadioStation] = &[
    // TODO: Add other stations
    // Stations 1LIVE and WDR
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
    RadioStation {
        name: "WDR2",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr2.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR3",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr3.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR4",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr4.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR5",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr5.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "Funkhaus Europa",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_fhe.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "Kiraka",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_kiraka.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR-Event",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_event.txt",
        get_song: radio_1live,
    },
    // Stations NDR
    RadioStation {
        name: "NDR 1 Niedersachsen",
        url: "http://www.ndr.de/public/radio_playlists/ndr1niedersachsen.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR 1 Radio MV",
        url: "http://www.ndr.de/public/radio_playlists/ndr1radiomv.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Welle Nord",
        url: "http://www.ndr.de/public/radio_playlists/ndr1wellenord.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR 90,3",
        url: "http://www.ndr.de/public/radio_playlists/ndr903.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR 2",
        url: "http://www.ndr.de/public/radio_playlists/ndr2.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Info",
        url: "http://www.ndr.de/public/radio_playlists/ndrinfo.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Kultur",
        url: "http://www.ndr.de/public/radio_playlists/ndrkultur.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NJOY",
        url: "http://www.ndr.de/public/radio_playlists/njoy.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Blue",
        url: "http://www.ndr.de/public/radio_playlists/ndrblue.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Plus",
        url: "http://www.ndr.de/public/radio_playlists/ndrplus.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "Antenne Niedersachsen On Air",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=29&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen 80er",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=31&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen 90er",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=422&nameSize=200&
                artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen Charts",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=30&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen Deutsch",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=565&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
];
