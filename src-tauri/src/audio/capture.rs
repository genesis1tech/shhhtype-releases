use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct AudioCapture {
    stream: Option<cpal::Stream>,
    sample_rate: u32,
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            stream: None,
            sample_rate: 0,
        }
    }

    /// Start capturing audio from the default input device.
    /// Audio samples (f32) are pushed into the shared buffer.
    pub fn start(&mut self, buffer: Arc<Mutex<Vec<f32>>>) -> Result<()> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

        let config = device.default_input_config()?;
        self.sample_rate = config.sample_rate().0;

        log::info!(
            "Audio capture: device={}, sample_rate={}, channels={}",
            device.name().unwrap_or_default(),
            self.sample_rate,
            config.channels()
        );

        let channels = config.channels() as usize;
        let stream_config: cpal::StreamConfig = config.into();

        let stream = device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Convert to mono by averaging channels
                let mut buf = buffer.lock();
                if channels == 1 {
                    buf.extend_from_slice(data);
                } else {
                    for chunk in data.chunks(channels) {
                        let mono: f32 = chunk.iter().sum::<f32>() / channels as f32;
                        buf.push(mono);
                    }
                }
            },
            |err| {
                log::error!("Audio capture error: {}", err);
            },
            None,
        )?;

        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Stop audio capture.
    pub fn stop(&mut self) {
        self.stream = None;
        log::info!("Audio capture stopped");
    }

    /// Get the native sample rate of the input device.
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Check if currently capturing.
    pub fn is_active(&self) -> bool {
        self.stream.is_some()
    }

    /// List available input devices.
    pub fn list_devices() -> Vec<String> {
        let host = cpal::default_host();
        host.input_devices()
            .map(|devices| {
                devices
                    .filter_map(|d| d.name().ok())
                    .collect()
            })
            .unwrap_or_default()
    }
}
