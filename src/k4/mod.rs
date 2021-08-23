pub mod amp;
pub mod effect;
pub mod filter;
pub mod lfo;
pub mod multi;
pub mod single;
pub mod source;
pub mod wave;
pub mod drum;
pub mod bank;

pub const NAME_LENGTH: usize = 10;  // length of patch name
pub const SOURCE_COUNT: usize = 4;  // number of sources in a single patch
pub const DRUM_NOTE_COUNT: usize = 61; // number of DRUM notes
pub const SUBMIX_COUNT: usize = 8;  // number of submix channels / outputs

fn get_effect_number(b: u8) -> u8 {
    let value = b & 0b00011111;
    // Now we should have a value in the range 0~31.
    // Use range 1~32 when storing the value.
    value + 1
}

pub fn get_note_name(note_number: u8) -> String {
    let notes = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ];
    let octave = (note_number / 12) - 2;
    let name = notes[note_number as usize % 12];

    format!("{}{}", name, octave.to_string())
}
