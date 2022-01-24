use bevy::prelude::*;
use notation_bevy_utils::prelude::ShapeOp;

use std::sync::Arc;

use crate::{prelude::NotationTheme, settings::notation_settings::NotationSettings, tone::{tone_line::{ToneLineData, ToneLineValue}, tone_mode::ToneMode}};
use notation_model::{prelude::{TabBar, Note, Semitones}, bar_lane::BarLane};

#[derive(Debug, Default, Component)]
pub struct MelodyGrid();

impl MelodyGrid {
    pub fn add_lines(
        &self,
        commands: &mut Commands,
        theme: &NotationTheme,
        _settings: &NotationSettings,
        entity: Entity,
        lane: &BarLane,
    ) {
        if let Some(bar) = lane.bar() {
            let scale = bar.tab_meta().scale;
            let key = bar.tab_meta().key;
            let root = scale.get_syllables()[0];
            for semitones in theme.sizes.melody.lowest.0 ..= theme.sizes.melody.highest.0 {
                let note = Note::from(Semitones(semitones));
                let syllable_note = scale.calc_syllable_note(&key, &note);
                if root == syllable_note.syllable {
                    let data = ToneLineData::new(lane, ToneLineValue {
                        mode: ToneMode::Melody,
                        is_root: false,
                        note,
                        syllable_note,
                        bar_size: 0.0,
                    });
                    data.create(commands, theme, entity);
                }
            }
        }
    }
}
