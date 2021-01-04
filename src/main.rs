// Author: Nicholas LaJoie
// Date: January 4, 2021
// Purpose: Messing around with audio-manipulation libraries in Rust

// Test #1: "creak" library - dumps f32 samples of an audio file out to stdout
// Test #2: "hound" library - create a wav file from f32 samples

use std::{env};
use creak;
use hound;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Outout wav file spec
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

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

    let mut out_buf: Vec<f32> = Vec::new();
    let coef: [f32; 4] = [1.0, 0.01, 0.01, 0.01];
    let window_size: usize = 4;
    for window in in_buf.windows(window_size) {
        let mut result: f32 = 0.0;
        for (i, w) in window.iter().enumerate() {
            result += w*coef[i];
        }
        println!("{:?} = {}", window, result);
        out_buf.push(result);
    }

    // Write the new vector to an audio file
    let mut writer = hound::WavWriter::create("test.wav", spec).unwrap();
    for b in out_buf {
        writer.write_sample(b).unwrap();
    }

    eprintln!("{} samples(s) read.", num_samples);
    Ok(())
}
