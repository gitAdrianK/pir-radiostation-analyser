use radio::RadioStation;
use radio::song::{radio_1live, radio_ndr, radio_antenne};

pub const RADIOSTATIONS: &'static [RadioStation] = &[
    // TODO: Add other stations
    // Stations 1LIVE and WDR
    RadioStation {
        name: "1LIVE",
        shorthand: "1live",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_1live.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "1LIVE diGGi",
        shorthand: "diggi",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_1live_diggi.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR2",
        shorthand: "wdr2",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr2.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR3",
        shorthand: "wdr3",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr3.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR4",
        shorthand: "wdr4",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr4.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "WDR5",
        shorthand: "wdr5",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_wdr5.txt",
        get_song: radio_1live,
    },
    RadioStation {
        name: "Funkhaus Europa",
        shorthand: "fhe",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_fhe.txt",
        get_song: radio_1live,
    },
    // RadioStation {
    //     name: "Kiraka",
    //     url: "http://www.wdr.de/radio/radiotext/streamtitle_kiraka.txt",
    //     get_song: radio_1live,
    // },
    RadioStation {
        name: "WDR-Event",
        shorthand: "wdre",
        url: "http://www.wdr.de/radio/radiotext/streamtitle_event.txt",
        get_song: radio_1live,
    },
    // Stations NDR
    RadioStation {
        name: "NDR 1 Niedersachsen",
        shorthand: "ndr1n",
        url: "http://www.ndr.de/public/radio_playlists/ndr1niedersachsen.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR 1 Radio MV",
        shorthand: "ndr1rmv",
        url: "http://www.ndr.de/public/radio_playlists/ndr1radiomv.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Welle Nord",
        shorthand: "ndrwn",
        url: "http://www.ndr.de/public/radio_playlists/ndr1wellenord.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR 90,3",
        shorthand: "ndr903",
        url: "http://www.ndr.de/public/radio_playlists/ndr903.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR 2",
        shorthand: "ndr2",
        url: "http://www.ndr.de/public/radio_playlists/ndr2.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Info",
        shorthand: "ndri",
        url: "http://www.ndr.de/public/radio_playlists/ndrinfo.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Kultur",
        shorthand: "ndrk",
        url: "http://www.ndr.de/public/radio_playlists/ndrkultur.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NJOY",
        shorthand: "njoy",
        url: "http://www.ndr.de/public/radio_playlists/njoy.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Blue",
        shorthand: "ndrb",
        url: "http://www.ndr.de/public/radio_playlists/ndrblue.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "NDR Plus",
        shorthand: "ndrp",
        url: "http://www.ndr.de/public/radio_playlists/ndrplus.json",
        get_song: radio_ndr,
    },
    RadioStation {
        name: "Antenne Niedersachsen On Air",
        shorthand: "antenneoa",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=29&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen 80er",
        shorthand: "antenne80",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=31&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen 90er",
        shorthand: "antenne90",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=422&nameSize=200&
                artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen Charts",
        shorthand: "antennec",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=30&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen Deutsch",
        shorthand: "antenned",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=565&nameSize=200
                &artistNameSize=200&descriptionSize=200&callback=radioplayer
                .playing.receive",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen X-Mas",
        shorthand: "antennexmas",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=110&nameSize=200
        &artistNameSize=200&descriptionSize=200&callback=radioplayer
        .playing.receive&_=1486056701107",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen Feel Good",
        shorthand: "antennefg",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=602&nameSize=200
        &artistNameSize=200&descriptionSize=200&callback=radioplayer
        .playing.receive&_=1486056790331",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Niedersachsen Love",
        shorthand: "antennelove",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=566&nameSize=200
        &artistNameSize=200&descriptionSize=200&callback=radioplayer
        .playing.receive&_=1486056946846",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Oldies",
        shorthand: "antenneold",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=32&nameSize=200
        &artistNameSize=200&descriptionSize=200&callback=radioplayer
        .playing.receive&_=1486057006247",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Relax",
        shorthand: "antennelax",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=454&nameSize=200
        &artistNameSize=200&descriptionSize=200&callback=radioplayer
        .playing.receive&_=1486057066325",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Rock",
        shorthand: "antennerock",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=33&nameSize=200
        &artistNameSize=200&descriptionSize=200&callback=radioplayer
        .playing.receive&_=1486057141342",
        get_song: radio_antenne,
    },
    RadioStation {
        name: "Antenne Schlager",
        shorthand: "antennesch",
        url: "http://np.radioplayer.de/qp/v3/onair?rpIds=601&nameSize=200
        &artistNameSize=200&descriptionSize=200&callback=radioplayer
        .playing.receive&_=1486057202504",
        get_song: radio_antenne,
    },
];
