use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_test() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::G, Scale::Major, Signature::_4_4, Tempo::Bpm(60))
        Lines: [
            {"shapes" [
                $duration = _1
                Shape [
                    ( 3 2 0 0 0 3 )
                    ( 0 2 2 0 0 0 )
                    ( 0 3 2 0 1 0 )
                    ( 0 0 0 2 3 2 )
                ]
            ]}
            {"picks" [
                $duration = T_1_8
                Pick [ 6 3 2 1 2 3 ]
                Pick [ 6 3 2 1 2 3 ]
                Pick [ 5 3 2 1 2 3 ]
                Pick [ 5 3 2 1 2 3 ]
                Pick [ 4 3 2 1 2 3 ]
                Pick [ 4 3 2 1 2 3 ]
            ]}
        ]
        Tracks: [
            {"guitar" Guitar [
                Fretboard
            ]}
        ]
        Layers: [
            {"G" [
                "shapes" 1 1
                "picks" 1 12
            ] Track: "guitar"}
            {"Em" [
                "shapes" 2 1
                "picks" 1 12
            ] Track: "guitar"}
            {"C" [
                "shapes" 3 1
                "picks" 13 12
            ] Track: "guitar"}
            {"D" [
                "shapes" 4 1
                "picks" 25 12
            ] Track: "guitar"}
        ]
        Sections: [
            {"A" Verse [
                ( "G" )
                ( "Em" )
                ( "C" )
                ( "D" )
            ]}
            {"B" Verse [
                ( "C" )
                ( "G" )
                ( "C" )
                ( "Em" )
            ]}
        ]
        Form: "A" "A" "B" "A"
    }
}
