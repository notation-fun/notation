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
            {lyrics Lyrics [
                $duration = _1_8
                "i:8" Word [ _** _ "when" "the" "night" @ ] |
                "v1:1" Word [ @ "" _** _ "has" "come" @ ] |
                "v1:2" Word [ @ "" ] |
                "v1:3" Word [ _*+ "and", "the", "land"+ "is"+ "dark" @ ] |
                "v1:4" Word [ @ ""* _*+ "and" "the" "moon" ] |
                "v1:5" Word [ ""+ _* _+ "is" "the" "on-" @ ] |
                "v1:6" Word [ @ "", _, "ly"* _* "light" "we'll"* ] |
                "v1:7" Word [ "see"* ] |
                "v1:8" Word [ _** _ "no" "I" "won't" @ ] |
                "v2:1" Word [ @ "" _** "be" "a-" "fraid" @ ] |
                "v2:2" Word [ @ "" _* "oh" "I"** ] |
                "v2:3" Word [ "won't"* _*+ "be" "a-"+ _, ] |
                "v2:4" Word [ "fraid"* _*+ "just" "as" "long" ] |
                "v2:5" Word [ ""* _*+ "as" "you" "stand" @ ] |
                "v2:6" Word [ @ ""* _*+ "stand" "by"* ] |
                "v2:7" Word [ "me"* _** _ "so" ] |
                "v2:8" Word [ "dar-"* "ling"* "dar-"* "ling"* ] |
                "c:1" Word [ "stand"* _*+ "by"* "me" @ ] |
                "c:2" Word [ @ "oh" _* "oh" "stand"** ] |
                "c:3" Word [ "", "", "by"* _* "me"*+ ] |
                "c:4" Word [ "" _** "oh"* "oh" ] |
                "c:5" Word [ "stand"*+ _** _ ] |
                "c:6" Word [ _* "stand" "by"* "me"* _ ] |
                "c:7" Word [ _* "stand" "by"*+ "me" _ ] |
                "c:8" Word [ _** _ "if" "the" "sky" @ ] |
                "v3:1" Word [ @ "" _** "that", "we", "look" "u-" @ ] |
                "v3:2" Word [ @ "pon"* _**+ ] |
                "v3:3" Word [ _*+ "should" "tum-" "ble" "and"* ] |
                "v3:4" Word [ "fall"* _*+ "or" "the" "moun-" ] |
                "v3:5" Word [ "tain"*+ _** "should" ] |
                "v3:6" Word [ "crum-" "ble" @ ""*+ "to" "the"* ] |
                "v3:7" Word [ "sea"* _**+ ] |
                "v3:8" Word [ _** _ "I" "won't" "cry" @ ] |
                "v4:1" Word [ @ "" _*+ "I" "won't"+ _, ] |
                "v4:2" Word [ @ "cry"* _ "no" "I"** ] |
                "v4:3" Word [ "wont" _*+ "shed" "a"* ] |
                "v4:4" Word [ "tear"* _*+ "just" "as" "long" ] |
                "v4:5" Word [ ""*+ _* "as" "you" "stand" ] |
                "v4:6" Word [ ""*+ _* "stand" "by"* ] |
                "v4:7" Word [ "me"* _** _ "oh" ] |
                "v4:8" Word [ "dar-"* "ling"* "dar-"* "ling"* ] |
            ]}
            {vocal Vocal [
                $key = A
                $scale = Major
                $duration = _1_8
                "i:8" Tone [ _** _ 3 5 6 @ ] |
                "v1:1" Tone [ @ 6 _** _ 3 5 @ ] |
                "v1:2" Tone [ @ 5 ] |
                "v1:3" Tone [ _*+ 1, 2, 3+ 2+ 1 @ ] |
                "v1:4" Tone [ @ 1* _*+ 1 2 3 ] |
                "v1:5" Tone [ 2+ _* _+ 1 3 2 @ ] |
                "v1:6" Tone [ @ 2, _, 2* _* 2 2* ] |
                "v1:7" Tone [ 1* ] |
                "v1:8" Tone [ _** _ 3 5 6 @ ] |
                "v2:1" Tone [ @ 6 _** 3 5 6 @ ] |
                "v2:2" Tone [ @ 6 _* 5 4 3 2 1, 2, ] |
                "v2:3" Tone [ 3* _*+ 2 1+ _, ] |
                "v2:4" Tone [ 1* _*+ 1 2 3 ] |
                "v2:5" Tone [ 2* _*+ 1 3 2 @ ] |
                "v2:6" Tone [ @ 2* _*+ 3 2* ] |
                "v2:7" Tone [ 1* _** _ 5 ] |
                "v2:8" Tone [ 6* 5* ^1* 7* ] |
                "c:1" Tone [ 6* _*+ 6* 6 @ ] |
                "c:2" Tone [ @ 6 _* 5 6*+ 3, 2, ] |
                "c:3" Tone [ 1, 2, 3* _* 3*+ ] |
                "c:4" Tone [ 1 _** 3* 2 ] |
                "c:5" Tone [ 1*+ _** _ ] |
                "c:6" Tone [ _* 3 2* 1* _ ] |
                "c:7" Tone [ _* 3 2*+ 1 _ ] |
                "c:8" Tone [ _** _ 3 5 6 @ ] |
                "v3:1" Tone [ @ 6 _** 3, 5, 6 6 @ ] |
                "v3:2" Tone [ @ 5* _**+ ] |
                "v3:3" Tone [ _*+ 2 3 3 2* ] |
                "v3:4" Tone [ 1* _*+ 1 2 3 ] |
                "v3:5" Tone [ 1*+ _** 3 ] |
                "v3:6" Tone [ 3 2 1*+ 1 2* ] |
                "v3:7" Tone [ 1* _**+ ] |
                "v3:8" Tone [ _** _ 3 5 6 @ ] |
                "v4:1" Tone [ @ 6 _*+ 3 6+ _, ] |
                "v4:2" Tone [ @ 6* _ 5 6 5* 1, 2, ] |
                "v4:3" Tone [ 3 _*+ 1 2* ] |
                "v4:4" Tone [ 1* _*+ 1 2 3 ] |
                "v4:5" Tone [ 1*+ _* 1 3 2 ] |
                "v4:6" Tone [ 1*+ _* 3 2* ] |
                "v4:7" Tone [ 1* _** _ 5 ] |
                "v4:8" Tone [ 6* 5* ^1* 7* ] |
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
                    lyrics [ "i:8" | ]
                    vocal [ "i:8" | ]
                }
            ]}
            {verse Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                    lyrics [ "v1:1" | @ 1 ; "v2:1" | @ 2 ; "v3:1" | @ 3 ; "v4:1" | @ 4 ]
                    vocal [ "v1:1" | @ 1 ; "v2:1" | @ 2 ; "v3:1" | @ 3 ; "v4:1" | @ 4 ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:2" | ]
                    lyrics [ "v1:2" | @ 1 ; "v2:2" | @ 2 ; "v3:2" | @ 3 ; "v4:2" | @ 4 ]
                    vocal [ "v1:2" | @ 1 ; "v2:2" | @ 2 ; "v3:2" | @ 3 ; "v4:2" | @ 4 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "#Fm" 1 ; "i:3" | ]
                    lyrics [ "v1:3" | @ 1 ; "v2:3" | @ 2 ; "v3:3" | @ 3 ; "v4:3" | @ 4 ]
                    vocal [ "v1:3" | @ 1 ; "v2:3" | @ 2 ; "v3:3" | @ 3 ; "v4:3" | @ 4 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "#Fm" 1 ; "i:4" | ]
                    lyrics [ "v1:4" | @ 1 ; "v2:4" | @ 2 ; "v3:4" | @ 3 ; "v4:4" | @ 4 ]
                    vocal [ "v1:4" | @ 1 ; "v2:4" | @ 2 ; "v3:4" | @ 3 ; "v4:4" | @ 4 ]
                }
                {
                    chord [ "4" 1 ]
                    guitar [ "D" 1 ; "i:5" | ]
                    lyrics [ "v1:5" | @ 1 ; "v2:5" | @ 2 ; "v3:5" | @ 3 ; "v4:5" | @ 4 ]
                    vocal [ "v1:5" | @ 1 ; "v2:5" | @ 2 ; "v3:5" | @ 3 ; "v4:5" | @ 4 ]
                } {
                    chord [ "5" 1 ]
                    guitar [ "E" 1 ; "i:6" | ]
                    lyrics [ "v1:6" | @ 1 ; "v2:6" | @ 2 ; "v3:6" | @ 3 ; "v4:6" | @ 4 ]
                    vocal [ "v1:6" | @ 1 ; "v2:6" | @ 2 ; "v3:6" | @ 3 ; "v4:6" | @ 4 ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                    lyrics [ "v1:7" | @ 1 ; "v2:7" | @ 2 ; "v3:7" | @ 3 ; "v4:7" | @ 4 ]
                    vocal [ "v1:7" | @ 1 ; "v2:7" | @ 2 ; "v3:7" | @ 3 ; "v4:7" | @ 4 ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                    lyrics [ "v1:8" | @ 1 ; "v2:8" | @ 2 ; "v3:8" | @ 3 ; "v4:8" | @ 4 ]
                    vocal [ "v1:8" | @ 1 ; "v2:8" | @ 2 ; "v3:8" | @ 3 ; "v4:8" | @ 4 ]
                }
            ]}
            {chorus Chorus [
                {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                    lyrics [ "c:1" | ]
                    vocal [ "c:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:2" | ]
                    lyrics [ "c:2" | ]
                    vocal [ "c:2" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "#Fm" 1 ; "i:3" | ]
                    lyrics [ "c:3" | ]
                    vocal [ "c:3" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "#Fm" 1 ; "i:4" | ]
                    lyrics [ "c:4" | ]
                    vocal [ "c:4" | ]
                }
                {
                    chord [ "4" 1 ]
                    guitar [ "D" 1 ; "i:5" | ]
                    lyrics [ "c:5" | ]
                    vocal [ "c:5" | ]
                } {
                    chord [ "5" 1 ]
                    guitar [ "E" 1 ; "i:6" | ]
                    lyrics [ "c:6" | ]
                    vocal [ "c:6" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                    lyrics [ "c:7" | ]
                    vocal [ "c:7" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "A" 1 ; "i:1" | ]
                    lyrics [ "c:8" | @ 1 ]
                    vocal [ "c:8" | @ 1 ]
                }
            ]}
        ]
        Form: intro verse verse chorus verse verse chorus
    }
}
