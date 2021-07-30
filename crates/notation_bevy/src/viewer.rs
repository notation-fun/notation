use crate::prelude::NotationApp;

pub fn run_notation_viewer(tab_pathes: Vec<String>) {
    NotationApp::run("Notation Viewer", tab_pathes, |_app| {})
}
