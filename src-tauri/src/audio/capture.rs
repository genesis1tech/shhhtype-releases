use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::Mutex;
use std::sync::Arc;

/// Push f32 samples into the buffer, averaging channels to mono.
fn push_mono_f32(buffer: &Arc<Mutex<Vec<f32>>>, data: &[f32], channels: usize) {
    let mut buf = buffer.lock();
    if channels == 1 {
        buf.extend_from_slice(data);
    } else {
        for chunk in data.chunks(channels) {
            let mono: f32 = chunk.iter().sum::<f32>() / channels as f32;
            buf.push(mono);
        }
    }
}

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

    /// Start capturing audio from the specified or default input device.
    /// Audio samples are normalized to mono f32 regardless of the device's native sample format.
    pub fn start(&mut self, buffer: Arc<Mutex<Vec<f32>>>, device_name: Option<&str>) -> Result<()> {
        let host = cpal::default_host();
        let device = if let Some(name) = device_name {
            host.input_devices()?
                .find(|d| d.name().ok().as_deref() == Some(name))
                .ok_or_else(|| anyhow::anyhow!("Input device '{}' not found, falling back to default", name))
                .or_else(|e| {
                    log::warn!("{}", e);
                    host.default_input_device()
                        .ok_or_else(|| anyhow::anyhow!("No input device available"))
                })?
        } else {
            host.default_input_device()
                .ok_or_else(|| anyhow::anyhow!("No input device available"))?
        };

        let config = device.default_input_config()?;
        self.sample_rate = config.sample_rate().0;
        let sample_format = config.sample_format();

        log::info!(
            "Audio capture: device={}, sample_rate={}, channels={}, format={:?}",
            device.name().unwrap_or_default(),
            self.sample_rate,
            config.channels(),
            sample_format
        );

        let channels = config.channels() as usize;
        let stream_config: cpal::StreamConfig = config.into();

        let err_callback = |err: cpal::StreamError| {
            log::error!("Audio capture error: {}", err);
        };

        // Build format-specific input streams that normalize to mono f32
        let stream = match sample_format {
            cpal::SampleFormat::F32 => {
                let buffer = buffer.clone();
                device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        push_mono_f32(&buffer, data, channels);
                    },
                    err_callback,
                    None,
                )?
            }
            cpal::SampleFormat::I16 => {
                let buffer = buffer.clone();
                device.build_input_stream(
                    &stream_config,
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer.lock();
                        for chunk in data.chunks(channels.max(1)) {
                            let sum: f32 = chunk.iter().map(|&s| s as f32 / i16::MAX as f32).sum();
                            buf.push(sum / channels as f32);
                        }
                    },
                    err_callback,
                    None,
                )?
            }
            cpal::SampleFormat::U16 => {
                let buffer = buffer.clone();
                device.build_input_stream(
                    &stream_config,
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer.lock();
                        for chunk in data.chunks(channels.max(1)) {
                            let sum: f32 = chunk
                                .iter()
                                .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                                .sum();
                            buf.push(sum / channels as f32);
                        }
                    },
                    err_callback,
                    None,
                )?
            }
            cpal::SampleFormat::I32 => {
                let buffer = buffer.clone();
                device.build_input_stream(
                    &stream_config,
                    move |data: &[i32], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer.lock();
                        for chunk in data.chunks(channels.max(1)) {
                            let sum: f32 = chunk.iter().map(|&s| s as f32 / i32::MAX as f32).sum();
                            buf.push(sum / channels as f32);
                        }
                    },
                    err_callback,
                    None,
                )?
            }
            format => {
                return Err(anyhow::anyhow!(
                    "Unsupported audio sample format: {:?}. Please select a different input device.",
                    format
                ));
            }
        };

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
