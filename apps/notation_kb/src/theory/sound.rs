use std::f64::consts::{PI, FRAC_PI_2};

use notation_bevy::bevy::prelude::*;
use notation_bevy::bevy_egui::egui::{self, *};
use notation_bevy::bevy_egui::egui::plot::*;

use notation_bevy::kb::markdown_page::MarkDownPage;
use notation_bevy::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbContent, EasyLinkEvent, BevyUtil, Syllable, PageHelper};

#[derive(Copy, Clone, Debug)]
pub enum SoundSection {
    SingleString(SingleStringData),
}

#[derive(Copy, Clone, Debug)]
pub struct SingleStringData {
    pub time: f64,
    pub size: f64,
    pub frequency: f64,
    pub max_segments: u8,
    pub separate_mode: bool,
    pub hide_base_note: bool,
    pub show_guitar_fret: bool,
}

impl Default for SingleStringData {
    fn default() -> Self {
        Self {
            time: 0.0,
            size: 1.0,
            frequency: 1.0,
            max_segments: 9,
            separate_mode: false,
            hide_base_note: false,
            show_guitar_fret: true,
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct SoundPage {
    pub path: &'static str,
    pub section: SoundSection,
}

impl KbPage for SoundPage {
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        MarkDownPage::markdown_ui(ui, texts, assets, state, theme, link_evts, self.path);
        ui.separator();
        match self.section {
            SoundSection::SingleString(ref mut data) => {
                Self::single_string_settings(ui, theme, data);
            },
        }
    }
}

impl KbContent for SoundPage {
    fn content_ui(
        &mut self,
        ui: &mut Ui,
        texts: &Assets<MarkDownAsset>,
        assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
        link_evts: &mut EventWriter<EasyLinkEvent>,
    ) {
        match self.section {
            SoundSection::SingleString(ref mut data) => {
                Self::single_string_ui(ui, texts, assets, state, theme, link_evts, data);
            },
        }
    }
}

impl SoundPage {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
            section: SoundSection::SingleString(Default::default()),
        }
    }
    pub fn content_visible(&self) -> bool {
        match self.section {
            SoundSection::SingleString(_) => true,
        }
    }
    /* https://en.wikipedia.org/wiki/Overtone */
    fn calc_harmonic_y(
        segments: u8,
        frequency: f64,
        time: f64,
        x: f64,
    ) -> f64 {
        let y_max = 0.25 / segments as f64;
        let x_offset = if segments % 2 == 0 {
            0.0
        } else {
            1.0 / segments as f64
        };
        let segments = segments as f64;
        let time = time * segments * PI * 2.0 * frequency;
        y_max * ((segments as f64) * FRAC_PI_2 * (x - x_offset)).sin() * time.sin()
    }
    /* https://en.wikipedia.org/wiki/String_vibration */
    fn harmonic_line(
        theme: &NotationTheme,
        data: &SingleStringData,
        segments: u8,
    ) -> Line {
        let y_offset = if data.separate_mode {
            match segments {
                1 => 0.5,
                2 => 0.9,
                3..=7 => 1.2 + 0.2 * (segments - 3) as f64,
                _ => 2.0 + (segments - 7) as f64 * 0.1,
            }
        } else {
            0.5
        };
        let mut segments_tone = segments;
        while segments_tone > 1 && segments_tone % 2 == 0 {
            segments_tone = segments_tone / 2;
        }
        let syllable = if segments_tone == 1 {
            Some(Syllable::Do)
        } else if segments_tone == 3 {
            Some(Syllable::So)
        } else if segments_tone == 5 {
            Some(Syllable::Mi)
        } else if segments_tone == 9 {
            Some(Syllable::Re)
        } else if segments_tone == 7 {
            Some(Syllable::Te)
        } else {
            None
        };
        let frequency = data.frequency;
        let time = data.time;
        let size = data.size;
        Line::new(Values::from_explicit_callback(
            move |x| {
                size * (y_offset + Self::calc_harmonic_y(segments, frequency, time, x / size))
            }, -size..=size, 256,
        )).color(BevyUtil::rgb_to_egui(&theme.colors.of_option_syllable(syllable)))
        .name(format!("harmonic {}", segments))
    }
    fn tone_line(
        theme: &NotationTheme,
        data: &SingleStringData,
    ) -> Line {
        let max_segments = data.max_segments;
        let frequency = data.frequency;
        let time = data.time;
        let size = data.size;
        Line::new(Values::from_explicit_callback(
            move |x| {
                let mut y = 0.0;
                for segments in 1..=max_segments {
                    y += size * Self::calc_harmonic_y(segments, frequency, time, x / size);
                }
                y
            }, -size..=size, 256,
        )).color(BevyUtil::rgb_to_egui(&theme.colors.of_syllable(Syllable::Do)))
        .name("tone")
    }
    pub fn single_string_ui(
        ui: &mut Ui,
        _texts: &Assets<MarkDownAsset>,
        _assets: &NotationAssets,
        _state: &NotationState,
        theme: &NotationTheme,
        _link_evts: &mut EventWriter<EasyLinkEvent>,
        data: &mut SingleStringData,
    ) {
        data.time += ui.input().unstable_dt.min(1.0 / 60.0) as f64;
        ui.ctx().request_repaint();
        let plot = Plot::new("single_string")
            .include_x(-data.size)
            .include_x(data.size)
            .include_y(0.0)
            .legend(Legend::default())
            .data_aspect(1.0);
        plot.show(ui, |plot_ui| {
            plot_ui.line(Self::tone_line(theme, data));
            for i in 1..=data.max_segments {
                plot_ui.line(Self::harmonic_line(theme, data, i));
            }
        });
    }
    pub fn single_string_settings(
        ui: &mut Ui,
        theme: &NotationTheme,
        data: &mut SingleStringData,
    ) {
        ui.add(Slider::new(&mut data.size, 0.5..=10.0).text("Size"));
        ui.add(Slider::new(&mut data.max_segments, 5..=30).text("Max Harmonics"));
        ui.add(Slider::new(&mut data.frequency, 0.1..=10.0).text("Frequency"));
        ui.separator();
        ui.checkbox(&mut data.separate_mode, "Separate Harmonics");
        ui.checkbox(&mut data.hide_base_note, "Hide Base Note");
        ui.checkbox(&mut data.show_guitar_fret, "Show Guitar Fret for Natural Harmonics");
        ui.separator();
        egui::Grid::new("notes").show(ui, |ui| {
            ui.label("harmonics");
            if data.show_guitar_fret {
                ui.label("fret");
            }
            ui.label("");
            ui.label("note");
            ui.label("math");
            ui.end_row();

            for (segments, fret, syllable, info) in vec![
                (2, "12", Syllable::Do, "2"),
                (3, "7", Syllable::So, "3/2 * 2"),
                (4, "5", Syllable::Do, "2 * 2"),
                (5, "4", Syllable::Mi, "5/4 * 2 * 2"),
                (6, "3.2", Syllable::So, "3/2 * 2 * 2"),
                (7, "2.7", Syllable::Te, "16/9 * 2 * 2 = 7.111"),
                (8, "2.2", Syllable::Do, "2 * 2 * 2"),
                (9, "2", Syllable::Re, "9/8 * 2 * 2 * 2"),
                (10, "1.8", Syllable::Mi, "5/4 * 2 * 2 * 2"),
            ].iter() {
                if data.hide_base_note && *syllable == Syllable::Do {
                    continue;
                }
                ui.label(format!("{}", segments));
                if data.show_guitar_fret {
                    ui.label(format!("{}", fret));
                }
                PageHelper::add_syllable(ui, theme, true, syllable, false, *syllable != Syllable::Do);
                ui.label(format!("{}", info));
                ui.end_row();
            }
        });
        ui.separator();
    }
}
