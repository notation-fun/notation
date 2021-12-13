use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_bevy_utils::prelude::{BevyUtil, GridCell, LayoutAnchor, LayoutChangedQuery, OutlineRectangle, ShapeOp, View, ViewBundle};
use notation_model::prelude::{PlayingState, Syllable, TabBar};

use crate::prelude::{BarData, BarPlaying, NotationAssets, NotationTheme};
use crate::ui::layout::NotationLayout;

use super::mini_section_separator::{
    MiniSectionSeparatorData, MiniSectionSeparatorValue,
};

pub type MiniBar = BarData<Arc<TabBar>>;

#[derive(Clone, Debug)]
pub struct MiniBarValue {
    pub width: f32,
    pub syllable: Option<Syllable>,
    pub playing_state: PlayingState,
}
impl MiniBarValue {
    pub fn new(width: f32, syllable: Option<Syllable>) -> Self {
        Self {
            width,
            syllable,
            playing_state: PlayingState::Idle,
        }
    }
}
impl Display for MiniBarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub type MiniBarData = BarData<MiniBarValue>;

impl ShapeOp<NotationTheme, OutlineRectangle> for MiniBarData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineRectangle {
        let (mut width, mut height) = (self.value.width, theme.sizes.mini_map.bar_height);
        let outline_width = theme
            .sizes
            .mini_map
            .bar_outline
            .of_state(&self.value.playing_state);
        if self.value.playing_state.is_current() {
            width += outline_width;
            height += outline_width;
        } else {
            width -= outline_width;
            height -= outline_width;
        }
        let color = theme
            .colors
            .of_option_syllable(self.value.syllable);
        let outline_color = if self.value.playing_state.is_current() {
            theme
            .colors
            .mini_map
            .bar_outline_current
        } else {
            theme.colors.of_section(self.bar_props.section_ordinal)
        };
        let mut z = theme.z.mini_bar;
        if self.value.playing_state.is_current() {
            z += 1.0;
        }
        OutlineRectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
            color,
            outline_width,
            outline_color,
            offset: Vec3::new(0.0, 0.0, z),
        }
    }
}

impl<'a> View<NotationLayout<'a>> for MiniBar {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::CENTER
    }
}
impl<'a> GridCell<NotationLayout<'a>> for MiniBar {
    fn order(&self) -> usize {
        self.bar_props.bar_ordinal
    }
}
impl MiniBar {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        bar: &Arc<TabBar>,
    ) -> Entity {
        let bar_entity = BevyUtil::spawn_child_bundle(
            commands,
            entity,
            ViewBundle::from(MiniBar::new(bar, bar.clone())),
        );
        let syllable = bar.get_chord(None).map(|x| x.root);
        let value = MiniBarValue::new(0.0, syllable);
        let data = MiniBarData::new(bar, value);
        let shape_entity = data.create(commands, theme, bar_entity);
        commands
            .entity(shape_entity)
            .insert(BarPlaying::new(bar, PlayingState::Idle));
        if bar.props.bar_index == 0 && bar.props.section_ordinal > 0 {
            let section_separator_data =
                MiniSectionSeparatorData::new(bar, MiniSectionSeparatorValue::new(0.0));
            section_separator_data.create(commands, theme, bar_entity);
            theme.texts.mini_map.spawn_bar_text(
                commands,
                assets,
                shape_entity,
                bar.props.bar_ordinal.to_string().as_str(),
            );
        }
        bar_entity
    }
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedQuery<MiniBar>,
        mut mini_bar_query: Query<(Entity, &mut MiniBarData)>,
        mut mini_section_separator_query: Query<(Entity, &mut MiniSectionSeparatorData)>,
    ) {
        for (_entity, _view, layout) in query.iter() {
            for (entity, mut data) in mini_bar_query.iter_mut() {
                data.value.width = layout.size.width;
                data.update(&mut commands, &theme, entity);
            }
            for (entity, mut data) in mini_section_separator_query.iter_mut() {
                data.value.width = layout.size.width;
                data.update(&mut commands, &theme, entity);
            }
        }
    }
}
