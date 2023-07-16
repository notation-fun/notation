use std::fmt::Display;

use edger_bevy_app::bevy_prelude::*;
use edger_bevy_app::bevy_prototype_lyon::prelude::*;
use edger_bevy_app::prelude::{FillRectangle, LayoutSize, ShapeOp};

use crate::prelude::{BarData, NotationTheme, TabState};
use notation_model::prelude::{Signature, TabBar};

#[derive(Clone, Debug)]
pub struct BarBeatValue {
    pub signature: Signature,
    pub bar_beats: u8,
    pub beat: u8,
    pub bar_size: LayoutSize,
    pub selected: bool,
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
            selected: false,
        }
    }
}

pub type BarBeatData = BarData<BarBeatValue>;

impl ShapeOp<NotationTheme, FillRectangle> for BarBeatData {
    fn get_shape(&self, theme: &NotationTheme) -> FillRectangle {
        let signature = self.value.signature;
        let color =
            theme
                .colors
                .bar
                .get_beat_color(&signature, self.value.beat, self.value.selected);
        let x = self.value.bar_size.width / self.value.bar_beats as f32 * self.value.beat as f32;
        FillRectangle {
            width: self.value.bar_size.width / self.value.bar_beats as f32,
            height: self.value.bar_size.height + theme.sizes.bar.bar_beat_extra * 2.0,
            origin: shapes::RectangleOrigin::TopLeft,
            color,
            offset: Vec3::new(x, theme.sizes.bar.bar_beat_extra, theme.z.beat),
        }
    }
}

impl BarBeatData {
    pub fn update_all(
        commands: &mut Commands,
        theme: &NotationTheme,
        tab_state: &TabState,
        beat_query: &mut Query<(Entity, &mut BarBeatData)>,
    ) {
        for (entity, mut data) in beat_query.iter_mut() {
            data.value.selected = tab_state.is_bar_selected(data.bar_props.bar_ordinal);
            data.update(commands, theme, entity);
        }
    }
}
