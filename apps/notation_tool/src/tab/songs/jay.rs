use notation_dsl::{tab};
use notation_proto::prelude::*;

pub fn new_tab_long_juan_feng() -> Tab {
    tab! {
        meta: TabMeta::new(Key::A, Scale::Major, Signature::_4_4, Tempo::Bpm(72))
        lines: [
            {"shapes" [
                $duration = _1
                "G" Shape ( 3 2 0 0 0 3 )
                "Em" Shape ( 0 2 0 0 0 0 )
                "C" Shape ( 0 3 0 0 0 0 )
                "Am" Shape ( 0 0 2 0 1 0 )
                "D" Shape ( 0 0 0 2 3 0 )
            ]}
            {"picks" [
                $duration = _1_8
                "intro-6" Pick [ (6 3) 4 2 3 2@1 ]
                $duration = _1_16
                Pick [ 2 3 3@2 2 ]
                $duration = _1_8
                Pick [ 3@2 ]
                "intro-5" Pick [ (5 3) 4 2 3 2@1 ]
                $duration = _1_16
                Pick [ 2 3 3@2 2 ]
                $duration = _1_8
                Pick [ 3@2 ]
                "verse-5" Pick [ 5 4 3 2 3 4 ]
                "verse-5" Pick [ 4 3 2 1 2 3 ]
            ]}
        ]
        tracks: [
            {"guitar" Guitar [
                Fretboard capo: 2
            ]}
        ]
        layers: [
            {"G-Intro" [
                "shapes" "G" 1
                "picks" "intro-6" 12
            ] track: "guitar"}
            {"Em-Intro" [
                "shapes" "Em" 1
                "picks" "intro-6" 12
            ] track: "guitar"}
            {"C-Intro" [
                "shapes" "C" 1
                "picks" "intro-5" 12
            ] track: "guitar"}
        ]
        sections: [
            {"Intro" Intro [
                "G-Intro" "Em-Intro" "C-Intro" "G-Intro"
            ]}
        ]
        form: "Intro"
    }
}

