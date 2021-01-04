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

    // Dump all samples into a new audio file
    let mut writer = hound::WavWriter::create("test.wav", spec).unwrap();
    for sample in decoder.into_samples()? {
        //let mut samp = &mut sample.unwrap();
        //if (*samp + 0.1) < 1.0 {
        //    // Print as f32 values (-1.0 -> 1.0)
        //    *samp += 0.1;
        //    println!("{:?}", samp);
        //}

        writer.write_sample(sample.unwrap()).unwrap();
        // Print as 4, byte chunks, litte-endian
        //println!("{:?}", &sample.unwrap().to_le_bytes());

        num_samples += 1;
    }

    eprintln!("{} samples(s) read.", num_samples);
    Ok(())
}
