use std::{fs, io::BufReader, thread, time::Duration};

use rodio::{source::Source, Decoder, OutputStream};
use colored::*;

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let f = "ascii.txt";

    let mut frame_raw = fs::read_to_string(f).expect("Error reading file!");
    frame_raw = frame_raw.replace(".", " ");

    let frames: Vec<&str> = frame_raw.split("SPLIT").collect();

    let mut frame_num = 0;

    let (_stream, audio_stream_handle) = OutputStream::try_default().unwrap();
    let audio_file = BufReader::new(fs::File::open("bad-apple-audio.mp3").unwrap());
    let audio_source = Decoder::new(audio_file).unwrap();
    audio_stream_handle
        .play_raw(audio_source.convert_samples())
        .unwrap();

    while frame_num <= 2180 {
        print!("{}", "\x1b[1;1H".white().on_black()); //equiv to cls or clear
        print!("{}", frames[frame_num].black().on_white());

        frame_num += 1;

        thread::sleep(Duration::from_secs_f32(1.0 / 10.67125)); // I spent literal hours figuring the exact value out compensating for lag
    }

    print!("{}", "\x1B[2J\x1B[1;1H".white().on_black());
}
