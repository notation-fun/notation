use std::collections::HashMap;

use notation_proto::prelude::*;

pub mod amature_guitar;

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
    vec![TabInfo::new(
        "ag:1_right_hand",
        amature_guitar::new_tab_1_right_hand,
    )]
}

pub fn get_tab_map<'a>() -> HashMap<&'a str, TabInfo<'a>> {
    get_tab_list()
        .into_iter()
        .map(|tab| (tab.name, tab))
        .collect()
}

pub fn new_tab<'a>(name: &'a str) -> Option<Tab> {
    get_tab_map().get(name).map(|x| (x.new_tab)())
}
