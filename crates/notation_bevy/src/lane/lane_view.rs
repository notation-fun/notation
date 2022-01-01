use bevy::prelude::*;

use crate::entry::entry_plugin;
use crate::lyrics::lyrics_plugin::LyricsPlugin;
use crate::prelude::{
    BevyUtil, LaneBundle, LaneLayoutData, MelodyPlugin, NotationAssets, NotationSettings,
    NotationTheme, ThemeColors,
};
use crate::shapes::shapes_plugin::ShapesPlugin;
use crate::strings::strings_plugin::StringsPlugin;
use crate::prelude::NotationLayout;
use notation_bevy_utils::prelude::{
    ColorBackground, LayoutConstraint, LayoutSize, VBoxCell, View, ViewBundle,
};
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
        self.order()
    }
}

impl LaneView {
    pub const DEBUGGING_LANE_LAYOUT: bool = false;
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
            if Self::setup_lane(commands, settings, lane, lane_entity) {
                if Self::DEBUGGING_LANE_LAYOUT {
                    let color = match lane_layout.lane_kind {
                        LaneKind::Shapes => ThemeColors::hex_linear("FF000033"),
                        LaneKind::Strings => ThemeColors::hex_linear("00FF0033"),
                        LaneKind::Lyrics => ThemeColors::hex_linear("0000FF33"),
                        LaneKind::Melody => ThemeColors::hex_linear("00FFFF33"),
                        _ => ThemeColors::hex_linear("00000033"),
                    };
                    ColorBackground::spawn(commands, lane_entity, 30.0, color);
                }
                for entry in lane.entries.iter() {
                    entry_plugin::create_entry(
                        commands,
                        assets,
                        theme,
                        settings,
                        lane_entity,
                        entry,
                    );
                }
            }
        } else {
            let view_bundle = ViewBundle::from(lane_layout.clone());
            BevyUtil::spawn_child_bundle(commands, bar_entity, view_bundle);
        }
    }
    pub fn setup_lane(
        commands: &mut Commands,
        settings: &NotationSettings,
        lane: &BarLane,
        lane_entity: Entity,
    ) -> bool {
        match lane.kind {
            LaneKind::Lyrics => {
                if !settings.hide_lyrics_lane {
                    LyricsPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
                }
                !settings.hide_lyrics_lane
            }
            LaneKind::Melody => {
                if !settings.hide_melody_lane {
                    MelodyPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
                }
                !settings.hide_melody_lane
            }
            LaneKind::Strings => {
                if !settings.hide_strings_lane {
                    StringsPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
                }
                true
            }
            LaneKind::Shapes => {
                if !settings.hide_shapes_lane {
                    ShapesPlugin::insert_lane_extra(&mut commands.entity(lane_entity), lane)
                }
                true
            }
            _ => false,
        }
    }
}
