use std::{thread::{Thread, JoinHandle}, time::Duration, sync::{Arc, RwLock}, os::windows::thread};

use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, SampleFormat, Sample, StreamError, Stream, BuildStreamError};

pub struct Beeper {
    pub stream: Option<Stream>
}
impl Beeper {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let config = supported_config.config();
        let sample_format = supported_config.sample_format();

        let streamres = match sample_format {
            SampleFormat::F32 => run::<f32>(&device, &config),
            SampleFormat::I16 => run::<i16>(&device, &config),
            SampleFormat::U16 => run::<u16>(&device, &config),
        };
        match streamres {
            Ok(stream) => {
                return Beeper {
                    stream: Some(stream)
                }
            },
            Err(e) => {
                println!("audio could not be initialized");
                return Beeper {
                    stream: None
                }
            }
        }
    }
    pub fn play(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().play().unwrap();
        }
    }
    pub fn pause(&self) {
        if self.stream.is_some() {
            self.stream.as_ref().unwrap().pause().unwrap();
        }
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<Stream, BuildStreamError>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin() / 20.0
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    );
    return stream;
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}