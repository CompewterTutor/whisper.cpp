use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SampleFormat, Stream, StreamConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Audio device information for frontend display
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub is_default: bool,
    pub channels: u16,
    pub sample_rate: u32,
}

/// Capture session state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureState {
    Idle,
    Listening,
    Transcribing,
    Error,
}

/// Audio capture session
pub struct AudioCaptureSession {
    host: Host,
    device: Option<Device>,
    stream: Option<Stream>,
    state: Arc<std::sync::Mutex<CaptureState>>,
    samples: Arc<std::sync::Mutex<Vec<f32>>>,
    is_capturing: Arc<AtomicBool>,
    sample_count: Arc<AtomicUsize>,
}

impl AudioCaptureSession {
    pub fn new() -> Result<Self, AudioError> {
        let host = cpal::default_host();

        Ok(Self {
            host,
            device: None,
            stream: None,
            state: Arc::new(std::sync::Mutex::new(CaptureState::Idle)),
            samples: Arc::new(std::sync::Mutex::new(Vec::new())),
            is_capturing: Arc::new(AtomicBool::new(false)),
            sample_count: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// List available input devices
    pub fn list_input_devices(&self) -> Result<Vec<AudioDeviceInfo>, AudioError> {
        let default_device = self.host.default_input_device();
        let default_name = default_device.as_ref().and_then(|d| d.name().ok());

        let devices: Vec<AudioDeviceInfo> = self
            .host
            .input_devices()?
            .filter_map(|device| {
                let name = device.name().ok()?;
                let config = device.default_input_config().ok()?;
                let is_default = default_name.as_ref() == Some(&name);

                Some(AudioDeviceInfo {
                    name,
                    is_default,
                    channels: config.channels(),
                    sample_rate: config.sample_rate().0,
                })
            })
            .collect();

        Ok(devices)
    }

    /// Select a device by name (or default if None)
    pub fn select_device(&mut self, name: Option<&str>) -> Result<(), AudioError> {
        self.device = if let Some(device_name) = name {
            self.host
                .input_devices()?
                .find(|d| d.name().map(|n| n == device_name).unwrap_or(false))
        } else {
            self.host.default_input_device()
        };

        if self.device.is_none() {
            return Err(AudioError::DeviceNotFound);
        }

        Ok(())
    }

    /// Get current state
    pub fn state(&self) -> CaptureState {
        self.state.lock().unwrap().clone()
    }

    /// Start capturing audio
    pub fn start_capture(&mut self) -> Result<(), AudioError> {
        let device = self.device.as_ref().ok_or(AudioError::NoDeviceSelected)?;
        let supported_config = device.default_input_config()?;
        let config: StreamConfig = supported_config.clone().into();

        // Clear previous samples
        self.samples.lock().unwrap().clear();
        self.sample_count.store(0, Ordering::SeqCst);

        let samples = Arc::clone(&self.samples);
        let state = Arc::clone(&self.state);
        let is_capturing = Arc::clone(&self.is_capturing);
        let sample_count = Arc::clone(&self.sample_count);

        is_capturing.store(true, Ordering::SeqCst);
        *state.lock().unwrap() = CaptureState::Listening;

        let stream = match supported_config.sample_format() {
            SampleFormat::F32 => {
                self.build_stream::<f32>(device, &config, samples, is_capturing, sample_count)?
            }
            SampleFormat::I16 => {
                self.build_stream::<i16>(device, &config, samples, is_capturing, sample_count)?
            }
            SampleFormat::U16 => {
                self.build_stream::<u16>(device, &config, samples, is_capturing, sample_count)?
            }
            _ => return Err(AudioError::UnsupportedFormat),
        };

        stream.play()?;
        self.stream = Some(stream);

        Ok(())
    }

    fn build_stream<T>(
        &self,
        device: &Device,
        config: &StreamConfig,
        samples: Arc<std::sync::Mutex<Vec<f32>>>,
        is_capturing: Arc<AtomicBool>,
        sample_count: Arc<AtomicUsize>,
    ) -> Result<Stream, AudioError>
    where
        T: cpal::Sample + cpal::SizedSample,
        f32: std::convert::From<T>,
    {
        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("audio stream error: {}", err);

        let stream = device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                if !is_capturing.load(Ordering::SeqCst) {
                    return;
                }

                // Convert to f32 and store mono (take first channel)
                let mut buffer = samples.lock().unwrap();
                for chunk in data.chunks(channels) {
                    if let Some(&sample) = chunk.first() {
                        buffer.push(f32::from(sample));
                    }
                }
                sample_count.fetch_add(data.len(), Ordering::SeqCst);
            },
            err_fn,
            None,
        )?;

        Ok(stream)
    }

    /// Stop capturing and return the samples
    pub fn stop_capture(&mut self) -> Result<Vec<f32>, AudioError> {
        self.is_capturing.store(false, Ordering::SeqCst);
        self.stream = None;
        *self.state.lock().unwrap() = CaptureState::Idle;

        let samples = self.samples.lock().unwrap().clone();
        Ok(samples)
    }

    /// Set state (for external state management)
    pub fn set_state(&self, new_state: CaptureState) {
        *self.state.lock().unwrap() = new_state;
    }

    /// Get current device info
    pub fn current_device_info(&self) -> Option<AudioDeviceInfo> {
        let device = self.device.as_ref()?;
        let name = device.name().ok()?;
        let config = device.default_input_config().ok()?;

        Some(AudioDeviceInfo {
            name,
            is_default: false, // We don't know after selection
            channels: config.channels(),
            sample_rate: config.sample_rate().0,
        })
    }

    /// Get sample count
    pub fn sample_count(&self) -> usize {
        self.sample_count.load(Ordering::SeqCst)
    }
}

impl Default for AudioCaptureSession {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            host: cpal::default_host(),
            device: None,
            stream: None,
            state: Arc::new(std::sync::Mutex::new(CaptureState::Idle)),
            samples: Arc::new(std::sync::Mutex::new(Vec::new())),
            is_capturing: Arc::new(AtomicBool::new(false)),
            sample_count: Arc::new(AtomicUsize::new(0)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioError {
    DeviceNotFound,
    NoDeviceSelected,
    UnsupportedFormat,
    StreamError,
    IoError,
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::DeviceNotFound => write!(f, "Audio device not found"),
            AudioError::NoDeviceSelected => write!(f, "No audio device selected"),
            AudioError::UnsupportedFormat => write!(f, "Unsupported audio format"),
            AudioError::StreamError => write!(f, "Audio stream error"),
            AudioError::IoError => write!(f, "I/O error"),
        }
    }
}

impl std::error::Error for AudioError {}

impl From<cpal::DevicesError> for AudioError {
    fn from(_: cpal::DevicesError) -> Self {
        AudioError::DeviceNotFound
    }
}

impl From<cpal::DefaultStreamConfigError> for AudioError {
    fn from(_: cpal::DefaultStreamConfigError) -> Self {
        AudioError::UnsupportedFormat
    }
}

impl From<cpal::BuildStreamError> for AudioError {
    fn from(_: cpal::BuildStreamError) -> Self {
        AudioError::StreamError
    }
}

impl From<cpal::PlayStreamError> for AudioError {
    fn from(_: cpal::PlayStreamError) -> Self {
        AudioError::StreamError
    }
}

/// Write samples to a WAV file at 16kHz mono
pub fn write_wav_file(
    samples: &[f32],
    sample_rate: u32,
    path: &std::path::Path,
) -> Result<(), AudioError> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec).map_err(|_| AudioError::IoError)?;

    for &sample in samples {
        // Convert f32 [-1.0, 1.0] to i16 [-32768, 32767]
        let value = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
        writer
            .write_sample(value)
            .map_err(|_| AudioError::IoError)?;
    }

    writer.finalize().map_err(|_| AudioError::IoError)?;
    Ok(())
}

