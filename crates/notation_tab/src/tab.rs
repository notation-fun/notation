use std::fs::File;
use std::io::Write;
use ron::ser::{to_string_pretty, PrettyConfig};

use notation_proto::prelude::*;

pub fn convert_tab(tab: &Tab) -> String {
    let pretty = PrettyConfig::new()
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    to_string_pretty(&tab, pretty).expect("Serialization failed")
}

pub fn print_tab(tab: &Tab) {
    let s = convert_tab(tab);
    println!("{}", s);
}

pub fn write_tab(tab: &Tab, path: &'static str) {
    let s = convert_tab(tab);
    let mut file = File::create(path).unwrap();
    file.write_all(s.as_bytes()).unwrap();
    println!("Tab written to: `{}` [{}] - {}", path, s.len(), tab);
}