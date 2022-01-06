use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use notation_bevy_utils::prelude::{BevyUtil, OutlineCircle, ShapeOp};
use notation_model::prelude::{Chord, Signature, Tab};

use crate::prelude::{BarData, NotationAssets, NotationTheme, TabState, NotationSettings};

use super::rhythm_beat::RhythmBeatData;
use super::rhythm_indicator::RhythmIndicatorData;

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

impl ShapeOp<NotationTheme, OutlineCircle> for RhythmBarData {
    fn get_shape(&self, theme: &NotationTheme) -> OutlineCircle {
        let color = theme.colors.of_option_chord(self.value.chord);
        let offset = Vec3::new(self.value.offset.x, self.value.offset.y, theme.z.rhythm_bar);
        let outline_width = theme.sizes.chord.diagram_outline.current;
        let outline_color = theme.colors.of_section(self.bar_props.section_ordinal);
        let radius = self.value.radius + outline_width;
        OutlineCircle {
            radius,
            color,
            outline_width,
            outline_color,
            offset,
        }
    }
}

impl RhythmBarData {
    pub fn update_size(
        &mut self,
        commands: &mut Commands,
        theme: &NotationTheme,
        beat_query: &mut Query<(Entity, &mut RhythmBeatData)>,
        indicator_query: &mut Query<(Entity, &mut RhythmIndicatorData)>,
        entity: Entity,
        children: &Children,
        radius: f32,
        offset: Vec2,
    ) {
        self.value.radius = radius;
        self.value.offset = offset;
        self.update(commands, theme, entity);
        for child in children.iter() {
            if let Ok((beat_entity, mut beat_data)) = beat_query.get_mut(*child) {
                beat_data.update_size(commands, theme, beat_entity, radius);
            } else if let Ok((indicator_entity, mut indicator_data)) =
                indicator_query.get_mut(*child)
            {
                indicator_data.update_size(commands, theme, indicator_entity, radius);
            }
        }
    }
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        tab: &Arc<Tab>,
    ) -> Entity {
        let signature = tab.signature();
        let bar_props = tab
            .get_bar_of_ordinal(1)
            .map(|x| x.props)
            .unwrap_or_default();
        let chord = tab.get_bar_of_ordinal(1).and_then(|x| x.get_chord(None));
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
        let bar_entity = bar_data.create(commands, theme, entity);
        let beats = signature.bar_beats;
        for index in 0..beats {
            RhythmBeatData::spawn(commands, theme, bar_entity, bar_props, signature, index);
        }
        RhythmIndicatorData::spawn(commands, theme, bar_entity, bar_props, signature);
        theme
            .texts
            .rhythm
            .spawn_bar_text(commands, assets, bar_entity, "0");
        bar_entity
    }
    pub fn update_rhythm(
        mut commands: Commands,
        settings: Res<NotationSettings>,
        theme: Res<NotationTheme>,
        mut query: Query<(Entity, &TabState), Changed<TabState>>,
        mut bar_query: Query<(Entity, &mut RhythmBarData, &Children)>,
        mut beat_query: Query<(Entity, &mut RhythmBeatData)>,
        mut font_query: Query<&mut Text>,
        mut indicator_query: Query<(Entity, &mut RhythmIndicatorData)>,
    ) {
        if theme._bypass_systems {
            return;
        }
        if settings.hide_chords_view {
            return;
        }
        let mut current_position = None;
        let mut current_bar = None;
        let mut current_chord = None;
        for (_entity, tab_state) in query.iter_mut() {
            current_position = Some(tab_state.play_control.position);
            if let Some(bar) = tab_state.tab.get_bar(tab_state.play_control.position.bar) {
                current_bar = Some(bar.clone());
                current_chord = bar.get_chord(Some(tab_state.play_control.position.bar.in_bar_pos));
                break;
            }
        }
        if current_bar.is_some() {
            let bar_props = current_bar.unwrap().props;
            for (bar_entity, mut bar_data, bar_children) in bar_query.iter_mut() {
                if bar_data.bar_props.bar_ordinal != bar_props.bar_ordinal
                    || bar_data.value.chord != current_chord
                {
                    bar_data.bar_props = bar_props;
                    bar_data.value.chord = current_chord;
                    bar_data.update(&mut commands, &theme, bar_entity);
                }
                for child in bar_children.iter() {
                    if let Ok(mut text) = font_query.get_mut(*child) {
                        let v = bar_props.bar_ordinal.to_string();
                        BevyUtil::set_text_value(&mut text, v);
                    }
                }
            }
            let in_bar_pos = current_position.unwrap().bar.in_bar_pos;
            for (beat_entity, mut beat_data) in beat_query.iter_mut() {
                beat_data.bar_props = bar_props;
                beat_data.value.in_bar_pos = in_bar_pos;
                beat_data.update(&mut commands, &theme, beat_entity);
            }
            for (indicator_entity, mut indicator_data) in indicator_query.iter_mut() {
                indicator_data.bar_props = bar_props;
                indicator_data.value.in_bar_pos = in_bar_pos;
                indicator_data.update(&mut commands, &theme, indicator_entity);
            }
        }
    }
}
