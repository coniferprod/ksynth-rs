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

const NAME_LENGTH: usize = 10;  // length of patch name
const SOURCE_COUNT: usize = 4;  // number of sources in a single patch
const DRUM_NOTE_COUNT: usize = 61; // number of DRUM notes
const SUBMIX_COUNT: usize = 8;

fn get_effect_number(b: u8) -> u8 {
    let value = b & 0b00011111;
    // Now we should have a value in the range 0~31.
    // Use range 1~32 when storing the value.
    value + 1
}
