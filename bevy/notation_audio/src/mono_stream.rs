use bevy::prelude::*;
use bevy::audio::{Source, AddAudioSource};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use bevy::utils::syncunsafecell::SyncUnsafeCell;
use std::sync::Arc;

pub struct MonoStreamDecoder {
    buffer: Arc<SyncUnsafeCell<AllocRingBuffer<f32>>>,
}

impl std::fmt::Debug for MonoStreamDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        write!(f, "<MonoStreamDecoder>({}/{})", buffer.len(), buffer.capacity())
    }
}

impl Iterator for MonoStreamDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        if buffer.is_full() {
            let capacity = buffer.capacity();
            println!("<MonoStreamDecoder>[{}] buffer is full", capacity);
        }
        let data = buffer.dequeue().unwrap_or(0.0);
        Some(data)
    }
}

impl Source for MonoStreamDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44_100
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

#[derive(Asset, TypePath)]
pub struct MonoStream {
    pub volume: f32,
    buffer: Arc<SyncUnsafeCell<AllocRingBuffer<f32>>>,
}

impl std::fmt::Debug for MonoStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        write!(f, "<MonoStream>({}/{}, v:{})", buffer.len(), buffer.capacity(), self.volume)
    }
}

impl Decodable for MonoStream {
    type DecoderItem = <MonoStreamDecoder as Iterator>::Item;
    type Decoder = MonoStreamDecoder;

    fn decoder(&self) -> Self::Decoder {
        MonoStreamDecoder {
            buffer: self.buffer.clone(),
        }
    }
}

impl MonoStream {
    pub const FRAME_STEP: f64 = 1.0 / 44_100.0;

    pub const DEFAULT_CAPACITY: usize = 4096;
    pub const DEFAULT_VOLUME: f32 = 1.0;

    pub fn remaining(&self) -> usize {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        buffer.capacity() - buffer.len()
    }

    pub fn push(&mut self, data: f32) {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        buffer.push(data * self.volume);
    }

    pub fn push_batch(&mut self, volume_factor: f32, data: &[f32]) {
        let buffer = unsafe {
            self.buffer.as_ref().get().as_mut().unwrap()
        };
        for i in 0..data.len() {
            buffer.push(data[i] * self.volume * volume_factor);
        }
    }

    pub fn init_streaming(
        app: &mut App,
        setup_default_streaming: bool,
    ) {
        app.add_audio_source::<MonoStream>();
        if setup_default_streaming {
            app.add_systems(Startup, Self::setup_default_streaming);
        }
    }
    pub fn setup_streaming(
        commands: &mut Commands,
        assets: &mut ResMut<Assets<MonoStream>>,
        capacity: usize,
        volume: f32,
    ) {
        let buffer = AllocRingBuffer::<f32>::new(capacity);
        let stream = MonoStream {
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
        mut assets: ResMut<Assets<MonoStream>>,
    ) {
        Self::setup_streaming(&mut commands, &mut assets, Self::DEFAULT_CAPACITY, Self::DEFAULT_VOLUME);
    }
}