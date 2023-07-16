use std::f64::consts::{PI, FRAC_PI_2};

use tab_viewer::edger_bevy_app::bevy_prelude::*;
use tab_viewer::edger_bevy_app::egui::{self, *};
use tab_viewer::edger_bevy_app::egui::plot::*;
use tab_viewer::prelude::{AudioConsts, StereoStream};

use tab_viewer::kb::markdown_page::MarkDownPage;
use tab_viewer::prelude::{NotationState, NotationAssets, NotationTheme, MarkDownAsset, KbPage, KbContent, EasyLinkEvent, Syllable, PageHelper, color_to_hsva};

#[derive(Copy, Clone, Debug)]
pub enum SoundSection {
    SingleString(SingleStringData),
}

#[derive(Copy, Clone, Debug)]
pub struct SingleStringData {
    pub t: f64,
    pub mute: bool,
    pub strengths: [f64; 10],
    pub time: f64,
    pub size: f64,
    pub speed: f64,
    pub separate_mode: bool,
}
impl SingleStringData {
    pub const MAX_SEGMENTS: usize = 10;
    pub const AUDIO_STRENGTH_FACTOR: f64 = 0.25;
    pub const PLOT_STRENGTH_FACTOR: f64 = 0.25;

    pub fn audio_strength(&self, segments: usize) -> f64 {
        if segments > self.strengths.len() {
            return 0.0;
        }
        self.strengths[segments - 1] * Self::AUDIO_STRENGTH_FACTOR
    }
    pub fn plot_strength(&self, segments: usize) -> f64 {
        if segments > self.strengths.len() {
            return 0.0;
        }
        self.strengths[segments - 1] * Self::PLOT_STRENGTH_FACTOR
    }
}

impl Default for SingleStringData {
    fn default() -> Self {
        Self {
            t: 0.0,
            mute: true,
            strengths: [
                1.0, 1.0 / 2.0, 1.0 / 3.0, 1.0 / 4.0, 1.0 / 5.0,
                1.0 / 6.0, 1.0 / 7.0, 1.0 / 8.0, 1.0 / 9.0, 1.0 / 10.0,
            ],
            time: 0.0,
            size: 1.0,
            speed: 0.5,
            separate_mode: false,
        }
    }
}


