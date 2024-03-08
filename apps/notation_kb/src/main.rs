use tab_viewer::edger_bevy::bevy_prelude::bevy_main;
use tab_viewer::prelude::NotationArgs;

#[bevy_main]
fn main() {
    let args = NotationArgs::parse_args();
    notation_kb::kb::NotationKnowledgeBase::run(args);
}

