use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab() -> Tab {
    tab! {
        "15ab6b58-1eaf-4075-95ab-f0b35117eddb"
        Meta: TabMeta::new(Key::E, Scale::Minor, Signature::_3_4, Tempo::Bpm(118))
        Tracks: [
            {chord Chord [
                $duration = D_1_2
                "1" Chord ( 1: 3 5 )
                "1/3" Chord ( 1: 3 5 /3 )
                "2-" Chord ( 2: 3- 5 )
                "2o" Chord ( 2: 3- 5% )
                "3" Chord ( 3: 3 5 7- )
                "5" Chord ( 5: 3 5 )
                "6-" Chord ( 6: 3- 5 )
                "6" Chord ( 6: 3 5 )
                $duration = _1_2
                "2o, 3" Chord [ ( 2: 3- 5- ) ( 3: 3 5 7- ) ]
                "3, 6-" Chord [ ( 3: 3 5 7- ) ( 6: 3- 5 ) ]
            ]}
            {guitar Guitar [
                Fretboard
                $duration = D_1_2
                "Em" Shape ( 0 2 2 0 0 0 )
                "Adim" Shape ( 0 0 1 2 1 0 )
                "B7" Shape ( 0 2 4 2 4 2 )
                "B7_1" Shape ( 0 2 1 2 0 2 )
                "G" Shape ( 3 2 0 0 0 3 )
                "E" Shape ( 0 2 2 1 0 0 )
                "Am" Shape ( 0 0 2 2 1 0 )
                "G/B" Shape ( _ 2 0 0 0 3 )
                "D" Shape ( 0 0 0 2 3 2 )
                $duration = _1_8
                "6-1" Pick [ 6 3 2 1 2 3 ]
                "5-1" Pick [ 5 3 2 1 2 3 ]
                "5-2" Pick [ 5 4 3 2 3 4 ]
                "4-1" Pick [ 4 3 2 1 2 3 ]
            ]}
        ]
        Sections: [
            {"A1" Verse [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "6-1" 6 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "5-1" 6 ]
                } {
                    chord [ "2o" 1 ]
                    guitar [ "Adim" 1 ; "5-2" 6 ]
                } {
                    chord [ "3" 1 ]
                    guitar [ "B7" 1 ; "5-1" 6 ]
                } {
                    chord [ "2o" 1 ]
                    guitar [ "Adim" 1 ; "5-2" 6 ]
                } {
                    chord [ "2o" 1 ]
                    guitar [ "Adim" 1 ; "5-2" 6 ]
                } {
                    chord [ "3" 1 ]
                    guitar [ "B7" 1 ; "5-1" 6 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "6-1" 6 ]
                }
            ]}
            {"A2" Verse [
                {
                    chord [ "6" 1 ]
                    guitar [ "E" 1 ; "6-1" 6 ]
                } {
                    chord [ "6" 1 ]
                    guitar [ "E" 1 ; "4-1" 6 ]
                } {
                    chord [ "2-" 1 ]
                    guitar [ "Am" 1 ; "5-1" 6 ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "6-1" 6 ]
                } {
                    chord [ "1/3" 1 ]
                    guitar [ "G/B" 1 ; "5-1" 6 ]
                } {
                    chord [ "2-" 1 ]
                    guitar [ "Am" 1 ; "5-1" 6 ]
                } {
                    chord [ "3" 1 ]
                    guitar [ "B7" 1 ; "5-1" 6 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "6-1" 6 ]
                }
            ]}
            {"B" Chorus [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "6-1" 6 ]
                } {
                    chord [ "2-" 1 ]
                    guitar [ "Am" 1 ; "5-1" 6 ]
                } {
                    chord [ "5" 1 ]
                    guitar [ "D" 1 ; "4-1" 6 ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "6-1" 6 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "6-1" 6 ]
                } {
                    chord [ "2-" 1 ]
                    guitar [ "Am" 1 ; "5-1" 6 ]
                } {
                    chord [ "2o, 3" 2 ]
                    guitar [
                        "Adim" 1 @ 1 3 ; "5-2" 6 @ 1 3
                        "B7" 1 @ 2 4 ; "5-2" 6 @ 2 4
                    ]
                } {
                    chord [ "3, 6-" 2 ]
                    guitar [
                        "B7" 1 @ 1 3 ; "5-1" 6 @ 1 3
                        "Em" 1 @ 2 4 ; "6-1" 6 @ 2 4
                    ]
                }
            ]}
            {"O" Outro [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Em" 1 ; "6-1" 6 ]
                }
            ]}
        ]
        Form: "A1"
            "A1" "A2" "A1" "A2" "B" "B"
            "A1" "A2" "A1" "A2" "B" "B"
            "A2" "A1" "A1" "O"
    }
}