#[derive(Clone, Debug)]
pub struct SoundPage {
    pub path: String,
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
        MarkDownPage::markdown_ui(ui, texts, assets, state, theme, link_evts, self.path.as_str());
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
    pub fn new(path: String) -> Self {
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
        segments: usize,
        strength: f64,
        speed: f64,
        time: f64,
        x: f64,
    ) -> f64 {
        let x_offset = if segments % 2 == 0 {
            0.0
        } else {
            1.0 / segments as f64
        };
        let segments = segments as f64;
        let time = time * segments * PI * 2.0 * speed;
        strength * ((segments as f64) * FRAC_PI_2 * (x - x_offset)).sin() * time.sin()
    }
    /* https://en.wikipedia.org/wiki/String_vibration */
    fn harmonic_line(
        theme: &NotationTheme,
        data: &SingleStringData,
        segments: usize,
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
        let strength = data.plot_strength(segments);
        let speed = data.speed;
        let time = data.time;
        let size = data.size;
        Line::new(PlotPoints::from_explicit_callback(
            move |x| {
                size * (y_offset + Self::calc_harmonic_y(segments, strength, speed, time, x / size))
            }, -size..=size, 256,
        )).color(color_to_hsva(&theme.colors.of_option_syllable(syllable)))
        .name(format!("harmonic {}", segments))
    }
    fn tone_line(
        theme: &NotationTheme,
        data: &SingleStringData,
    ) -> Line {
        let frequency = data.speed;
        let time = data.time;
        let size = data.size;
        let strengths = data.strengths;
        Line::new(PlotPoints::from_explicit_callback(
            move |x| {
                let mut y = 0.0;
                for segments in 1..=SingleStringData::MAX_SEGMENTS {
                    let strength = strengths[segments - 1] * SingleStringData::PLOT_STRENGTH_FACTOR;
                    y += size * Self::calc_harmonic_y(segments, strength, frequency, time, x / size);
                }
                y
            }, -size..=size, 256,
        )).color(color_to_hsva(&theme.colors.of_syllable(Syllable::Do)))
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
        let delta = ui.input(|i| i.unstable_dt.min(1.0 / 60.0) as f64);
        data.time += delta;
        ui.ctx().request_repaint();
        let plot = Plot::new("single_string")
            .include_x(-data.size)
            .include_x(data.size)
            .include_y(0.0)
            .legend(Legend::default())
            .data_aspect(1.0);
        plot.show(ui, |plot_ui| {
            plot_ui.line(Self::tone_line(theme, data));
            for i in 1..=SingleStringData::MAX_SEGMENTS {
                plot_ui.line(Self::harmonic_line(theme, data, i));
            }
        });
    }
    pub fn single_string_settings(
        ui: &mut Ui,
        theme: &NotationTheme,
        data: &mut SingleStringData,
    ) {
        ui.add(Slider::new(&mut data.size, 0.25..=10.0).text("Size").logarithmic(true));
        ui.add(Slider::new(&mut data.speed, 0.1..=10.0).text("Speed").logarithmic(true));
        ui.separator();
        ui.checkbox(&mut data.separate_mode, "Separate Harmonics");
        ui.separator();
        ui.horizontal(|ui| {
            ui.checkbox(&mut data.mute, "mute");
            if ui.button("clear").clicked() {
                for i in 0..10 {
                    data.strengths[i] = 0.0;
                }
            }
            if ui.button("reset").clicked() {
                for i in 0..10 {
                    data.strengths[i] = 1.0 / (i as f64 + 1.0);
                }
            }
        });
        egui::Grid::new("notes").show(ui, |ui| {
            ui.label("strength");
            ui.label("harmonics");
            ui.label("guitar fret");
            ui.label("");
            ui.label("note");
            ui.label("math");
            ui.end_row();

            for (segments, fret, syllable, info) in vec![
                (1, "", Syllable::Do, "1"),
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
                ui.add(Slider::new(&mut data.strengths[segments - 1], 0.0..=1.0).show_value(false));
                ui.label(format!("{}", segments));
                if *segments > 1 {
                    if ui.button(format!("{}", fret)).clicked() {
                        for i in 0..10 {
                            if (i + 1) % *segments == 0 {
                                data.strengths[i] = 1.0 / (i as f64 + 1.0);
                            } else {
                                data.strengths[i] = 0.0;
                            }
                        }
                    }
                } else {
                    ui.label("");
                }
                PageHelper::add_syllable(ui, theme, true, syllable, false, *syllable != Syllable::Do);
                ui.label(format!("{}", info));
                ui.end_row();
            }
        });
        ui.separator();
    }
    pub fn audio(&mut self, stream: &mut StereoStream) {
        match self.section {
            SoundSection::SingleString(ref mut data) => {
                Self::single_string_audio(stream, data);
            },
        }
    }
    pub fn calc_harmonics_audio(
        t: f64,
        segments: usize,
        volume: f64,
    ) -> f64 {
        if volume <= 0.0 {
            return 0.0;
        }
        (t * 2.0 * PI * 220.0 * segments as f64).sin() * volume
    }
    pub fn single_string_audio(
        stream: &mut StereoStream,
        data: &mut SingleStringData,
    ) {
        if data.mute {
            return;
        }
        let step = AudioConsts::FRAME_STEP;
        let mut t = 0.0;
        loop {
            if stream.buffer.remaining() < 2 {
                break;
            }
            let mut total = 0.0;
            for segments in 1..=10 {
                let volume = data.audio_strength(segments);
                total += Self::calc_harmonics_audio(data.t + t, segments, volume);
            }
            stream.push(total as f32, total as f32);
            t += step;
        }
        data.t += t;
    }
}
