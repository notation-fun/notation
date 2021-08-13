use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::prelude::{BarData, BarLayout, LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::{Signature, TabBar};

#[derive(Clone, Debug)]
pub struct BarBeatValue {
    pub signature: Signature,
    pub bar_beats: u8,
    pub bar_height: f32,
    pub beat: u8,
}

impl BarBeatValue {
    pub fn new(
        tab_bar: &Arc<TabBar>,
        signature: &Signature,
        bar_layout: &BarLayout,
        beat: u8,
    ) -> Self {
        let bar_beats = tab_bar.bar_beats();
        let bar_height = bar_layout.height;
        BarBeatValue {
            signature: *signature,
            bar_beats,
            bar_height,
            beat,
        }
    }
    pub fn may_new(
        theme: &NotationTheme,
        tab_bar: &Arc<TabBar>,
        signature: &Signature,
        bar_layout: &BarLayout,
        beat: u8,
    ) -> Option<Self> {
        theme
            .core
            .get_beat_color(signature, beat)
            .map(|_color| Self::new(tab_bar, signature, bar_layout, beat))
    }
}

pub type BarBeatData = BarData<BarBeatValue>;

pub struct BarBeat<'a> {
    theme: &'a NotationTheme,
    data: BarBeatData,
}

impl<'a> LyonShape<shapes::Rectangle> for BarBeat<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{}",
            self.data.bar_props.bar_ordinal, self.data.value.beat
        )
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.theme.grid.bar_size / self.data.value.bar_beats as f32,
            height: (self.data.value.bar_height + self.theme.grid.bar_beat_extra * 2.0),
            origin: shapes::RectangleOrigin::TopLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let signature = self.data.value.signature;
        let color = self
            .theme
            .core
            .get_beat_color(&signature, self.data.value.beat);
        ShapeColors::new(color.unwrap_or(self.theme.core.background_color))
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size / self.data.value.bar_beats as f32
            * self.data.value.beat as f32;
        Transform::from_xyz(x, self.theme.grid.bar_beat_extra, self.theme.core.beat_z)
    }
}

impl<'a> LyonShapeOp<'a, BarBeatData, shapes::Rectangle, BarBeat<'a>> for BarBeat<'a> {
    fn new_shape(theme: &'a NotationTheme, data: BarBeatData) -> BarBeat<'a> {
        BarBeat::<'a> { theme, data }
    }
}
