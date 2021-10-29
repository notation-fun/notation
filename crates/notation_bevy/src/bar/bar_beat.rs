use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{FillRectangle, LayoutSize, ShapeOp};

use crate::prelude::{BarData, NotationTheme};
use notation_model::prelude::{Signature, TabBar};

#[derive(Clone, Debug)]
pub struct BarBeatValue {
    pub signature: Signature,
    pub bar_beats: u8,
    pub beat: u8,
    pub bar_size: LayoutSize,
}
impl Display for BarBeatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl BarBeatValue {
    pub fn new(tab_bar: &TabBar, signature: &Signature, beat: u8) -> Self {
        let bar_beats = tab_bar.bar_beats();
        BarBeatValue {
            signature: *signature,
            bar_beats,
            beat,
            bar_size: LayoutSize::ZERO,
        }
    }
    pub fn may_new(
        theme: &NotationTheme,
        tab_bar: &TabBar,
        signature: &Signature,
        beat: u8,
    ) -> Option<Self> {
        theme
            .colors.bar
            .get_beat_color(signature, beat)
            .map(|_color| Self::new(tab_bar, signature, beat))
    }
}

pub type BarBeatData = BarData<BarBeatValue>;

impl ShapeOp<NotationTheme, FillRectangle> for BarBeatData {
    fn get_shape(&self, theme: &NotationTheme) -> FillRectangle {
        let signature = self.value.signature;
        let color = theme
            .colors.bar
            .get_beat_color(&signature, self.value.beat)
            .unwrap_or(theme.core.background_color);
        let x = self.value.bar_size.width / self.value.bar_beats as f32
            * self.value.beat as f32;
        FillRectangle {
            width: self.value.bar_size.width / self.value.bar_beats as f32,
            height: self.value.bar_size.height + theme.sizes.bar.bar_beat_extra * 2.0,
            origin: shapes::RectangleOrigin::TopLeft,
            color,
            offset: Vec3::new(
                x, theme.sizes.bar.bar_beat_extra, theme.core.beat_z,
            ),
        }
    }
}