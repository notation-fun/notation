use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use notation_bevy_utils::prelude::{
    BevyUtil, GridCell, LayoutAnchor, LayoutChangedWithChildrenQuery, View, ViewBundle,
};
use notation_model::prelude::TabChord;

use crate::prelude::{NotationAssets, NotationTheme};
use crate::prelude::NotationLayout;

use super::chord_base::ChordBaseData;
use super::chord_diagram::ChordDiagramData;
use super::chord_interval::ChordIntervalData;
use super::chord_playing::ChordPlaying;
use super::interval_dot::IntervalDotData;

#[derive(Clone, Debug)]
pub struct ChordView {
    pub chord: TabChord,
}

impl Display for ChordView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ChordView>({})", self.chord)
    }
}

impl<'a> View<NotationLayout<'a>> for ChordView {
    fn pivot(&self) -> LayoutAnchor {
        LayoutAnchor::CENTER
    }
}
impl<'a> GridCell<NotationLayout<'a>> for ChordView {}

impl ChordView {
    pub fn on_layout_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: LayoutChangedWithChildrenQuery<ChordView>,
        mut diagram_query: Query<(Entity, &mut ChordDiagramData, &Children)>,
        mut interval_query: Query<(Entity, &mut ChordIntervalData, &Children)>,
        mut base_query: Query<(Entity, &mut ChordBaseData, &Children)>,
        mut dot_query: Query<(Entity, &mut IntervalDotData)>,
        mut text_query: Query<&mut Transform, With<Text>>,
    ) {
        if theme._bypass_systems {
            return;
        }
        for (_entity, _view, layout, children) in query.iter() {
            let radius = layout.size.width * theme.sizes.chord.diagram_factor;
            for child in children.iter() {
                if let Ok((diagram_entity, mut diagram_data, diagram_children)) =
                    diagram_query.get_mut(*child)
                {
                    diagram_data.update_size(
                        &mut commands,
                        &theme,
                        &mut interval_query,
                        &mut base_query,
                        &mut dot_query,
                        diagram_entity,
                        diagram_children,
                        radius,
                    );
                }
                if let Ok(mut transform) = text_query.get_mut(*child) {
                    theme.texts.chord.update_bars_xy(&mut transform, layout);
                }
            }
        }
    }
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        entity: Entity,
        chord: &TabChord,
    ) -> Entity {
        let chord_entity = BevyUtil::spawn_child_bundle(
            commands,
            entity,
            ViewBundle::from(ChordView {
                chord: chord.clone(),
            }),
        );
        //TODO: handle initialization in a nicer way.
        let radius = 0.0;
        ChordDiagramData::spawn(
            commands,
            theme,
            chord_entity,
            chord.first_entry().unwrap().props,
            chord.chord,
            radius,
        );
        commands.entity(chord_entity).insert(ChordPlaying::from((
            chord.first_entry().unwrap().props,
            chord.chord,
        )));
        if chord.bars.len() > 1 {
            theme.texts.chord.spawn_bars_text(
                commands,
                assets,
                chord_entity,
                chord.bars.len().to_string().as_str(),
                theme.z.chord_text,
            );
        }
        chord_entity
    }
    pub fn on_chord_playing_changed(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        mut query: Query<
            (Entity, &ChordPlaying, &Arc<ChordView>, &Children),
            Changed<ChordPlaying>,
        >,
        mut diagram_query: Query<(Entity, &mut ChordDiagramData)>,
    ) {
        if theme._bypass_systems {
            return;
        }
        for (_entity, playing, _view, children) in query.iter_mut() {
            for child in children.iter() {
                if let Ok((diagram_entity, mut diagram_data)) = diagram_query.get_mut(*child) {
                    diagram_data.update_playing_state(
                        &mut commands,
                        &theme,
                        diagram_entity,
                        playing.value.state,
                    );
                }
            }
        }
    }
}
