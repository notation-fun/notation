use bevy::prelude::*;

use crate::settings::layout_settings::LayoutMode;

use crate::prelude::{NotationState, NotationSettings, NotationTheme};

use super::events::WindowResizedEvent;

pub struct Control();

impl Control {
    pub const PRESET_GUITAR_TAB: &'static str = "guitar_tab";
    pub const PRESET_GUITAR_NOTES: &'static str = "guitar_notes";
    pub const PRESET_GUITAR_STRINGS: &'static str = "guitar_strings";
    pub const PRESET_MELODY: &'static str = "melody";
    pub const ALL_PRESETS: [&'static str ; 4 ] = [
        Self::PRESET_GUITAR_TAB,
        Self::PRESET_GUITAR_NOTES,
        Self::PRESET_GUITAR_STRINGS,
        Self::PRESET_MELODY,
    ];

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
    pub fn toggle_show_melody_pitch(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.show_melody_pitch = !settings.show_melody_pitch;
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
    fn set_preset_strings(
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
        always_show_fret: bool,
    ) {
        settings.hide_strings_lane = false;
        settings.always_show_fret = always_show_fret;
        theme.sizes.layout.page_margin = 24.0;
        theme.sizes.strings.string_space = 20.0;
        theme.sizes.strings.note_height = 9.0;
        theme.texts.strings.text_y = -4.0;
        theme.texts.strings.fret_font_size = 20.0;
    }
    fn set_preset_harmony(
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
    ) {
        settings.hide_harmony_lane = false;
        theme.sizes.layout.page_margin = 24.0;
        theme.sizes.harmony.note_height = 6.0;
        theme.sizes.harmony.semitone_height = 6.0;
        theme.texts.harmony.text_y = 9.0;
        theme.texts.harmony.syllable_font_size = 20.0;
    }
    fn set_preset_melody(
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
        show_melody_pitch: bool,
    ) {
        settings.hide_melody_lane = false;
        settings.show_melody_pitch = show_melody_pitch;
        settings.show_melody_syllable = true;
        settings.show_syllable_as_num = true;
        theme.sizes.layout.page_margin = 24.0;
        theme.sizes.melody.note_height = 9.0;
        theme.sizes.melody.semitone_height = 9.0;
        theme.texts.melody.text_y = -18.0;
        theme.texts.melody.syllable_font_size = 20.0;
    }
    pub fn set_preset(
        state: &mut NotationState,
        settings: &mut NotationSettings,
        theme: &mut NotationTheme,
        windows: &mut Windows,
        _window_resized_evts: &mut EventWriter<WindowResizedEvent>,
        preset: &'static str,
    ) {
        state.show_kb = false;
        state.preset = Some(preset.to_owned());
        #[cfg(not(target_arch = "wasm32"))]
        Self::set_primary_window_size(windows, 1080, 1920);
        match preset {
            Self::PRESET_GUITAR_TAB => {
                settings.hack_for_screenshot();
                Self::set_preset_strings(settings, theme, true);
                Self::set_preset_harmony(settings, theme);
                settings.hide_shapes_lane = false;
            },
            Self::PRESET_GUITAR_NOTES => {
                settings.hack_for_screenshot();
                Self::set_preset_harmony(settings, theme);
            },
            Self::PRESET_GUITAR_STRINGS => {
                settings.hack_for_screenshot();
                Self::set_preset_strings(settings, theme, true);
            },
            Self::PRESET_MELODY => {
                settings.hack_for_screenshot();
                Self::set_preset_melody(settings, theme, true);
            },
            _ => {
                println!("Control::set_preset() Invalid Preset: {}", preset);
            },
        }
        Self::reload_tab(state, theme);
    }
}
