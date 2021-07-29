fn main() {
    #[cfg(target_arch = "wasm32")]
    let tabs = vec![ notation_bevy::prelude::get_tab_from_url().unwrap_or("beginner/1_right_hand.ron".to_owned()) ];

    #[cfg(not(target_arch = "wasm32"))]
    let tabs = vec![
        "songs/jay/long_juan_feng.ron".to_owned(),
        "songs/pu_shu/bai_hua_lin.ron".to_owned(),
        "beginner/1_right_hand.ron".to_owned(),
    ];
    notation_bevy::viewer::app::main(tabs);
}
