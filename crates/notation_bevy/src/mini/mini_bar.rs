use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, GridCell, LayoutAnchor, LayoutChangedQuery, View, ViewBundle};
use notation_model::prelude::{PlayingState, Syllable, TabBar};

use crate::prelude::{BarData, BarPlaying, LyonShape, LyonShapeOp, NotationTheme};
use crate::ui::layout::NotationLayout;

use super::mini_section_separator::{
    MiniSectionSeparator, MiniSectionSeparatorData, MiniSectionSeparatorValue,
};

pub type MiniBar = BarData<Arc<TabBar>>;

#[derive(Clone, Debug)]
pub struct MiniBarValue {
    pub width: f32,
    pub syllable: Syllable,
    pub playing_state: PlayingState,
}
impl MiniBarValue {
    pub fn new(width: f32, syllable: Syllable) -> Self {
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

pub struct MiniBarShape<'a> {
    theme: &'a NotationTheme,
    data: MiniBarData,
}

impl<'a> LyonShape<shapes::Rectangle> for MiniBarShape<'a> {
    fn get_name(&self) -> String {
        format!("{}: {:?}", self.data.bar_props.bar_ordinal, self.data.value)
    }
    fn get_shape(&self) -> shapes::Rectangle {
        let (mut width, mut height) = (self.data.value.width, self.theme.sizes.mini_map.bar_height);
        let outline = self
            .theme
            .sizes
            .mini_map
            .bar_outline
            .of_state(&self.data.value.playing_state);
        if self.data.value.playing_state.is_current() {
            width += outline;
            height += outline;
        } else {
            width -= outline;
            height -= outline;
        }
        shapes::Rectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let fill = self.theme.colors.of_syllable(self.data.value.syllable);
        let outline = self
            .theme
            .colors
            .mini_map
            .bar_outline
            .of_state(&self.data.value.playing_state);
        ShapeColors::outlined(fill, outline)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(
                self.theme
                    .sizes
                    .mini_map
                    .bar_outline
                    .of_state(&self.data.value.playing_state),
            ),
        }
    }
    fn get_transform(&self) -> Transform {
        if self.data.value.width <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let mut z = self.theme.core.mini_bar_z;
        if self.data.value.playing_state.is_current() {
            z += 2.0;
        }
        Transform::from_xyz(0.0, 0.0, z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, MiniBarData, shapes::Rectangle, MiniBarShape<'a>>
    for MiniBarShape<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: MiniBarData) -> MiniBarShape<'a> {
        MiniBarShape::<'a> { theme, data }
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
        theme: &NotationTheme,
        entity: Entity,
        bar: &Arc<TabBar>,
    ) -> Entity {
        let bar_entity = BevyUtil::spawn_child_bundle(
            commands,
            entity,
            ViewBundle::from(MiniBar::new(bar, bar.clone())),
        );
        let syllable = bar.get_chord(None).map(|x| x.root).unwrap_or(Syllable::Do);
        let value = MiniBarValue::new(0.0, syllable);
        let data = MiniBarData::new(bar, value);
        let shape_entity = MiniBarShape::create(commands, theme, bar_entity, data);
        commands
            .entity(shape_entity)
            .insert(BarPlaying::new(bar, PlayingState::Idle));
        if bar.props.bar_index == 0 {
            let section_separator_data =
                MiniSectionSeparatorData::new(bar, MiniSectionSeparatorValue::new(0.0));
            MiniSectionSeparator::create(commands, theme, bar_entity, section_separator_data);
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
                MiniBarShape::update(&mut commands, &theme, entity, &data);
            }
            for (entity, mut data) in mini_section_separator_query.iter_mut() {
                data.value.width = layout.size.width;
                MiniSectionSeparator::update(&mut commands, &theme, entity, &data);
            }
        }
    }
}
