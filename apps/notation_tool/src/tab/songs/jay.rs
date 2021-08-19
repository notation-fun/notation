use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_long_juan_feng() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::A, Scale::Major, Signature::_4_4, Tempo::Bpm(72))
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 3 5 )
                "1/4" Chord ( 1: 3 5 /4 )
                "2-" Chord ( 2: 3- 5 7- )
                "3-" Chord ( 3: 3- 5 7- )
                "4" Chord ( 4: 3 5 7 )
                "5" Chord ( 5: 3 5 )
                "b6" Chord ( %6: 3 5 7- )
                "6-" Chord ( 6: 3- 5 7- )
            ]}
            {guitar Guitar [
                Fretboard capo: 2
                $duration = _1
                "G" Shape ( 3 2 0 0 0 3 )
                "Em7" Shape ( 0 2 0 0 0 0 )
                "G/C" Shape ( 0 3 0 0 0 0 )
                "Am7" Shape ( 0 0 2 0 1 0 )
                "D" Shape ( 0 0 0 2 3 2 )
                "Cmaj7" Shape ( 0 3 2 0 0 0 )
                "Bm7" Shape ( 2 2 4 2 3 2 )
                "bEdim7" Shape ( 0 0 1 2 1 2 )
                $duration = _1_8
                "i:1,2" Pick [ (6 3) 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "i:3" Pick [ (5 3) 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "i:4" Pick [ 3 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "v:1,2" Pick [ (6 3) 4 2 2, 3, (6 3) 4 2@3* ] |
                "v:3" Pick [ (5 3) 4 2 2, 3, (5 3) 4 2@1* ] |
                $duration = _1
                "v:4" Pick [ (4 3 2 1) ] |
                $duration = _1_8
                "b:1" Pick [ 5 4 2@3 3 2 3 2@3 3 ] |
                "b:2" Pick [ 5 4 2 3 3@4 3 2* ] |
                "b:3" Pick [ 5 4 2 4 (5 3) 4 2 4 ] |
                "b:4" Pick [ 4 3 2 1@3 1** ] |
            ]}
            {vocal Vocal [
                $key = A
                $scale = Major
                $duration = _1_4
                "v:1,2" Tone [ .5 2 2, 1, 3 ] |
                "v:3" Tone [ .6 3 3, 2, 4 ] |
                "v:4_1" Tone [ .%6 3,+ 4,, 3, 2, 1, .%6,, .5,, ] |
                "v:4_2" Tone [ .%6 3,+ 4,, 3, 2, 1, 5,, 6,, ] |
                $duration = _1_8
                "b:1" Tone [ 6 3 3 1 1 .5 1 5 ] |
                "b:2" Tone [ 5 2 2 .7 .7 .5 .7 1 ] |
                "b:3" Tone [ _ .6 1 .6 1 .6, 1+ 2, 3, @ ] |
                "b:4" Tone [ @ 3 2 @ 2** 3 4 ] |
            ]}
            {lyrics Lyrics [
                $duration = _1_4
                "1:1" Word [ "爱" "像" "一", "阵", "风" ] |
                "1:2" Word [ "吹" "完" "它", "就", "走" ] |
                "1:3" Word [ "这" "样" "的", "节", "奏" ] |
                "1:4" Word [ "谁" "都",+ "无",, "可", "奈", "何" ] |
                "2:1" Word [ "没" "有" "你", "以", "后" ] |
                "2:2" Word [ "我" "灵" "魂", "失", "控" ] |
                "2:3" Word [ "黑" "云" "在", "降", "落" ] |
                "2:4" Word [ "我" "被",+ "它",, "拖", "着", "走", "静", ] |
                $duration = _1_8
                "b:1" Word [ "静" "悄" "悄" "默" "默" "离" "开" "陷" ] |
                "b:2" Word [ "入" "了" "危" "险" "边" "缘" "Ba-" "by"] |
                "b:3" Word [ _ "我" "的" "世" "界" "已", "狂"+ "风", "暴", @ ] |
                "b:4" Word [ @ "" "雨" @ ""** "呜"* ] |
            ]}
        ]
        Sections: [
            {intro Intro [
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "i:1,2" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em7" 1 ; "i:1,2" | ]
                } {
                    chord [ "1/4" 1 ]
                    guitar [ "G/C" 1 ; "i:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "i:4" | ]
                }
            ]}
            {verse Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "v:1,2" | ]
                    vocal [ "v:1,2" | ]
                    lyrics [ "1:1" | @ 1 ; "2:1" | @ 2 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em7" 1 ; "v:1,2" | ]
                    vocal [ "v:1,2" | ]
                    lyrics [ "1:2" | @ 1 ; "2:2" | @ 2 ]
                } {
                    chord [ "4" 1 ]
                    guitar [ "Cmaj7" 1 ; "v:3" | ]
                    vocal [ "v:3" | ]
                    lyrics [ "1:3" | @ 1 ; "2:3" | @ 2 ]
                } {
                    chord [ "b6" 1 ]
                    guitar [ "bEdim7" 1 ; "v:4" | ]
                    vocal [ "v:4_1" | @ 1 ; "v:4_2" | @ 2 ]
                    lyrics [ "1:4" | @ 1 ; "2:4" | @ 2 ]
                }
            ]}
            {bridge Bridge [
                {
                    chord [ "4" 1 ]
                    guitar [ "Cmaj7" 1 ; "b:1" | ]
                    vocal [ "b:1" | ]
                    lyrics [ "b:1" | ]
                } {
                    chord [ "3-" 1 ]
                    guitar [ "Bm7" 1 ; "b:2" | ]
                    vocal [ "b:2" | ]
                    lyrics [ "b:2" | ]
                } {
                    chord [ "2-" 1 ]
                    guitar [ "Am7" 1 ; "b:3" | ]
                    vocal [ "b:3" | ]
                    lyrics [ "b:3" | ]
                } {
                    chord [ "5" 1 ]
                    guitar [ "D" 1 ; "b:4" | ]
                    vocal [ "b:4" | ]
                    lyrics [ "b:4" | ]
                }
            ]}
        ]
        Form: intro verse verse bridge
    }
}
