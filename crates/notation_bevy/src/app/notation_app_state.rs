use std::sync::Arc;

use crate::prelude::*;
use bevy::prelude::*;
use notation_model::prelude::*;

pub struct TabPathes(pub Vec<String>);

pub struct NotationAppState {
    pub tab_path: String,
    pub tab_asset: Handle<TabAsset>,
    pub tab: Option<Arc<Tab>>,
    pub parse_error: Option<ParseError>,
    pub camera_panning: bool,
}

impl NotationAppState {
    pub fn new(asset_server: &AssetServer, tab_path: String, camera_panning: bool) -> Self {
        let tab_asset = asset_server.load(tab_path.as_str());
        Self {
            tab_path,
            tab_asset,
            tab: None,
            parse_error: None,
            camera_panning,
        }
    }
    pub fn change_tab(&mut self, asset_server: &AssetServer, tab_path: String) {
        self.tab_path = tab_path;
        self.tab_asset = asset_server.load(self.tab_path.as_str());
        self.tab = None;
        self.parse_error = None;
    }
}

impl FromWorld for NotationAppState {
    fn from_world(world: &mut World) -> Self {
        let server = world.get_resource::<AssetServer>().unwrap();
        let tab_pathes = world.get_resource::<TabPathes>().unwrap();
        Self::new(server, tab_pathes.0[0].clone(), false)
    }
}
