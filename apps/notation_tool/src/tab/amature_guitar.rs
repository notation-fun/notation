use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_1_right_hand() -> Tab {
    /*
    let meta = TabMeta::new(Key::G, Scale::Major, Signature::_4_4, Tempo::Bpm(60));
    let lines = vec![
        line! {
            "shapes" [
                !duration = _1
                Shape 0 2 2 0 0 0
                Shape 3 2 0 0 0 0
            ]
        },
        line! {
            "picks" [
                !duration = T_1_8
                Pick 6
                Pick 3
                Pick 2
                Pick 1
                Pick 2
                Pick 3
                Pick 6
                Pick 3
                Pick 2
                Pick 1
                Pick 2
                Pick 3
            ]
        },
    ];
    let tracks = vec![track! {
        "guitar" Guitar [
            Fretboard
        ]
    }];
    let layers = vec![
        layer! {
            "Em" [
                "shapes" 1 1
                "picks" 1 12
            ] track: "guitar"
        },
        layer! {
            "G" [
                "shapes" 2 1
                "picks" 1 12
            ] track: "guitar"
        },
    ];
    let sections = vec![section! {
        "A" Verse [
            [ "Em" ]
            [ "G" ]
        ]
    }];
    let form = form! {
        "A" "A"
    };
    Tab::new(meta, lines, tracks, layers, sections, form)
     */
    tab! {
        meta: TabMeta::new(Key::G, Scale::Major, Signature::_4_4, Tempo::Bpm(60))
        lines: [
            {"shapes" [
                !duration = _1
                Shape 0 2 2 0 0 0
                Shape 3 2 0 0 0 0
            ]}
            {"picks" [
                !duration = T_1_8
                Pick 6
                Pick 3
                Pick 2
                Pick 1
                Pick 2
                Pick 3
                Pick 6
                Pick 3
                Pick 2
                Pick 1
                Pick 2
                Pick 3
            ]}
        ]
        tracks: [
            {"guitar" Guitar [
                Fretboard
            ]}
        ]
        layers: [
            {"Em" [
                "shapes" 1 1
                "picks" 1 12
            ] track: "guitar"}
            {"G" [
                "shapes" 2 1
                "picks" 1 12
            ] track: "guitar"}
        ]
        sections: [
            {"A" Verse [
                [ "Em" ]
                [ "G" ]
            ]}
        ]
        form: "A" "A"
    }
}
