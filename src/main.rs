use std::{ fs };

fn main() {
    let f = "ascii.txt";

    let mut frame_raw = fs::read_to_string(f).expect("Error reading file!");
    frame_raw = frame_raw.replace(".", " ");

    let frames: Vec<&str> = frame_raw.split("SPLIT").collect();
}
