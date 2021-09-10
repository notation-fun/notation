use std::fmt::Display;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy_utils::prelude::BevyUtil;
use notation_model::prelude::{Chord, Signature, TabBarProps};

use crate::{prelude::{BarData, LyonShape, LyonShapeOp, NotationAssets, NotationTheme, TabState}};

use super::rhythm_beat::{RhythmBeat, RhythmBeatData};

#[derive(Clone, Debug)]
pub struct RhythmBarValue {
    pub signature: Signature,
    pub chord: Option<Chord>,
    pub radius: f32,
    pub offset: Vec2,
}
impl Display for RhythmBarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RhythmBarData>({})", self.signature)
    }
}

pub type RhythmBarData = BarData<RhythmBarValue>;

pub struct RhythmBar<'a> {
    theme: &'a NotationTheme,
    data: RhythmBarData,
}

impl<'a> LyonShape<shapes::Circle> for RhythmBar<'a> {
    fn get_name(&self) -> String {
        format!("{}", self.data)
    }
    fn get_shape(&self) -> shapes::Circle {
        let radius = self.data.value.radius;
        shapes::Circle {
            center: Vec2::ZERO,
            radius,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        let fill = self.theme.colors.of_option_chord(self.data.value.chord);
        ShapeColors::new(fill)
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Fill(FillOptions::default())
    }
    fn get_transform(&self) -> Transform {
        if self.data.value.radius <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        Transform::from_xyz(self.data.value.offset.x, self.data.value.offset.y, self.theme.core.mini_bar_z)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, RhythmBarData, shapes::Circle, RhythmBar<'a>>
    for RhythmBar<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: RhythmBarData) -> RhythmBar<'a> {
        RhythmBar::<'a> { theme, data }
    }
}

impl<'a> RhythmBar<'a> {
    pub fn update_size(
        commands: &mut Commands,
        theme: &NotationTheme,
        beat_query: &mut Query<(Entity, &mut RhythmBeatData)>,
        entity: Entity,
        data: &mut RhythmBarData,
        children: &Children,
        radius: f32,
        offset: Vec2,
    ) {
        data.value.radius = radius;
        data.value.offset = offset;
        RhythmBar::update(commands, theme, entity, data);
        for child in children.iter() {
            if let Ok((beat_entity, mut beat_data)) =
                beat_query.get_mut(*child)
            {
                RhythmBeat::update_size(
                    commands,
                    theme,
                    beat_entity,
                    &mut beat_data,
                    radius,
                );
            }
        }
    }
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        bar_props: TabBarProps,
        signature: Signature,
        chord: Option<Chord>,
    ) -> Entity {
        let bar_value = RhythmBarValue {
            signature,
            chord,
            radius: 0.0,
            offset: Vec2::ZERO,
        };
        let bar_data = RhythmBarData {
            bar_props,
            value: bar_value,
        };
        let bar_entity = RhythmBar::create(commands, theme, entity, bar_data);
        let beats = signature.bar_beats;
        for index in 0..beats {
            RhythmBeat::spawn(commands, theme, bar_entity, bar_props, signature, index);
        }
        theme.texts.rhythm.spawn_bar_text(commands, bar_entity, assets, "0");
        bar_entity
    }
    pub fn update_chord(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        mut query: Query<(Entity, &TabState), Changed<TabState>,>,
        mut bar_query: Query<(Entity, &mut RhythmBarData, &Children)>,
        mut font_query: Query<&mut Text>,
    ) {
        let mut current_bar = None;
        let mut current_chord = None;
        for (_entity, tab_state) in query.iter_mut() {
            if let Some(bar) = tab_state.tab.get_bar(tab_state.play_control.position.bar) {
                current_bar = Some(bar.clone());
                current_chord = bar.get_chord(Some(tab_state.play_control.position.bar.in_bar_pos));
                break;
            }
        }
        if current_chord.is_some() {
            let bar_props = current_bar.unwrap().props;
            for (bar_entity, mut bar_data, bar_children) in bar_query.iter_mut() {
                if bar_data.value.chord != current_chord {
                    bar_data.value.chord = current_chord;
                    RhythmBar::update(&mut commands, &theme, bar_entity, &bar_data);
                }
                for child in bar_children.iter() {
                    if let Ok(mut text) = font_query.get_mut(*child) {
                        let v = bar_props.bar_ordinal.to_string();
                        BevyUtil::set_text_value(&mut text, v);
                    }
                }
            }
        }
    }
}

