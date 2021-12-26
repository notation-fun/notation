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
    pub tab: Option<Arc<Tab>>,
    pub hide_control: bool,
    pub parse_error: Option<ParseError>,
    pub debug_str: Option<String>,
    pub _despawn_delay_seconds: f32,
    pub _load_tab_delay_seconds: f32,
}

impl NotationAppState {
    pub fn new(tab_path: String) -> Self {
        Self {
            window_width: 0.0,
            window_height: 0.0,
            scale_factor_override: None,
            tab_path,
            tab: None,
            hide_control: true,
            parse_error: None,
            debug_str: None,
            _despawn_delay_seconds: 0.0,
            _load_tab_delay_seconds: 0.0,
        }
    }
    pub fn change_tab(
        &mut self,
        theme: &mut NotationTheme,
        tab_path: String
    ) {
        theme._bypass_systems = true;
        self.tab_path = tab_path;
        self.parse_error = None;
        self.reload_tab()
    }
    pub fn reload_tab(&mut self) {
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
        let tab_pathes = world.get_resource::<TabPathes>().unwrap();
        Self::new(tab_pathes.0[0].clone())
    }
}
