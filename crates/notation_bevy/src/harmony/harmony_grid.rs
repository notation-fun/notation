use bevy::prelude::*;
use notation_bevy_utils::prelude::ShapeOp;

use crate::{prelude::{NotationTheme, NotationSettings}, tone::tone_mode::ToneMode};
use notation_model::{bar_lane::BarLane, prelude::{Note, Semitones}};

use crate::tone::tone_line::{ToneLineValue, ToneLineData};

#[derive(Debug, Default, Component)]
pub struct HarmonyGrid();

impl HarmonyGrid {
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
            let syllables = scale.get_syllables();
            for semitones in theme.sizes.harmony.lowest.0 ..= theme.sizes.harmony.highest.0 {
                let note = Note::from(Semitones(semitones));
                let syllable_note = scale.calc_syllable_note(&key, &note);
                if syllables.contains(&syllable_note.syllable) {
                    let data = ToneLineData::new(lane, ToneLineValue {
                        mode: ToneMode::Harmony,
                        is_root: syllables[0] == syllable_note.syllable,
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
