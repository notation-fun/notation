use notation_bevy::bevy::prelude::bevy_main;

use notation_kb::assets::NotationKnowledgeBaseAssets;

#[bevy_main]
fn main() {
    notation_kb::kb::NotationKnowledgeBase::run::<NotationKnowledgeBaseAssets>();
}

