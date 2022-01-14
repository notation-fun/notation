use bevy::prelude::*;

use crate::settings::layout_settings::LayoutMode;

use crate::prelude::{NotationState, NotationSettings, NotationTheme};

use super::events::WindowResizedEvent;

pub struct Control();

impl Control {
    pub const PRESET_GUITAR_STRINGS: &'static str = "guitar_strings";

    pub fn reload_tab(state: &mut NotationState, theme: &mut NotationTheme) {
        state.reload_tab();
        theme._bypass_systems = true;
    }
    pub fn toggle_layout_mode(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        if settings.layout.mode == LayoutMode::Grid {
            settings.layout.mode = LayoutMode::Line;
        } else {
            settings.layout.mode = LayoutMode::Grid;
        }
        Self::reload_tab(state, theme);
    }
    pub fn toggle_hide_guitar_view(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.hide_guitar_view = !settings.hide_guitar_view;
        Self::reload_tab(state, theme);
    }
    pub fn toggle_hide_chords_view(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.hide_chords_view = !settings.hide_chords_view;
        Self::reload_tab(state, theme);
    }
    pub fn toggle_hide_mini_map(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.hide_mini_map = !settings.hide_mini_map;
        Self::reload_tab(state, theme);
    }
    pub fn toggle_show_guitar_syllable(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.show_guitar_syllable = !settings.show_guitar_syllable;
        Self::reload_tab(state, theme);
    }
    pub fn toggle_show_melody_syllable(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.show_melody_syllable = !settings.show_melody_syllable;
        Self::reload_tab(state, theme);
    }
    pub fn toggle_show_syllable_as_pitch(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.show_syllable_as_pitch = !settings.show_syllable_as_pitch;
        Self::reload_tab(state, theme);
    }
    pub fn toggle_always_show_fret(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.always_show_fret = !settings.always_show_fret;
        Self::reload_tab(state, theme);
    }
    pub fn set_window_size(window: &mut Window, width: usize, height: usize) {
        /* Bevy is using the requested width and height for a check, so if the window got resized after
         * set_resolution(), set same value won't trigger update, use a quick hack here for now.
         */
        if window.requested_width() == width as f32 && window.requested_height() == height as f32 {
            window.set_resolution(width as f32, (height / 2) as f32);
        }
        window.set_resolution(width as f32, height as f32);
    }
    pub fn set_primary_window_size(windows: &mut Windows, width: usize, height: usize) {
        if let Some(window) = windows.get_primary_mut() {
            Self::set_window_size(window, width, height);
        }
    }
    pub fn set_preset(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
        windows: &mut Windows,
        _window_resized_evts: &mut EventWriter<WindowResizedEvent>,
        preset: &'static str,
    ) {
        match preset {
            Self::PRESET_GUITAR_STRINGS => {
                state.preset = Some(preset.to_owned());
                settings.add_ready_section = false;
                settings.hide_indicators = true;
                settings.hide_guitar_view = true;
                settings.hide_chords_view = true;
                settings.hide_shapes_lane = true;
                settings.always_show_fret = true;
                settings.override_beat_size = Some(128.0);
                theme.sizes.strings.string_space = 20.0;
                theme.sizes.strings.note_height = 9.0;
                theme.texts.strings.fret_font_size = 20.0;
                theme.texts.strings.text_y = 8.0;
                theme.sizes.layout.page_margin = 24.0;
                #[cfg(not(target_arch = "wasm32"))]
                Self::set_primary_window_size(windows, 1280, 1920);
                Self::reload_tab(state, theme);
            },
            _ => {},
        }
    }
}
