use std::{fs::File, path::Path};

use anyhow::Result;
use mpris_server::{PlaybackRate, PlaybackStatus, Volume};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

#[cfg(test)]
pub mod tests;

pub mod mpris;

pub struct Player {
    sink: Sink,
    stream_handle: OutputStream,
}

impl Player {
    pub fn new() -> Result<Self> {
        let stream_handle = OutputStreamBuilder::open_default_stream()?;
        Ok(Player {
            sink: Sink::connect_new(stream_handle.mixer()),
            stream_handle: stream_handle,
        })
    }

    pub fn append(&self, audio_path: &Path) -> Result<()> {
        let source = Decoder::try_from(File::open(audio_path)?)?;
        self.sink.append(source);

        Ok(())
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn volume(&self) -> Volume {
        self.sink.volume() as Volume
    }

    pub fn set_volume(&self, value: Volume) {
        self.sink.set_volume(value as f32);
    }

    pub fn playback_rate(&self) -> PlaybackRate {
        self.sink.speed() as PlaybackRate
    }

    pub fn playback_status(&self) -> PlaybackStatus {
        if self.sink.empty() {
            PlaybackStatus::Stopped
        } else if self.sink.is_paused() {
            PlaybackStatus::Paused
        } else {
            PlaybackStatus::Playing
        }
    }
}
