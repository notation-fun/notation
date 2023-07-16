use tab_viewer::edger_bevy_app::bevy_prelude::bevy_main;
use tab_viewer::prelude::NotationArgs;
use notation_kb::assets::NotationKnowledgeBaseAssets;

#[bevy_main]
fn main() {
    let args = NotationArgs::parse_args();
    notation_kb::kb::NotationKnowledgeBase::run::<NotationKnowledgeBaseAssets>(args);
}

