use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rb::{RbConsumer, RbProducer, SpscRb, RB};
use symphonia::core::{
    audio::{AudioBufferRef, RawSample, SampleBuffer, SignalSpec},
    conv::{ConvertibleSample, IntoSample},
    units::Duration,
};

pub trait AudioOutput {
    fn write(&mut self, decoded: AudioBufferRef);
    fn flush(&mut self);
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

pub struct AudioOutputFactory;

impl AudioOutputFactory {
    pub fn open(spec: &SignalSpec, duration: Duration) -> Box<dyn AudioOutput> {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        match config.sample_format() {
            cpal::SampleFormat::F32 => Box::new(Output::<f32>::open(spec, duration, device)),
            format => panic!("unimplemented sample format: {format}"),
        }
    }
}

pub struct Output<T: AudioOutputSample> {
    ring_buffer: rb::Producer<T>,
    sample_buffer: SampleBuffer<T>,
    stream: cpal::Stream,
}

impl<T: AudioOutputSample> Output<T> {
    pub fn open(spec: &SignalSpec, duration: Duration, device: cpal::Device) -> Output<T> {
        let channels = spec.channels.count();

        let config = cpal::StreamConfig {
            channels: channels as u16,
            sample_rate: cpal::SampleRate(spec.rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let ring_len = (200 * config.sample_rate.0 as usize) / 1000 * channels;
        let ring: SpscRb<T> = SpscRb::new(ring_len);
        let (rb_producer, rb_consumer) = (ring.producer(), ring.consumer());

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], &_| {
                    let written = rb_consumer.read(data).unwrap_or(0);
                    data[written..].iter_mut().for_each(|x| *x = T::MID);
                },
                move |e| eprintln!("consume stream error: {e}"),
                None,
            )
            .unwrap();
        stream.play().unwrap();

        let sample_buf = SampleBuffer::new(duration, *spec);

        Self {
            ring_buffer: rb_producer,
            sample_buffer: sample_buf,
            stream,
        }
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
