use notation_dsl::tab;
use notation_proto::prelude::*;

pub fn new_tab_long_juan_feng() -> Tab {
    tab! {
        Meta: TabMeta::new(Key::A, Scale::Major, Signature::_4_4, Tempo::Bpm(72))
        Tracks: [
            {guitar Guitar [
                Fretboard capo: 2
                $duration = _1
                "G" Shape ( 3 2 0 0 0 3 )
                "Em" Shape ( 0 2 0 0 0 0 )
                "G/C" Shape ( 0 3 0 0 0 0 )
                "Am" Shape ( 0 0 2 0 1 0 )
                "D" Shape ( 0 0 0 2 3 0 )
                "C" Shape ( 0 3 2 0 0 0 )
                $duration = _1_8
                "intro-6" Pick [ (6 3) 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "intro-5" Pick [ (5 3) 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "verse-6" Pick [ (6 3) 4 2 2, 3, (6 3) 4 2@3* ] |
                "verse-5" Pick [ (5 3) 4 2 2, 3, (5 3) 4 2@1* ] |
            ]}
            {vocal Vocal [
                $key = A
                $scale = Major
                $duration = _1_4
                "1" Tone [ .5 2 2, 1, 3 ] |
                "2" Tone [ .6 3 3, 2, 4 ] |
                "3_1" Tone [ .%6 3,+ 4,, 3, 2, 1, .%6,, .5,, ] |
                "3_2" Tone [ .%6 3,+ 4,, 3, 2, 1, 5,, 6,, ] |
            ]}
            {lyrics Lyrics [
                $duration = _1_4
                "1:1" Word [ "爱" "像" "一", "阵", "风" ] |
                "1:2" Word [ "吹" "完" "它", "就", "走" ] |
                "1:3" Word [ "这" "样" "的", "节", "奏" ] |
                "1:4" Word [ "谁" "都",+ "无",, "可", "奈", "何", "～",, "～",, ] |
                "2:1" Word [ "没" "有" "你", "以", "后" ] |
                "2:2" Word [ "我" "灵" "魂", "失", "控" ] |
                "2:3" Word [ "黑" "云" "在", "降", "落" ] |
                "2:4" Word [ "我" "被",+ "它",, "拖", "着", "走", "静",, "~",, ] |
            ]}
        ]
        Sections: [
            {intro Intro [
                {
                    guitar [
                        "G" 1
                        "intro-6" |
                    ]
                }
                {
                    guitar [
                        "Em" 1
                        "intro-6" |
                    ]
                }
                {
                    guitar [
                        "G/C" 1
                        "intro-6" |
                    ]
                }
                {
                    guitar [
                        "G" 1
                        "intro-6" |
                    ]
                }
            ]}
            {verse Verse [
                {
                    guitar [
                        "G" 1
                        "verse-6" |
                    ]
                    vocal [
                        "1" |
                    ]
                    lyrics [
                        "1:1" | @ 1
                        "2:1" | @ 2
                    ]
                }
                {
                    guitar [
                        "Em" 1
                        "verse-6" |
                    ]
                    vocal [
                        "1" |
                    ]
                    lyrics [
                        "1:2" | @ 1
                        "2:2" | @ 2
                    ]
                }
                {
                    guitar [
                        "G/C" 1
                        "verse-5" |
                    ]
                    vocal [
                        "2" |
                    ]
                    lyrics [
                        "1:3" | @ 1
                        "2:3" | @ 2
                    ]
                }
                {
                    guitar [
                        "G" 1
                        "verse-6" |
                    ]
                    vocal [
                        "3_1" | @ 1
                        "3_2" | @ 2
                    ]
                    lyrics [
                        "1:4" | @ 1
                        "2:4" | @ 2
                    ]
                }
            ]}
        ]
        Form: intro verse verse
    }
}
