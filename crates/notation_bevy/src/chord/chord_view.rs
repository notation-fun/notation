use std::sync::Arc;
use std::fmt::Display;

use bevy::prelude::*;

use bevy_utils::prelude::{
    BevyUtil, GridCell, LayoutAnchor, LayoutChangedWithChildrenQuery, View, ViewBundle,
};
use notation_model::prelude::{Chord, ModelEntry, Position, Tab, TabBar};

use crate::prelude::{NotationTheme};
use crate::ui::layout::NotationLayout;

use super::chord_base::ChordBaseData;
use super::chord_diagram::{ChordDiagram, ChordDiagramData};
use super::chord_interval::ChordIntervalData;
use super::chord_playing::ChordPlaying;
use super::interval_dot::IntervalDotData;

pub struct ChordView {
    pub entry: Arc<ModelEntry>,
    pub chord: Chord,
}

impl Display for ChordView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ChordView>({})", self.chord)
    }
}

impl ChordView {
    pub fn search_in_bars(&self, tab: &Arc<Tab>, begin_bar_ordinal: usize, end_bar_ordinal: usize) -> Option<Arc<TabBar>> {
        for bar_ordinal in begin_bar_ordinal..=end_bar_ordinal {
            if let Some(bar) = tab.get_bar_of_ordinal(bar_ordinal) {
                if let Some(chord) = bar.get_chord(None) {
                    if chord == self.chord {
                        return Some(bar);
                    }
                }
            }
        }
        None
    }
    pub fn search_next(&self, pass_end: bool, position: Option<Position>) -> Option<Arc<TabBar>> {
        if let Some(tab) = self.entry.tab() {
            let last_bar_ordinal = tab.bars.len() + 1;
            match position {
                Some(pos) => {
                    let bar_ordinal = pos.bar.bar_ordinal;
                    if let Some(entry) = self.search_in_bars(&tab, bar_ordinal + 1, last_bar_ordinal) {
                        return Some(entry);
                    } else if pass_end {
                        return self.search_in_bars(&tab, 1, bar_ordinal);
                    }
                },
                None => {
                    return self.search_in_bars(&tab, 1, last_bar_ordinal);
                },
            }
        }
        None
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
    ) {
        for (_entity, _view, layout, children) in query.iter() {
            let radius = layout.size.width * theme.sizes.chord.diagram_factor;
            for child in children.iter() {
                if let Ok((diagram_entity, mut diagram_data, diagram_children)) =
                    diagram_query.get_mut(*child)
                {
                    ChordDiagram::update_size(
                        &mut commands,
                        &theme,
                        &mut interval_query,
                        &mut base_query,
                        &mut dot_query,
                        diagram_entity,
                        &mut diagram_data,
                        diagram_children,
                        radius,
                    );
                }
            }
        }
    }
    pub fn spawn(
        commands: &mut Commands,
        theme: &NotationTheme,
        entity: Entity,
        chord: Chord,
        entry: &Arc<ModelEntry>,
    ) -> Entity {
        let chord_entity = BevyUtil::spawn_child_bundle(
            commands,
            entity,
            ViewBundle::from(ChordView{entry: entry.clone(), chord}),
        );
        //TODO: handle initialization in a nicer way.
        let radius = 0.0;
        ChordDiagram::spawn(commands, theme, chord_entity, entry.props, chord, radius);
        commands
            .entity(chord_entity)
            .insert(ChordPlaying::from((entry.props, chord)));
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
        for (_entity, playing, _view, children) in query.iter_mut() {
            for child in children.iter() {
                if let Ok((diagram_entity, mut diagram_data)) = diagram_query.get_mut(*child) {
                    ChordDiagram::update_playing_state(
                        &mut commands,
                        &theme,
                        diagram_entity,
                        &mut diagram_data,
                        playing.value.state,
                    );
                }
            }
        }
    }
}
