use notation_dsl::{tab};
use notation_proto::prelude::*;

pub fn new_tab_bai_hua_lin() -> Tab {
    tab! {
        meta: TabMeta::new(Key::E, Scale::Minor, Signature::_3_4, Tempo::Bpm(118))
        lines: [
            {"shapes" [
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
            ]}
            {"picks" [
                $duration = _1_8
                "6-1" Pick [ 6 3 2 1 2 3 ]
                "5-1" Pick [ 5 3 2 1 2 3 ]
                "5-2" Pick [ 5 4 3 2 3 4 ]
                "4-1" Pick [ 4 3 2 1 2 3 ]
            ]}
        ]
        tracks: [
            {"guitar" Guitar [
                Fretboard
            ]}
        ]
        layers: [
            {"Em" [
                "shapes" "Em" 1
                "picks" "6-1" 6
            ] track: "guitar"}
            {"Em_1" [
                "shapes" "Em" 1
                "picks" "5-1" 6
            ] track: "guitar"}
            {"Em_2" [
                "shapes" "Em" 1
                "picks" "4-1" 6
            ] track: "guitar"}
            {"Adim" [
                "shapes" "Adim" 1
                "picks" "5-2" 6
            ] track: "guitar"}
            {"B7" [
                "shapes" "B7" 1
                "picks" "5-1" 6
            ] track: "guitar"}
            {"B7_1" [
                "shapes" "B7_1" 1
                "picks" "5-1" 6
            ] track: "guitar"}
            {"G" [
                "shapes" "G" 1
                "picks" "6-1" 6
            ] track: "guitar"}
            {"E" [
                "shapes" "E" 1
                "picks" "6-1" 6
            ] track: "guitar"}
            {"E_1" [
                "shapes" "E" 1
                "picks" "4-1" 6
            ] track: "guitar"}
            {"Am" [
                "shapes" "Am" 1
                "picks" "5-1" 6
            ] track: "guitar"}
            {"G/B" [
                "shapes" "G/B" 1
                "picks" "5-1" 6
            ] track: "guitar"}
            {"D" [
                "shapes" "D" 1
                "picks" "4-1" 6
            ] track: "guitar"}
        ]
        sections: [
            {"A1" Verse [
                "Em" "Em_1" "Adim" "B7"
                "Adim" "Adim" "B7" "Em"
            ]}
            {"A2" Verse [
                "E" "E_1" "Am" "G"
                "G/B" "Am" "B7_1" "Em"
            ]}
            {"B1" Chorus [
                "Em" "Am" "D" "G"
                "Em" "Am" "Adim" "B7_1"
            ]}
            {"B2" Chorus [
                "Em" "Am" "D" "G" "Em" "Am" "B7_1" "Em"
            ]}
            {"O" Outro [
                "Em"
            ]}
        ]
        form: "A1"
            "A1" "A2" "A1" "A2" "B1" "B2"
            "A1" "A2" "A1" "A2" "B1" "B2"
            "A2" "A1" "A1" "O"
    }
}

