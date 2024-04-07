use cpal::{traits::{DeviceTrait, HostTrait}, FromSample, SizedSample};

pub fn play() {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        cpal::SampleFormat::I8 => todo!(),
        cpal::SampleFormat::I16 => todo!(),
        cpal::SampleFormat::I32 => todo!(),
        cpal::SampleFormat::I64 => todo!(),
        cpal::SampleFormat::U8 => todo!(),
        cpal::SampleFormat::U16 => todo!(),
        cpal::SampleFormat::U32 => todo!(),
        cpal::SampleFormat::U64 => todo!(),
        cpal::SampleFormat::F32 => todo!(),
        cpal::SampleFormat::F64 => todo!(),
        sf => panic!("Unsupprted sample format: {sf}")
    }
}

pub fn run<T: SizedSample + FromSample<f32>>(device: &cpal::Device, config: &cpal::StreamConfig) {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

}