#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! notation_tab = { version = "0.3.0", path = "crates/notation_tab" }
//! ```

use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "06dd7278-cdaf-40dd-abc6-6e66ec2d6b8c"
        Meta: E Dorian 3 _4 110
        Tracks: [
            {chord Chord [
                $duration = D_1_2
                "2sus4_7" Chord ( 2: 4 5 7- )
                "2sus4_7/4" Chord ( 2: 4 5 7- /4 )
                "1/4" Chord ( 1: 3 5 /4 )
                "1" Chord ( 1: 3 5 )
                "2sus2" Chord ( 2: 2 5 )
                "2-" Chord ( 2: 3- 5 )
                "4" Chord ( 4: 3 5 )
                "v:12"
                    $duration = _1_4
                    Chord ( 4: 3 5 )
                    $duration = _1_2
                    Chord ( 5: 3 5 ) |
                "v:19"
                    $duration = _1_4
                    Chord ( 4: 3 5 )
                    Chord ( 1: 3 5 /3)
                    Chord ( 2: 3- 5 ) |
            ]}
            {guitar Guitar [
                Fretboard capo: 7
                $duration = D_1_2
                "Asus4" Shape ( 0 0 4 0 3 0 )
                "Asus4/D" Shape ( 0 5 4 0 3 0 )
                "G/C" Shape ( _ 3 0 0 0 _ )
                "Asus2" Shape ( _ 0 2 2 0 0 )
                "G'" Shape ( 3 _ 0 0 0 _ )
                "G" Shape ( 3 2 0 0 0 3 )
                "C" Shape ( 0 3 2 0 1 0 )
                "Am" Shape ( 0 0 2 2 1 0 )
                "C_D"
                    $duration = _1_4
                    "C" Shape ( 0 3 2 0 1 0 )
                    $duration = _1_2
                    "D" Shape ( 0 0 0 2 3 2 ) |
                "C_G/B_Am"
                    $duration = _1_4
                    "C" Shape ( 0 3 2 0 1 0 )
                    "G/B" Shape ( _ 2 0 0 3 0 )
                    "Am" Shape ( 0 0 2 2 1 0 ) |
                $duration = _1_8
                "i" Pick [ 5 1 3 4 2 3 ] |
                "i'" Pick [ 5 1 3 4 1 3 ] |
                "i:3" Pick [ 5 2 3* 5@2* ] |
                "v:7" Pick [ (6 2) 3 4 (4@2 2@1) (4 2) 3 ] |
                "v:12" Pick [ (5 2 1) 3 (4 2 1) 3 (4 2) 3 ] |
                "v:17" Pick [ 5 1 3 4 5 5@2 ] |
                "v:19" Pick [ (5 1) 3 (5 2) 3 (5 2) 3 ] |
                "v:20" Pick [ (6 2) 3 4 3 (4@2 2@1) (4 2) ] |
                "v:21" Pick [ (4@2 2@1 *) (4 2 *) 6* ] |
                "v:23" Pick [ (6 2) 3 4 3 (4@2 2@1 *) ] |
                "v:24" Pick [ (4 2 *) (4 2 -) (4@2 2@1 -) (4 2 -) (6 3) 4 ] |
                $duration = D_1_2
                "o:1" Pick [ 5 ] |
                "o:2" Pick [ 1@12 ] |
            ]}
            {lyrics Lyrics [
                $duration = _1_4
                "v1:1" Word [ "are"* "you" ] |
                "v1:2" Word [ "go-", "ing"* "to", ] |
                "v1:3" Word [ "scar-"+ "bo-", "rough" ] |
                "v1:4" Word [ "fare"*+ ] |
                "v:6" Word [ _ "pars-" "ley" ] |
                "v:7" Word [ "sage"* "rose-" ] |
                "v:8" Word [ "ma-" "ry" "and" ] |
                "v:9" Word [ "thyme"*+ ] |
                "v1:12" Word [ _* "re-" ] |
                "v1:13" Word [ "mem-"* "ber" ] |
                "v1:14" Word [ "me"* "to" ] |
                "v1:15" Word [ "one" "who" "lives" ] |
                "v1:16" Word [ "there"*+ ] |
                "v1:18" Word [ "she"* "once" ] |
                "v1:19" Word [ "was"* "a" ] |
                "v1:20" Word [ "true" "love" "of" ] |
                "v1:21" Word [ "mine"*+ ] |
                "v2:1" Word [ "tell", "her"+ "to" ] |
                "v2:2" Word [ "make" "me" "a" ] |
                "v2:3" Word [ "ca-" "m-" "bric" ] |
                "v2:4" Word [ "shirt"*+ ] |
                "v2:12" Word [ _* "with-" ] |
                "v2:13" Word [ "out"* "no" ] |
                "v2:14" Word [ "seams"* "nor" ] |
                "v2:15" Word [ "needle"*+ ] |
                "v2:16" Word [ "work"*+ ] |
                "v2:18" Word [ "then"* "she'll" ] |
                "v2:19" Word [ "be"* "a" ] |
                "v2:20" Word [ "true" "love" "of" ] |
                "v2:21" Word [ "mine"*+ ] |
                "v3:1" Word [ "tell", "her"+ "to" ] |
                "v3:2" Word [ "find" "me" "an" ] |
                "v3:3" Word [ "acre"* "of" ] |
                "v3:4" Word [ "land"*+ ] |
                "v3:12" Word [ _* "bet-" ] |
                "v3:13" Word [ "ween" "the" "salt" ] |
                "v3:14" Word [ "wa-"* "ter" ] |
                "v3:15" Word [ "and" "the" "sea" ] |
                "v3:16" Word [ "strand"*+ ] |
                "v4:1" Word [ "tell", "her"+ "to" ] |
                "v4:2" Word [ "reap", "it"+ "in", "a", ] |
                "v4:3" Word [ "si-", "ckle"+ "of" ] |
                "v4:4" Word [ "lea-", "ther", @ ""* ] |
                "v4:12" Word [ _* "and" ] |
                "v4:13" Word [ "ga-" "ther" "it" ] |
                "v4:14" Word [ "all"* "in", "a", ] |
                "v4:15" Word [ "bunch"* "of" ] |
                "v4:16" Word [ "hea-", "ther", @ ""* ] |
            ]}
            {vocal Vocal [
                $duration = _1_4
                "v:1" Tone [ 2* 2 ] |
                "v:2" Tone [ 6, 6* 6, ] |
                "v:3" Tone [ 3+ 4, 3 ] |
                "v:4" Tone [ 2*+ ] |
                "v:6" Tone [ _ 6 ^1 ] |
                "v:7" Tone [ ^2* ^1 ] |
                "v:8" Tone [ 6 7 5 ] |
                "v:9" Tone [ 6*+ ] |
                "v:12" Tone [ _* ^2 ] |
                "v:13" Tone [ ^2* ^2 ] |
                "v:14" Tone [ ^1* 6 ] |
                "v:15" Tone [ 6 5 4 ] |
                "v:16" Tone [ 3, 1, @ 1*  ] |
                "v:18" Tone [ 2* 6 ] |
                "v:19" Tone [ 5* 4 ] |
                "v:20" Tone [ 3 2 1 ] |
                "v:21" Tone [ 2*+ ] |
                "v2:1" Tone [ 2, 2+ 2 ] |
                "v2:2" Tone [ 6 6 6 ] |
                "v2:3" Tone [ 3 4 3 ] |
                "v3:2" Tone [ 6 5 4 ] |
                "v3:13" Tone [ ^2 ^2 ^2 ] |
                "v4:2" Tone [ 6, 6+ 6, 6, ] |
                "v4:3" Tone [ 3, 4+ 3 ] |
                "v4:4" Tone [ 2, 2, @ 2* ] |
                "v4:12" Tone [ _* ^2, ^2, ] |
                "v4:14" Tone [ ^1* 5, 5, ] |
                "v4:15" Tone [ 4 3 2 ] |
                "v4:16" Tone [ 3, 1, @ 1* ] |
            ]}
        ]
        Sections: [
            {intro Intro [
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                } {
                    chord [ "2sus4_7/4" 1 ]
                    guitar [ "Asus4/D" 1 ; "i" | ]
                } {
                    chord [ "1/4" 1 ]
                    guitar [ "G/C" 1 ; "i:3" | ]
                }
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                }
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                }
            ]}
            {verse Verse [
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                    lyrics [ "v1:1" | @ 1 5 ; "v2:1" | @ 2 ; "v3:1" | @ 3 ; "v4:1" | @ 4 ; "v5:1" | @ 5 ]
                    vocal [ "v:1" | @ 1 5 ; "v2:1" | @ 2 3 4 ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                    lyrics [ "v1:2" | @ 1 5 ; "v2:2" | @ 2 ; "v3:2" | @ 3 ; "v4:2" | @ 4 ; "v5:2" | @ 5 ]
                    vocal [ "v:2" | @ 1 5 ; "v2:2" | @ 2 ; "v3:2" | @ 3 ; "v4:2" | @ 4 ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "G'" 1 ; "v:7" | ]
                    lyrics [ "v1:3" | @ 1 5 ; "v2:3" | @ 2 ; "v3:3" | @ 3 ; "v4:3" | @ 4 ; "v5:3" | @ 5 ]
                    vocal [ "v:3" | @ 1 5 ; "v2:3" | @ 2 3 ; "v4:3" | @ 4 ]
                }
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                    lyrics [ "v1:4" | @ 1 5 ; "v2:4" | @ 2 ; "v3:4" | @ 3 ; "v4:4" | @ 4 ; "v5:4" | @ 5 ]
                    vocal [ "v:4" | @ 1 2 3 5 ; "v4:4" | @ 4 ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                }
                {
                    chord [ "4" 1 ]
                    guitar [ "C" 1 ; "i" | ]
                    lyrics [ "v:6" | ]
                    vocal [ "v:6" | ]
                }
                {
                    chord [ "2-" 1 ]
                    guitar [ "Am" 1 ; "i" | ]
                    lyrics [ "v:7" | ]
                    vocal [ "v:7" | ]
                }
                {
                    chord [ "v:12" | ]
                    guitar [ "C_D" | ; "v:12" | ]
                    lyrics [ "v:8" | ]
                    vocal [ "v:8" | ]
                }
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                    lyrics [ "v:9" | ]
                    vocal [ "v:9" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                }
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                    lyrics [ "v1:12" | @ 1 5 ; "v2:12" | @ 2 ; "v3:12" | @ 3 ; "v4:12" | @ 4 ]
                    vocal [ "v:12" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "v:17" | ]
                    lyrics [ "v1:13" | @ 1 5 ; "v2:13" | @ 2 ; "v3:13" | @ 3 ; "v4:13" | @ 4 ]
                    vocal [ "v:13" | @ 1 2 5 ; "v3:13" | @ 3 4 ]
                }
                {
                    chord [ "4" 1 ]
                    guitar [ "C" 1 ; "i" | ]
                    lyrics [ "v1:14" | @ 1 5 ; "v2:14" | @ 2 ; "v3:14" | @ 3 ; "v4:14" | @ 4 ]
                    vocal [ "v:14" | @ 1 2 3 5 ; "v4:14" | @ 4 ]
                }
                {
                    chord [ "v:19" | ]
                    guitar [ "C_G/B_Am" | ; "v:19" | ]
                    lyrics [ "v1:15" | @ 1 5 ; "v2:15" | @ 2 ; "v3:15" | @ 3 ; "v4:15" | @ 4 ]
                    vocal [ "v:15" | @ 1 2 3 5 ; "v4:15" | @ 4 ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "v:20" | ]
                    lyrics [ "v1:16" | @ 1 5 ; "v2:16" | @ 2 ; "v3:16" | @ 3 ; "v4:16" | @ 4 ]
                    vocal [ "v:16" | @ 1 2 3 5 ; "v4:16" | @ 4 ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "v:21" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i'" | ]
                    lyrics [ "v1:18" | @ 1 5 ; "v2:18" | @ 2 3 4 ]
                    vocal [ "v:18" | ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "v:23" | ]
                    lyrics [ "v1:19" | @ 1 5 ; "v2:19" | @ 2 3 4 ]
                    vocal [ "v:19" | ]
                }
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "v:24" | ]
                    lyrics [ "v1:20" | @ 1 5 ; "v2:20" | @ 2 3 4 ]
                    vocal [ "v:20" | ]
                }
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                    lyrics [ "v1:21" | @ 1 5 ; "v2:21" | @ 2 3 4 ]
                    vocal [ "v:21" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                }
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                }
                {
                    chord [ "2sus2" 1 ]
                    guitar [ "Asus2" 1 ; "i" | ]
                }
            ]}
            {outro Outro [
                {
                    chord [ "2sus4_7" 1 ]
                    guitar [ "Asus4" 1 ; "i" | ]
                } {
                    chord [ "2sus4_7/4" 1 ]
                    guitar [ "Asus4/D" 1 ; "i" | ]
                } {
                    chord [ "1/4" 1 ]
                    guitar [ "G/C" 1 ; "i:3" | ]
                }
                {
                    chord [ "2-" 1 ]
                    guitar [ "o:1" | ]
                }
                {
                    chord [ "2-" 1 ]
                    guitar [ "o:2" | ]
                }
            ]}
        ]
        Form: intro verse verse verse verse verse outro
    }
}
