use std::collections::HashMap;

use notation_proto::prelude::*;

pub mod beginner;
pub mod songs;
pub mod test;

pub struct TabInfo<'a> {
    pub name: &'a str,
    pub new_tab: fn() -> Tab,
}

impl<'a> TabInfo<'a> {
    pub fn new(name: &'a str, new_tab: fn() -> Tab) -> Self {
        Self { name, new_tab }
    }
}

pub fn get_tab_list<'a>() -> Vec<TabInfo<'a>> {
    vec![
        TabInfo::new("test", test::new_tab_test),
        TabInfo::new("beginner:1_right_hand", beginner::new_tab_1_right_hand),
        TabInfo::new(
            "songs:pu_shu:bai_hua_lin",
            songs::pu_shu::new_tab_bai_hua_lin,
        ),
        TabInfo::new(
            "songs:jay:long_juan_feng",
            songs::jay::new_tab_long_juan_feng,
        ),
    ]
}

pub fn get_tab_map<'a>() -> HashMap<&'a str, TabInfo<'a>> {
    get_tab_list()
        .into_iter()
        .map(|tab| (tab.name, tab))
        .collect()
}

pub fn new_tab(name: &str) -> Option<Tab> {
    get_tab_map().get(name).map(|x| (x.new_tab)())
}
