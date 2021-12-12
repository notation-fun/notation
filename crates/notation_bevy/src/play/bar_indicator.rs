use std::sync::Arc;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, LayoutData, StrokeRectangle, ShapeOp};
use notation_model::prelude::{TabBarProps, Chord, Tab, Units};

use crate::prelude::{NotationTheme};

#[derive(Clone, Debug)]
pub struct BarIndicatorData {
    pub tab: Arc<Tab>,
    pub bar_props: TabBarProps,
    pub bar_layout: LayoutData,
    pub chord: Option<Chord>,
}

impl BarIndicatorData {
    pub fn new(tab: Arc<Tab>) -> Self {
        BarIndicatorData {
            tab,
            bar_props: TabBarProps::default(),
            bar_layout: LayoutData::ZERO,
            chord: None,
        }
    }
}

impl ShapeOp<NotationTheme, StrokeRectangle> for BarIndicatorData {
    fn get_shape(&self, theme: &NotationTheme) -> StrokeRectangle {
        let offset = if self.bar_layout.size.width <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let x = self.bar_layout.offset.x;
            let y = self.bar_layout.offset.y + theme.sizes.bar.bar_separator_extra;
            Vec3::new(x, y, theme.core.bar_indicator_z)
        };
        let color = theme
            .colors
            .of_option_chord(self.chord);
        StrokeRectangle {
            width: self.bar_layout.size.width,
            height: self.bar_layout.size.height + theme.sizes.bar.bar_separator_extra * 2.0,
            origin: shapes::RectangleOrigin::TopLeft,
            color, //: theme.colors.bar.bar_indicator,
            line_width: theme.sizes.bar.pos_indicator_size,
            offset,
        }
    }
}

impl BarIndicatorData {
    fn update_chord(&mut self,
        bar_props: TabBarProps,
        in_bar_pos: Option<Units>,
    ) {
        self.bar_props = bar_props;
        self.chord = self.tab.get_bar_of_ordinal(bar_props.bar_ordinal).and_then(|x| x.get_chord(in_bar_pos));
    }
    pub fn update_data(&mut self,
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        bar_props: TabBarProps,
        bar_layout: LayoutData,
        in_bar_pos: Option<Units>,
    ) {
        self.update_chord(bar_props, in_bar_pos);
        self.bar_layout = bar_layout;
        self.update(commands, theme, entity);
    }
    pub fn update_pos(
        commands: &mut Commands,
        theme: &NotationTheme,
        bar_indicator_query: &mut Query<(Entity, &mut BarIndicatorData), With<BarIndicatorData>>,
        bar_props: TabBarProps,
        in_bar_pos: Units,
    ) {
        if let Ok((entity, mut data)) = bar_indicator_query.single_mut() {
            data.update_chord(bar_props, Some(in_bar_pos));
            data.update(commands, theme, entity);
        }
    }
}