use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rb::{RbConsumer, RbProducer, SpscRb, RB};
use symphonia::core::{
    audio::{AudioBufferRef, RawSample, SampleBuffer, SignalSpec},
    conv::{ConvertibleSample, IntoSample},
    units::Duration,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioOutputError {
    #[error("No availavle for default host.")]
    DeviceNotAvailable,
    #[error("Default stream config error: {0}")]
    DefaultStreamConfigError(#[from] cpal::DefaultStreamConfigError),
    #[error("Unimplemented format: {0}")]
    UnimplementedFormat(cpal::SampleFormat),
    #[error("Player Stream Error: {0}")]
    PlayStreamError(#[from] cpal::PlayStreamError),
    #[error("Build stream error: {0}")]
    BuildStreamError(#[from] cpal::BuildStreamError),
}

pub trait AudioOutput {
    fn write(&mut self, decoded: AudioBufferRef);
    fn flush(&mut self);
    fn steam_play(&mut self) -> Result<(), AudioOutputError>;
}

pub trait AudioOutputSample:
    cpal::Sample
    + cpal::SizedSample
    + ConvertibleSample
    + IntoSample<f32>
    + RawSample
    + std::marker::Send
    + 'static
{
}

impl AudioOutputSample for f32 {}

pub fn try_open(
    spec: &SignalSpec,
    duration: Duration,
) -> Result<Box<dyn AudioOutput>, AudioOutputError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(AudioOutputError::DeviceNotAvailable)?;
    let config = device.default_output_config()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => Ok(Box::new(Output::<f32>::try_open(spec, duration, device)?)),
        format => Err(AudioOutputError::UnimplementedFormat(format)),
    }
}

pub struct Output<T: AudioOutputSample> {
    ring_buffer: rb::Producer<T>,
    sample_buffer: SampleBuffer<T>,
    stream: cpal::Stream,
}

impl<T: AudioOutputSample> Output<T> {
    pub fn try_open(
        spec: &SignalSpec,
        duration: Duration,
        device: cpal::Device,
    ) -> Result<Output<T>, AudioOutputError> {
        let channels = spec.channels.count();

        let config = cpal::StreamConfig {
            channels: channels as u16,
            sample_rate: cpal::SampleRate(spec.rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let ring_len = (200 * config.sample_rate.0 as usize) / 1000 * channels;
        let ring: SpscRb<T> = SpscRb::new(ring_len);
        let (rb_producer, rb_consumer) = (ring.producer(), ring.consumer());

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [T], &_| {
                let written = rb_consumer.read(data).unwrap_or(0);
                data[written..].iter_mut().for_each(|x| *x = T::MID);
            },
            move |e| eprintln!("consume stream error: {e}"),
            None,
        )?;
        stream.play()?;

        let sample_buf = SampleBuffer::new(duration, *spec);

        Ok(Self {
            ring_buffer: rb_producer,
            sample_buffer: sample_buf,
            stream,
        })
    }

    pub fn write(&mut self, decoded: AudioBufferRef) {
        if decoded.frames() == 0 {
            return;
        }

        self.sample_buffer.copy_interleaved_ref(decoded);
        let mut samples = self.sample_buffer.samples();

        while let Some(written) = self.ring_buffer.write_blocking(samples) {
            samples = &samples[written..];
        }
    }

    pub fn flush(&mut self) {
        let _ = self.stream.pause();
    }
}

impl<T: AudioOutputSample> AudioOutput for Output<T> {
    fn write(&mut self, decoded: AudioBufferRef) {
        self.write(decoded)
    }

    fn flush(&mut self) {
        self.flush()
    }

    fn steam_play(&mut self) -> Result<(), AudioOutputError> {
        self.stream.play()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_config() {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let configs = device.default_output_config().unwrap();
        // println!("{:?}", configs)
    }
}
