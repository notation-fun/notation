use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::config::bevy_config::BevyConfig;
use crate::prelude::{LyonShape, LyonShapeOp};
use notation_model::prelude::{Signature, TabBar, Units};

#[derive(Clone, Debug)]
pub struct BarBeatData {
    pub signature: Signature,
    pub beat_units: Units,
    pub bar_ordinal: usize,
    beat: u8,
}

impl BarBeatData {
    pub fn new(tab_bar: &Arc<TabBar>, signature: &Signature, beat: u8) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        let beat_units = Units::from(signature.beat_unit);
        BarBeatData {
            signature: signature.clone(),
            beat_units,
            bar_ordinal,
            beat,
        }
    }
    pub fn may_new(
        config: &BevyConfig,
        tab_bar: &Arc<TabBar>,
        signature: &Signature,
        beat: u8,
    ) -> Option<Self> {
        config
            .theme
            .core
            .get_beat_color(signature, beat)
            .map(|_color| Self::new(tab_bar, signature, beat))
    }
}

pub struct BarBeat<'a> {
    config: &'a BevyConfig,
    data: BarBeatData,
}

impl<'a> LyonShape<shapes::Rectangle> for BarBeat<'a> {
    fn get_name(&self) -> String {
        format!("{}:{}", self.data.bar_ordinal, self.data.beat)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.config.grid.unit_size * self.data.beat_units.0,
            height: (self.config.grid.bar_beat_top - self.config.grid.bar_beat_bottom),
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let signature = self.data.signature;
        let color = self
            .config
            .theme
            .core
            .get_beat_color(&signature, self.data.beat);
        ShapeColors::new(color.unwrap_or(self.config.theme.core.background_color))
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        let x = self.config.grid.unit_size * self.data.beat_units.0 * self.data.beat as f32;
        Transform::from_xyz(
            x,
            self.config.grid.bar_beat_bottom,
            self.config.theme.core.beat_z,
        )
    }
}

impl<'a> LyonShapeOp<'a, BarBeatData, shapes::Rectangle, BarBeat<'a>> for BarBeat<'a> {
    fn new_shape(config: &'a BevyConfig, data: BarBeatData) -> BarBeat<'a> {
        BarBeat::<'a> { config, data }
    }
}
