// Author: Nicholas LaJoie
// Date: January 4, 2021
// Purpose: Messing around with audio-manipulation libraries in Rust

// Test #1: "creak" library - dumps f32 samples of an audio file out to stdout

use std::{env, io, io::Write};
use creak;

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

    let mut stdout = io::stdout();
    let mut num_samples: usize = 0;

    // Dump all samples to stdout
    for sample in decoder.into_samples()? {
        // stdout.write(&sample?.to_le_bytes())?;
        //print!("{:?}", &sample?.to_le_bytes());
        println!("{:?}", &sample.unwrap());
        num_samples += 1;
    }

    eprintln!("{} samples(s) read.", num_samples);
    Ok(())
}
