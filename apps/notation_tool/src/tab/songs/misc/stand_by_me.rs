use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::A, Scale::Major, Signature::_4_4, Tempo::Bpm(120))
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 3 5 )
                "6-" Chord ( 6: 3- 5 )
                "4" Chord ( 4: 3 5 )
                "5" Chord ( 5: 3 5 )
            ]}
            {guitar Guitar [
                Fretboard
                $duration = _1
                "A" Shape ( 0 0 2 2 2 0 )
                "#Fm" Shape ( 2 4 4 2 2 2 )
                "D" Shape ( _ _ 0 2 3 2 )
                "E" Shape ( 0 2 2 1 0 0 )
                $duration = _1_8
                "i:1" Pick [ 3* _ 3* _ 4 3@1 ] |
                "i:2" Pick [ 3* _ 3* _ 3 3@1 ] |
                "i:3" Pick [ 4* _ 4* _ 4@2* ] |
                "i:4" Pick [ 4* _ 4* _ 4 4@2 ] |
                "i:5" Pick [ 4* _ 4* _ 4 4@4 ] |
                "i:6" Pick [ 4* _ 4* _ 4 3 ] |
            ]}
        ]
        Sections: [
            {intro Intro [
                {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:2" | ]
                }
                {
                    chord [ "6-" 1 ]
                    guitar [ "#Fm" 1 ; "i:3" | ]
                }
                {
                    chord [ "6-" 1 ]
                    guitar [ "#Fm" 1 ; "i:4" | ]
                }
                {
                    chord [ "4" 1 ]
                    guitar [ "D" 1 ; "i:5" | ]
                }
                {
                    chord [ "5" 1 ]
                    guitar [ "E" 1 ; "i:6" | ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                }
            ]}
            {verse Verse [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "picks" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "picks" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "picks" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "picks" | ]
                }
            ]}
        ]
        Form: intro
    }
}
