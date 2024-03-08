use std::sync::Arc;

use crate::{prelude::*, tab::tab_asset::TabError};
use edger_bevy::bevy_prelude::*;
use notation_model::prelude::*;

#[derive(Resource)]
pub struct NotationState {
    pub tab_path: String,
    pub tab: Option<Arc<Tab>>,
    pub bars_range: Option<(usize, usize)>,
    pub show_control: bool,
    pub show_kb: bool,
    pub preset: Option<String>,
    pub tab_error: Option<TabError>,
    pub debug_str: Option<String>,
    pub _despawn_delay_seconds: f32,
    pub _load_tab_delay_seconds: f32,
}

impl NotationState {
    pub fn new(tab_path: String) -> Self {
        Self {
            tab_path,
            tab: None,
            bars_range: None,
            show_control: false,

            #[cfg(debug_assertions)]
            show_kb: false,

            #[cfg(not(debug_assertions))]
            show_kb: true,

            preset: None,
            tab_error: None,
            debug_str: None,
            _despawn_delay_seconds: 0.0,
            _load_tab_delay_seconds: 0.0,
        }
    }
    pub fn change_tab(&mut self, theme: &mut NotationTheme, tab_path: String) {
        theme._bypass_systems = true;
        self.tab_path = tab_path;
        self.bars_range = None;
        self.reload_tab()
    }
    pub fn reload_tab(&mut self) {
        self.tab = None;
        self.tab_error = None;
        self._despawn_delay_seconds = 0.1;
        self._load_tab_delay_seconds = 0.2;
    }
    pub fn calc_bar_number(&self, add_ready_section: bool, bar_ordinal: usize) -> usize {
        if let Some((begin, _end)) = self.bars_range {
            if add_ready_section {
                if bar_ordinal > 0 {
                    return bar_ordinal + begin - 1;
                }
            } else {
                return bar_ordinal + begin + 1;
            }
        } else {
            if !add_ready_section {
                return bar_ordinal + 1;
            }
        }
        bar_ordinal
    }
}

impl FromWorld for NotationState {
    fn from_world(world: &mut World) -> Self {
        let args = world.get_resource::<NotationArgs>().unwrap();
        Self::new(args.tab.first().unwrap_or(&"".to_owned()).clone())
    }
}
