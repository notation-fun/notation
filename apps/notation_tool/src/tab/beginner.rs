use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_1_right_hand() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::G, Scale::Major, Signature::_4_4, Tempo::Bpm(60))
        Lines: [
            {"shapes" [
                $duration = _1
                Shape [
                    ( 0 2 2 0 0 0 )
                    ( 3 2 0 0 0 0 )
                ]
            ]}
            {"picks" [
                $duration = T_1_8
                Pick [ 6 3 2 1 2 3 ]
                Pick [ 6 3 2 1 2 3 ]
            ]}
        ]
        Tracks: [
            {"guitar" Guitar [
                Fretboard
            ]}
        ]
        Layers: [
            {"Em" [
                "shapes" 1 1
                "picks" 1 12
            ] Track: "guitar"}
            {"G" [
                "shapes" 2 1
                "picks" 1 12
            ] Track: "guitar"}
        ]
        Sections: [
            {"A" Verse [
                "Em" "Em" "G" "G"
            ]}
        ]
        Form: "A" "A"
    }
}