/// Resample audio to target sample rate (simple linear interpolation)
pub fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate {
        return samples.to_vec();
    }

    let ratio = from_rate as f64 / to_rate as f64;
    let output_len = (samples.len() as f64 / ratio) as usize;
    let mut output = Vec::with_capacity(output_len);

    for i in 0..output_len {
        let src_idx = i as f64 * ratio;
        let idx0 = src_idx.floor() as usize;
        let idx1 = (idx0 + 1).min(samples.len() - 1);
        let frac = src_idx - idx0 as f64;

        if idx0 < samples.len() {
            let sample = samples[idx0] * (1.0 - frac) as f32 + samples[idx1] * frac as f32;
            output.push(sample);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn resample_preserves_length_ratio() {
        let samples = vec![0.5; 48000]; // 1 second at 48kHz
        let resampled = resample(&samples, 48000, 16000);
        assert_eq!(resampled.len(), 16000); // Should be 1 second at 16kHz
    }

    #[test]
    fn resample_identity_returns_same() {
        let samples = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let resampled = resample(&samples, 16000, 16000);
        assert_eq!(resampled, samples);
    }

    #[test]
    fn write_wav_file_creates_valid_file() {
        let samples = vec![0.5; 16000]; // 1 second at 16kHz
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        write_wav_file(&samples, 16000, path).unwrap();

        // Verify file was written
        let metadata = std::fs::metadata(path).unwrap();
        assert!(metadata.len() > 0);

        // Verify we can read it back
        let reader = hound::WavReader::open(path).unwrap();
        assert_eq!(reader.spec().channels, 1);
        assert_eq!(reader.spec().sample_rate, 16000);
    }

    #[test]
    fn capture_state_serialization() {
        let state = CaptureState::Listening;
        let json = serde_json::to_string(&state).unwrap();
        let decoded: CaptureState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, decoded);
    }

    #[test]
    fn audio_device_info_serialization() {
        let info = AudioDeviceInfo {
            name: "Default Microphone".to_string(),
            is_default: true,
            channels: 2,
            sample_rate: 48000,
        };
        let json = serde_json::to_string(&info).unwrap();
        let decoded: AudioDeviceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, decoded);
    }
}
