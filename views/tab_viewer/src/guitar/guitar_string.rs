use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, LayoutSize, OutlineRectangle, ShapeOp};
use notation_model::prelude::{
    Duration, Fretboard6, HandShape6, Pick, PlaySpeed, PlayingState, Note, TabMeta, Units,
};

use crate::prelude::NotationTheme;

#[derive(Clone, Debug, Component)]
pub struct GuitarStringData {
    pub string: u8,
    pub upper: bool,
    pub fret: Option<u8>,
    pub pick_fret: Option<u8>,
    pub capo: u8,
    pub note: Option<Note>,
    pub state: PlayingState,
    pub hit: bool,
    pub hit_duration: Duration,
    pub hit_seconds: f32,
    pub hit_expired_seconds: f64,
    pub guitar_size: LayoutSize,
}

impl GuitarStringData {
    pub fn new(string: u8, upper: bool, fretboard: Option<Fretboard6>) -> Self {
        let capo = fretboard.map(|x| x.capo).unwrap_or(0);
        Self {
            string,
            upper,
            fret: Some(0),
            pick_fret: None,
            capo,
            note: None,
            state: PlayingState::Idle,
            hit: false,
            hit_duration: Duration::Zero,
            hit_seconds: 0.0,
            hit_expired_seconds: 0.0,
            guitar_size: LayoutSize::ZERO,
        }
    }
    pub fn reset(&mut self) {
        self.fret = Some(0);
        self.note = None;
        self.state = PlayingState::Idle;
        self.hit = false;
    }
    fn is_muted(&self) -> bool {
        self.pick_fret.is_none() && self.fret.is_none()
    }
    fn fret(&self) -> u8 {
        self.pick_fret.unwrap_or(self.fret.unwrap_or(0))
    }
    fn width(&self, theme: &NotationTheme) -> f32 {
        let width = theme.guitar.get_string_width(self.string);
        let width_with_extra = if !self.upper && self.state.is_current() {
            width + theme.guitar.current_extra_width
        } else {
            width
        };
        width_with_extra * self.guitar_size.width / theme.guitar.guitar_width
    }
    fn outline(&self, theme: &NotationTheme) -> f32 {
        if !self.upper && self.hit {
            theme.guitar.hit_outline * self.guitar_size.width / theme.guitar.guitar_width
        } else {
            0.0
        }
    }
    fn calc_hit_seconds(
        hit_duration: Duration,
        hit_string_seconds_range: (f32, f32),
        play_speed: PlaySpeed,
    ) -> f32 {
        let seconds = play_speed.calc_seconds(Units::from(hit_duration));
        BevyUtil::in_range(seconds * 0.5, hit_string_seconds_range)
    }
    pub fn set_hit(
        &mut self,
        hit: bool,
        hit_duration: Duration,
        time: &Time,
        hit_string_seconds_range: (f32, f32),
        play_speed: PlaySpeed,
    ) {
        if self.hit && !hit && time.elapsed_seconds_f64() < self.hit_expired_seconds {
            return;
        }
        self.hit = hit;
        self.hit_duration = hit_duration;
        self.hit_seconds = if hit {
            Self::calc_hit_seconds(hit_duration, hit_string_seconds_range, play_speed)
        } else {
            0.0
        };
        self.hit_expired_seconds = time.elapsed_seconds_f64() + self.hit_seconds as f64;
    }
    fn set_note(&mut self, fretboard: Option<Fretboard6>, meta: Option<Arc<TabMeta>>) {
        self.note = None;
        if let (Some(meta), Some(fretboard)) = (meta, fretboard) {
            if let Some(note) = fretboard.fretted_note(&meta.scale, &meta.key, self.string, self.fret()) {
                self.note = Some(note);
            }
        }
    }
    pub fn update_pick(
        &mut self,
        fretboard: Option<Fretboard6>,
        pick: Pick,
        meta: Option<Arc<TabMeta>>,
    ) {
        let pick_note = pick.get_pick_note(self.string);
        self.pick_fret = pick_note.and_then(|x| x.fret);
        self.set_note(fretboard, meta);
    }
    pub fn update_value(
        &mut self,
        shape: &HandShape6,
        fretboard: Option<Fretboard6>,
        pick: Option<Pick>,
        meta: Option<Arc<TabMeta>>,
    ) {
        let pick_note = pick.and_then(|x| x.get_pick_note(self.string));
        self.pick_fret = pick_note.and_then(|x| x.fret);
        self.fret = shape.string_fret_with_barre(self.string);
        if let Some(fretboard) = fretboard {
            self.capo = fretboard.capo;
        } else {
            self.capo = 0;
        }
        self.set_note(fretboard, meta);
    }
}

impl ShapeOp<NotationTheme, OutlineRectangle> for GuitarStringData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineRectangle {
        let fret_y = theme
            .guitar
            .calc_fret_y(self.fret() + self.capo, self.guitar_size.height);
        let end_y = if self.upper {
            self.guitar_size.height * theme.guitar.string_y_factor
        } else {
            -self.guitar_size.height / 2.0
        };
        let width = self.width(theme);
        let height = (end_y - fret_y).abs();
        let origin = if end_y > fret_y {
            shapes::RectangleOrigin::BottomLeft
        } else {
            shapes::RectangleOrigin::TopLeft
        };
        let color = if self.upper {
            if self.is_muted() {
                theme.colors.strings.muted
            } else {
                theme.colors.strings.string.of_state(&PlayingState::Idle)
            }
        } else if self.is_muted() {
            theme.colors.strings.muted
        } else if self.state.is_current() && self.note.is_some() {
            theme.colors.of_syllable(self.note.unwrap().syllable)
        } else {
            theme.colors.strings.string.of_state(&self.state)
        };
        let outline_color = if self.hit {
            theme.colors.strings.hit
        } else {
            color
        };
        let outline_width = self.outline(theme);
        let x = theme
            .guitar
            .calc_string_x(self.string, self.guitar_size.width);
        let fret_y = theme
            .guitar
            .calc_fret_y(self.fret() + self.capo, self.guitar_size.height);
        let offset = Vec3::new(x - width / 2.0, fret_y, theme.z.guitar_string);
        OutlineRectangle {
            width,
            height,
            origin,
            color,
            outline_width,
            outline_color,
            offset,
        }
    }
}
