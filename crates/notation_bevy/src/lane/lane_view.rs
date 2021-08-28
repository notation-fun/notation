use bevy::prelude::*;

use crate::entry::entry_plugin;
use crate::lyrics::lyrics_plugin::LyricsPlugin;
use crate::prelude::{
    BevyUtil, LaneBundle, LaneLayoutData, MelodyPlugin, NotationAssets, NotationSettings,
    NotationTheme,
};
use crate::shapes::shapes_plugin::ShapesPlugin;
use crate::strings::strings_plugin::StringsPlugin;
use crate::ui::layout::NotationLayout;
use bevy_utils::prelude::{LayoutConstraint, LayoutSize, VBoxCell, View, ViewBundle};
use notation_model::prelude::{BarLane, LaneKind, TabBar};

pub type LaneView = LaneLayoutData;

impl<'a> View<NotationLayout<'a>> for LaneView {
    fn calc_size(&self, _engine: &NotationLayout, constraint: LayoutConstraint) -> LayoutSize {
        if self.visible() {
            LayoutSize::new(constraint.max.width, self.height + self.margin)
        } else {
            LayoutSize::ZERO
        }
    }
}
impl<'a> VBoxCell<NotationLayout<'a>> for LaneView {
    fn order(&self) -> usize {
        self.index
    }
}

impl LaneView {
    pub fn spawn(
        commands: &mut Commands,
        assets: &NotationAssets,
        theme: &NotationTheme,
        settings: &NotationSettings,
        bar_entity: Entity,
        _bar: &TabBar,
        lane_layout: &LaneLayoutData,
    ) {
        if let Some(lane) = &lane_layout.lane {
            let lane_bundle = LaneBundle::new(lane.clone(), lane_layout.clone());
            let lane_entity = BevyUtil::spawn_child_bundle(commands, bar_entity, lane_bundle);
            Self::setup_lane(commands, lane, lane_entity);
            for entry in lane.entries.iter() {
                entry_plugin::create_entry(commands, assets, theme, settings, lane_entity, entry);
            }
        } else {
            let view_bundle = ViewBundle::from(lane_layout.clone());
            BevyUtil::spawn_child_bundle(commands, bar_entity, view_bundle);
        }
    }
    pub fn setup_lane(commands: &mut Commands, lane: &BarLane, lane_entity: Entity) {
        match lane.kind {
            LaneKind::Lyrics => {
                LyricsPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
            }
            LaneKind::Melody => {
                MelodyPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
            }
            LaneKind::Strings => {
                StringsPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
            }
            LaneKind::Shapes => {
                ShapesPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
            }
            _ => (),
        }
    }
}
