use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_1_right_hand() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::G, Scale::Major, Signature::_4_4, Tempo::Bpm(60))
        Tracks: [
            {"guitar" Guitar [
                Fretboard
                $duration = _1
                "Em" Shape ( 0 2 2 0 0 0 )
                "G" Shape ( 3 2 0 0 0 0 )
                $duration = T_1_8
                "picks" Pick [ 6 3 2 1 2 3 ]
                Pick [ 6 3 2 1 2 3 ] |
            ]}
        ]
        Sections: [
            {"A" Verse [
                {
                    guitar [
                        "Em" 1
                        "picks" |
                    ]
                }
                {
                    guitar [
                        "Em" 1
                        "picks" |
                    ]
                }
                {
                    guitar [
                        "G" 1
                        "picks" |
                    ]
                }
                {
                    guitar [
                        "G" 1
                        "picks" |
                    ]
                }
            ]}
        ]
        Form: "A" "A"
    }
}
