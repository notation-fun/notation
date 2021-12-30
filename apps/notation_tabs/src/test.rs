#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! notation_tab = "0.3.3"
//! ```

use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "c430733f-46c3-4db2-9685-a72c05027e62"
        Meta: G Major 4 _4 60
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 3 5 )
                "6-" Chord ( 6: 3- 5 )
            ]}
            {guitar Guitar [
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
        Form: "A" "A"
    }
}
