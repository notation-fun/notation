use std::sync::Arc;

use bevy::prelude::*;

use notation_model::prelude::{BarPosition, Duration, TabBar, Tone};

use crate::prelude::{LyonShapeOp, NotationSettings, NotationTheme};

use super::{tone_mode::ToneMode, tone_note::{ToneNoteData, ToneNoteShape}};

pub fn new_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(create_tone_notes.system())
}

fn create_tone_notes(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    theme: Res<NotationTheme>,
    _settings: Res<NotationSettings>,
    query: Query<(&Parent, Entity, &Tone, &Duration, &BarPosition), Added<Tone>>,
    layer_query: Query<(&Arc<TabBar>, &ToneMode)>,
) {
    for (parent, entity, tone, duration, pos) in query.iter() {
        if let Ok((bar, mode)) = layer_query.get(parent.0) {
            let bar_units = bar.bar_units();
            for note in tone.get_notes() {
                let data =
                    ToneNoteData::new(bar_units, &bar, *duration, *pos, note, *mode);
                ToneNoteShape::create(
                    &mut commands,
                    entity,
                    &theme,
                    data,
                );
            }
        }
    }
}
