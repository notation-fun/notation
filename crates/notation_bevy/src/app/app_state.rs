use std::sync::Arc;

use crate::prelude::*;
use bevy::prelude::*;
use notation_model::prelude::*;

pub struct TabPathes(pub Vec<String>);

pub struct NotationAppState {
    pub window_width: f32,
    pub window_height: f32,
    pub scale_factor_override: Option<f64>,
    pub tab_path: String,
    pub tab_asset: Handle<TabAsset>,
    pub tab: Option<Arc<Tab>>,
    pub hide_control: bool,
    pub parse_error: Option<ParseError>,
    pub debug_str: Option<String>,
    pub _despawn_delay_seconds: f32,
    pub _load_tab_delay_seconds: f32,
}

impl NotationAppState {
    pub fn new(asset_server: &AssetServer, tab_path: String) -> Self {
        let tab_asset = asset_server.load(tab_path.as_str());
        Self {
            window_width: 0.0,
            window_height: 0.0,
            scale_factor_override: None,
            tab_path,
            tab_asset,
            tab: None,
            hide_control: true,
            parse_error: None,
            debug_str: None,
            _despawn_delay_seconds: 0.0,
            _load_tab_delay_seconds: 0.0,
        }
    }
    pub fn change_tab(&mut self, asset_server: &AssetServer, tab_path: String) {
        self.tab_path = tab_path;
        self.tab_asset = asset_server.load(self.tab_path.as_str());
        self.parse_error = None;
        self.reset_tab()
    }
    pub fn reset_tab(&mut self) {
        self.tab = None;
        self._despawn_delay_seconds = 0.1;
        self._load_tab_delay_seconds = 0.2;
    }
    pub fn convert_pos(&self, pos: Vec2) -> Vec2 {
        Vec2::new(
            pos.x - self.window_width / 2.0,
            pos.y - self.window_height / 2.0,
        )
    }
}

impl FromWorld for NotationAppState {
    fn from_world(world: &mut World) -> Self {
        let server = world.get_resource::<AssetServer>().unwrap();
        let tab_pathes = world.get_resource::<TabPathes>().unwrap();
        Self::new(server, tab_pathes.0[0].clone())
    }
}
