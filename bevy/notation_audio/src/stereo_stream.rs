use bevy::prelude::*;
use bevy::audio::{Source, AddAudioSource};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use bevy::utils::syncunsafecell::SyncUnsafeCell;
use std::sync::Arc;

pub struct StereoStreamDecoder {
    buffer: Arc<SyncUnsafeCell<AllocRingBuffer<[f32; 2]>>>,
    data: Option<f32>,
}

impl std::fmt::Debug for StereoStreamDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        write!(f, "<StereoStreamDecoder>({}/{})", buffer.len(), buffer.capacity())
    }
}

impl Iterator for StereoStreamDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(data) = self.data {
            self.data = None;
            return Some(data);
        }
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        if buffer.is_full() {
            let capacity = buffer.capacity();
            println!("<StereoStreamDecoder>[{}] buffer is full", capacity);
            buffer.clear();
        }
        let data = buffer.dequeue().unwrap_or([0.0, 0.0]);
        self.data = Some(data[1]);
        Some(data[0])
    }
}

impl Source for StereoStreamDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        44_100
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

#[derive(Asset, TypePath)]
pub struct StereoStream {
    pub volume: f32,
    buffer: Arc<SyncUnsafeCell<AllocRingBuffer<[f32; 2]>>>,
}

impl std::fmt::Debug for StereoStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        write!(f, "<StereoStream>({}/{}, v:{})", buffer.len(), buffer.capacity(), self.volume)
    }
}

impl Decodable for StereoStream {
    type DecoderItem = <StereoStreamDecoder as Iterator>::Item;
    type Decoder = StereoStreamDecoder;

    fn decoder(&self) -> Self::Decoder {
        StereoStreamDecoder {
            buffer: self.buffer.clone(),
            data: None,
        }
    }
}


impl StereoStream {
    pub const FRAME_STEP: f64 = 1.0 / 44_100.0;

    pub const DEFAULT_CAPACITY: usize = 4096;
    pub const DEFAULT_VOLUME: f32 = 1.0;

    pub fn remaining(&self) -> usize {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        buffer.capacity() - buffer.len()
    }

    pub fn push(&mut self, left: f32, right: f32) {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        buffer.push([left * self.volume, right * self.volume]);
    }

    pub fn push_batch(&mut self, volume_factor: f32, left: &[f32], right: &[f32]) {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        for i in 0..left.len() {
            buffer.push([left[i] * self.volume * volume_factor, right[i] * self.volume * volume_factor]);
        }
    }

    pub fn init_streaming(
        app: &mut App,
        setup_default_streaming: bool,
    ) {
        app.add_audio_source::<StereoStream>();
        if setup_default_streaming {
            app.add_systems(Startup, Self::setup_default_streaming);
        }
    }

    pub fn setup_streaming(
        commands: &mut Commands,
        assets: &mut ResMut<Assets<StereoStream>>,
        capacity: usize,
        volume: f32,
    ) {
        let buffer = AllocRingBuffer::<[f32; 2]>::new(capacity);
        let stream = StereoStream {
            volume,
            buffer: Arc::new(SyncUnsafeCell::new(buffer)),
        };
        let stream_handle = assets.add(stream);
        commands.spawn(AudioSourceBundle {
            source: stream_handle,
            ..default()
        });
    }

    pub fn setup_default_streaming(
        mut commands: Commands,
        mut assets: ResMut<Assets<StereoStream>>,
    ) {
        Self::setup_streaming(&mut commands, &mut assets, Self::DEFAULT_CAPACITY, Self::DEFAULT_VOLUME);
    }
}