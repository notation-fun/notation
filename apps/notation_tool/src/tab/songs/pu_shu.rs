use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_bai_hua_lin() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::E, Scale::Minor, Signature::_3_4, Tempo::Bpm(118))
        Tracks: [
            {"guitar" Guitar [
                Fretboard
                $duration = _1
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
                    guitar [
                        "Em" 1
                        "6-1" 6
                    ]
                }
                {
                    guitar [
                        "Em" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "Adim" 1
                        "5-2" 6
                    ]
                }
                {
                    guitar [
                        "B7" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "Adim" 1
                        "5-2" 6
                    ]
                }
                {
                    guitar [
                        "Adim" 1
                        "5-2" 6
                    ]
                }
                {
                    guitar [
                        "B7" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "Em" 1
                        "6-1" 6
                    ]
                }
            ]}
            {"A2" Verse [
                {
                    guitar [
                        "E" 1
                        "6-1" 6
                    ]
                }
                {
                    guitar [
                        "E" 1
                        "4-1" 6
                    ]
                }
                {
                    guitar [
                        "Am" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "G" 1
                        "6-1" 6
                    ]
                }
                {
                    guitar [
                        "G/B" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "Am" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "B7" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "Em" 1
                        "6-1" 6
                    ]
                }
            ]}
            {"B" Chorus [
                {
                    guitar [
                        "Em" 1
                        "6-1" 6
                    ]
                }
                {
                    guitar [
                        "Am" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "D" 1
                        "4-1" 6
                    ]
                }
                {
                    guitar [
                        "G" 1
                        "6-1" 6
                    ]
                }
                {
                    guitar [
                        "Em" 1
                        "6-1" 6
                    ]
                }
                {
                    guitar [
                        "Am" 1
                        "5-1" 6
                    ]
                }
                {
                    guitar [
                        "Adim" 1 @ 1 3
                        "5-2" 6 @ 1 3
                        "B7" 1 @ 2 4
                        "5-2" 6 @ 2 4
                    ]
                }
                {
                    guitar [
                        "B7" 1 @ 1 3
                        "5-1" 6 @ 1 3
                        "Em" 1 @ 2 4
                        "6-1" 6 @ 2 4
                    ]
                }
            ]}
            {"O" Outro [
                {
                    guitar [
                        "Em" 1
                        "6-1" 6
                    ]
                }
            ]}
        ]
        Form: "A1"
            "A1" "A2" "A1" "A2" "B" "B"
            "A1" "A2" "A1" "A2" "B" "B"
            "A2" "A1" "A1" "O"
    }
}
