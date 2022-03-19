use bevy::prelude::*;
use notation_bevy_utils::prelude::{ShapeOp, StrokeLine};
use notation_model::{bar_lane::BarLane, prelude::Semitones};

use crate::prelude::{LaneData, NotationTheme, Note};

use super::tone_mode::ToneMode;

#[derive(Clone, Debug)]
pub struct ToneLineValue {
    pub mode: ToneMode,
    pub index : usize,
    pub is_root: bool,
    pub note: Note,
    pub bar_size: f32,
}

pub type ToneLineData = LaneData<ToneLineValue>;

impl ToneLineData {
    pub fn add_lines(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        lane: &BarLane,
        mode: ToneMode,
    ) {
        if let Some(bar) = lane.bar() {
            let scale = bar.tab_meta().scale;
            let key = bar.tab_meta().key;
            let syllables = scale.get_syllables();
            let root = syllables[0];
            let mut show_line = true;
            for semitones in theme.sizes.harmony.lowest.0 ..= theme.sizes.harmony.highest.0 {
                let note = scale.calc_note_from_semitones(&key, Semitones(semitones));
                let index = syllables.iter().position(|&x| x == note.syllable);
                if index.is_some() {
                    let is_root = note.syllable == root;
                    if mode == ToneMode::Melody && !is_root {
                        continue;
                    }
                    if !show_line {
                        show_line = true;
                        continue;
                    }
                    show_line = false;
                    let index = index.unwrap();
                    let data = ToneLineData::new(lane, ToneLineValue {
                        mode,
                        index,
                        is_root,
                        note,
                        bar_size: 0.0,
                    });
                    data.create(commands, theme, entity);
                }
            }
        }
    }
}

impl ShapeOp<NotationTheme, StrokeLine> for ToneLineData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeLine {
        let sizes = match self.value.mode {
            ToneMode::Melody => theme.sizes.melody,
            _ => theme.sizes.harmony,
        };
        let y = sizes.calc_note_y(self.value.note);
        let color = theme.colors.bar.line_color;
        let line_width = if self.value.is_root {
            theme.sizes.bar.grid_root_line_width
        } else {
            theme.sizes.bar.grid_line_width
        };
        StrokeLine {
            from: Vec2::ZERO,
            to: Vec2::new(self.value.bar_size, 0.0),
            line_width,
            color,
            offset: Vec3::new(0.0, y, theme.z.grid),
        }
    }
}
