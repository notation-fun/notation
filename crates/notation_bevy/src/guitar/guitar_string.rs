use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutSize};
use notation_model::prelude::{
    Duration, Fretboard6, HandShape6, Pick, PlaySpeed, PlayingState, SyllableNote, TabMeta, Units,
};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct GuitarStringData {
    pub string: u8,
    pub upper: bool,
    pub fret: Option<u8>,
    pub pick_fret: Option<u8>,
    pub capo: u8,
    pub note: Option<SyllableNote>,
    pub state: PlayingState,
    pub hit: bool,
    pub hit_duration: Duration,
    pub hit_seconds: f32,
    pub hit_expired_seconds: f64,
    pub guitar_size: LayoutSize,
}

impl GuitarStringData {
    pub fn new(string: u8, upper: bool) -> Self {
        Self {
            string,
            upper,
            fret: None,
            pick_fret: None,
            capo: 0,
            note: None,
            state: PlayingState::Idle,
            hit: false,
            hit_duration: Duration::Zero,
            hit_seconds: 0.0,
            hit_expired_seconds: 0.0,
            guitar_size: LayoutSize::ZERO,
        }
    }
    fn is_muted(&self) -> bool {
        self.pick_fret.is_none() && self.fret.is_none()
    }
    fn fret(&self) -> u8 {
        self.pick_fret.unwrap_or(self.fret.unwrap_or(0))
    }
    fn width(&self, theme: &NotationTheme) -> f32 {
        let width = theme.guitar.get_string_width(self.string);
        if !self.upper && self.state.is_current() {
            width + theme.guitar.current_extra_width
        } else {
            width
        }
    }
    fn outline(&self, theme: &NotationTheme) -> f32 {
        if !self.upper && self.hit {
            theme.guitar.hit_outline
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
        if self.hit && !hit && time.seconds_since_startup() < self.hit_expired_seconds {
            return;
        }
        self.hit = hit;
        self.hit_duration = hit_duration;
        self.hit_seconds = if hit {
            Self::calc_hit_seconds(hit_duration, hit_string_seconds_range, play_speed)
        } else {
            0.0
        };
        self.hit_expired_seconds = time.seconds_since_startup() + self.hit_seconds as f64;
    }
    fn set_note(&mut self, fretboard: Option<Fretboard6>, meta: Option<Arc<TabMeta>>) {
        self.note = None;
        if let Some(fretboard) = fretboard {
            if let Some(note) = fretboard.fretted_note(self.string, self.fret()) {
                self.note = meta.map(|x| x.calc_syllable_note(&note));
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
    pub fn update(
        &mut self,
        shape: &HandShape6,
        fretboard: Option<Fretboard6>,
        pick: Option<Pick>,
        meta: Option<Arc<TabMeta>>,
    ) {
        let pick_note = pick.and_then(|x| x.get_pick_note(self.string));
        self.pick_fret = pick_note.and_then(|x| x.fret);
        self.fret = shape.string_fret(self.string);
        if let Some(fretboard) = fretboard {
            self.capo = fretboard.capo;
        } else {
            self.capo = 0;
        }
        self.set_note(fretboard, meta);
    }
}

pub struct GuitarString<'a> {
    theme: &'a NotationTheme,
    data: GuitarStringData,
}

impl<'a> LyonShape<shapes::Rectangle> for GuitarString<'a> {
    fn get_name(&self) -> String {
        format!("<GuitarString>({})", self.data.string)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        let fret_y = self.theme.guitar.calc_fret_y(
            self.data.fret() + self.data.capo,
            self.data.guitar_size.height,
        );
        let end_y = if self.data.upper {
            self.data.guitar_size.height * self.theme.guitar.string_y_factor
        } else {
            -self.data.guitar_size.height / 2.0 - 10.0
        };
        let width = self.data.width(self.theme);
        let height = (end_y - fret_y).abs();
        let origin = if end_y > fret_y {
            shapes::RectangleOrigin::BottomLeft
        } else {
            shapes::RectangleOrigin::TopLeft
        };
        shapes::Rectangle {
            width,
            height,
            origin,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let color = if self.data.upper {
            self.theme
                .colors
                .strings
                .string
                .of_state(&PlayingState::Idle)
        } else if self.data.is_muted() {
            self.theme.colors.strings.muted
        } else if self.data.state.is_current() && self.data.note.is_some() {
            self.theme
                .colors
                .of_syllable(self.data.note.unwrap().syllable)
        } else {
            self.theme.colors.strings.string.of_state(&self.data.state)
        };
        let outline_color = if self.data.hit {
            self.theme.colors.strings.hit
        } else {
            color
        };
        ShapeColors::outlined(color, outline_color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let outline = self.data.outline(self.theme);
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(outline),
        }
    }
    fn get_transform(&self) -> Transform {
        if self.data.guitar_size.width <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let x = self
            .theme
            .guitar
            .calc_string_x(self.data.string, self.data.guitar_size.width);
        let width = self.data.width(self.theme);
        let fret_y = self.theme.guitar.calc_fret_y(
            self.data.fret() + self.data.capo,
            self.data.guitar_size.height,
        );
        Transform::from_xyz(x - width / 2.0, fret_y, self.theme.core.mini_bar_z + 1.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, GuitarStringData, shapes::Rectangle, GuitarString<'a>>
    for GuitarString<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: GuitarStringData) -> GuitarString<'a> {
        GuitarString::<'a> { theme, data }
    }
}
