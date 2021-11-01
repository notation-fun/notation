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
        TabInfo::new("test", test::new_tab),
        TabInfo::new("beginner:1_right_hand", beginner::right_hand_1::new_tab),
        TabInfo::new(
            "songs:pu_shu:bai_hua_lin",
            songs::pu_shu::bai_hua_lin::new_tab,
        ),
        TabInfo::new(
            "songs:jay:long_juan_feng",
            songs::jay::long_juan_feng::new_tab,
        ),
        TabInfo::new(
            "songs:misc:stand_by_me",
            songs::misc::stand_by_me::new_tab,
        ),
        TabInfo::new(
            "songs:misc:scarborough_fair",
            songs::misc::scarborough_fair::new_tab,
        ),
        TabInfo::new(
            "songs:misc:scarborough_fair",
            songs::misc::scarborough_fair_hard::new_tab,
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
