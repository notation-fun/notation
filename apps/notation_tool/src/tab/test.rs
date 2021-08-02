use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_test() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::G, Scale::Major, Signature::_4_4, Tempo::Bpm(60))
        Tracks: [
            {"guitar" Guitar [
                Fretboard
                $duration = _1
                "G" Shape ( 3 2 0 0 0 3 )
                "Em" Shape ( 0 2 2 0 0 0 )
                "C" Shape ( 0 3 2 0 1 0 )
                "D" Shape ( 0 0 0 2 3 2 )
                $duration = T_1_8
                "picks:1" Pick [ 6 3 2 1 2 3 ] |
                "picks:2" Pick [ 6 3 2 1 2 3 ] |
                "picks:3" Pick [ 5 3 2 1 2 3 ] |
                "picks:4" Pick [ 5 3 2 1 2 3 ] |
                "picks:5" Pick [ 4 3 2 1 2 3 ] |
                "picks:6" Pick [ 4 3 2 1 2 3 ] |
            ]}
        ]
        Sections: [
            {"A" Verse [
                {
                    guitar [
                        "G" 1
                        "picks:1" |
                    ]
                }
            ]}
            {"B" Verse [
                {
                    guitar [
                        "G" 1
                        "picks:1" |
                    ]
                }
            ]}
        ]
        Form: "A" "A" "B" "A"
    }
}
