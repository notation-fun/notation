use tab_viewer::{bevy::prelude::bevy_main, prelude::NotationArgs};

use notation_kb::assets::NotationKnowledgeBaseAssets;

#[bevy_main]
fn main() {
    let args = NotationArgs::parse_args();
    notation_kb::kb::NotationKnowledgeBase::run::<NotationKnowledgeBaseAssets>(args);
}

