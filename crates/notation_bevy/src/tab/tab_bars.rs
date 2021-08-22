use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{BevyUtil, View, ViewAddedQuery};
use notation_model::prelude::Tab;

use crate::prelude::{
    AddEntryEvent, BarBundle, BarLayout, BarPlugin, NotationAppState, NotationSettings,
    NotationTheme, PlayPlugin,
};
use crate::ui::layout::NotationLayout;

pub struct TabBars {
    pub tab: Arc<Tab>,
    pub bar_layouts: Arc<Vec<BarLayout>>,
}
impl Display for TabBars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabBars>({})", self.tab.bars.len())
    }
}
impl TabBars {
    pub fn new(tab: Arc<Tab>, bar_layouts: Arc<Vec<BarLayout>>) -> Self {
        Self { tab, bar_layouts }
    }
}
impl<'a> View<NotationLayout<'a>> for TabBars {}

impl TabBars {
    pub fn on_added(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        query: ViewAddedQuery<TabBars>,
        mut add_entry_evts: EventWriter<AddEntryEvent>,
    ) {
        for (_parent, entity, view) in query.iter() {
            let bar_layouts = view.bar_layouts.clone();
            PlayPlugin::spawn_indicators(
                &mut commands,
                &theme,
                entity,
                &view.tab,
                view.bar_layouts.get(0),
            );
            let bar_bundles: Vec<(&BarLayout, BarBundle)> = view
                .tab
                .bars
                .iter()
                .enumerate()
                .filter_map(|(index, bar)| {
                    bar_layouts.get(index).map(|bar_layout| {
                        let transform = theme.grid.calc_bar_transform(&bar_layout);
                        (
                            bar_layout,
                            BarBundle::new(bar.clone(), bar_layout.clone(), transform),
                        )
                    })
                })
                .collect();
            for (bar_layout, bar_bundle) in bar_bundles.into_iter() {
                let bar = bar_bundle.bar.clone();
                let bar_entity = BevyUtil::spawn_child_bundle(&mut commands, entity, bar_bundle);
                BarPlugin::create_lanes(
                    &mut commands,
                    &state,
                    &settings,
                    &theme,
                    bar_entity,
                    bar,
                    &bar_layout,
                    &mut add_entry_evts,
                );
            }
        }
    }
}
