// Author: Nicholas LaJoie
// Date: January 5, 2021
// Purpose: Messing around with audio-manipulation libraries in Rust

// Test #1: "creak" library - dumps f32 samples of an audio file out to stdout
// Test #2: "hound" library - create a wav file from f32 samples

use std::{env};
use creak;
use hound;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a file name from the cmdline args
    let file_name = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("No audio file specified!");
            return Ok(())
        }
    };

    // Open an audio file of any supported format with one function call
    let decoder = creak::Decoder::open(&file_name)?;

    // Print basic audio info to stderr
    let info = decoder.info();
    eprintln!("Format: {}; Channels: {}; Sample Rate: {}Hz", 
        info.format(), 
        info.channels(), 
        info.sample_rate()
    );

    let mut num_samples: usize = 0;

    // Dump all samples into a vector
    let mut in_buf: Vec<f32> = Vec::new();
    for sample in decoder.into_samples()? {
        in_buf.push(sample.unwrap());
        num_samples += 1;
    }

    // Use a basic (useless) FIR filter
    let mut out_buf: Vec<f32> = Vec::new();
    let coef: [f32; 4] = [1.0, 0.01, 0.01, 0.01];
    let window_size: usize = 4;
    for window in in_buf.windows(window_size) {
        let mut result: f32 = 0.0;
        for (i, w) in window.iter().enumerate() {
            result += w*coef[i];
        }
        out_buf.push(result);
    }

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer1 = hound::WavWriter::create("filtered.wav", spec).unwrap();
    for s in out_buf {
        writer1.write_sample(s).unwrap();
    }

    // Use Int instead of Float
    let bad_spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer2 = hound::WavWriter::create("float_as_int.wav", bad_spec).unwrap();
    for s in &in_buf {
        writer2.write_sample(*s).unwrap();
    }

    // Remove the right channel (every odd sample)
    let mut writer3 = hound::WavWriter::create("left_channel_only.wav", spec).unwrap();
    let mut i: usize = 0;
    for s in &in_buf {
        if i % 2 == 0 {
            writer3.write_sample(*s).unwrap();
        } else {
            writer3.write_sample(0.0).unwrap();
        }
        i = i + 1;
    }

    // Speed up by 2x (remove every other pair of samples)
    let mut writer4 = hound::WavWriter::create("2x_speed.wav", spec).unwrap();
    let mut i: usize = 0;
    for s in &in_buf {
        if i % 2 == 0 {
            writer4.write_sample(*s).unwrap();
        }
        i = i + 1;
    }

    // Slow down by 2x (double up every sample)
    let mut writer5 = hound::WavWriter::create("0.5x_speed.wav", spec).unwrap();
    for s in &in_buf {
        writer5.write_sample(*s).unwrap();
        writer5.write_sample(*s).unwrap();
    }

    eprintln!("{} samples(s) read.", num_samples);
    Ok(())
}
