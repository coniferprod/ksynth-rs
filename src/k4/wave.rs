use std::fmt;
use std::convert::TryInto;

use crate::k4::WaveNumber;
use crate::SystemExclusiveData;

static WAVE_NAMES: &[&str] = &[
    "(not used)",  // just to bring the index in line with the one-based wave number

    // 1 ~ 96 = CYCLIC WAVE LIST
    "SIN 1ST",
    "SIN 2ND",
    "SIN 3RD",
    "SIN 4TH",
    "SIN 5TH",
    "SIN 6TH",
    "SIN 7TH",
    "SIN 8TH",
    "SIN 9TH",
    "SAW 1",
    "SAW 2",
    "SAW 3",
    "SAW 4",
    "SAW 5",
    "SAW 6",
    "SAW 7",
    "SAW 8",
    "PULSE",
    "TRIANGLE",
    "SQUARE",
    "RECTANGULAR 1",
    "RECTANGULAR 2",
    "RECTANGULAR 3",
    "RECTANGULAR 4",
    "RECTANGULAR 5",
    "RECTANGULAR 6",
    "PURE HORN L",
    "PUNCH BRASS 1",
    "OBOE 1",
    "OBOE 2",
    "CLASSIC GRAND",
    "EP 1",
    "EP 2",
    "EP 3",
    "E.ORGAN 1",
    "E.ORGAN 2",
    "POSITIF",
    "E.ORGAN 3",
    "E.ORGAN 4",
    "E.ORGAN 5",
    "E.ORGAN 6",
    "E.ORGAN 7",
    "E.ORGAN 8",
    "E.ORGAN 9",
    "CLASSIC GUITAR",
    "STEEL STRINGS",
    "HARP",
    "WOOD BASS",
    "SYN BASS 3",
    "DIGI BASS",
    "FINGER BASS",
    "MARIMBA",
    "SYN VOICE",
    "GLASS HARP 1",
    "CELLO",
    "XYLO",
    "EP 4",
    "SYN CLAVI 1",
    "EP 5",
    "E.ORGAN 10",
    "E.ORGAN 11",
    "E.ORGAN 12",
    "BIG PIPE",
    "GLASS HARP 2",
    "RANDOM",
    "EP 6",
    "SYN BASS 4",
    "SYN BASS 1",
    "SYN BASS 2",
    "QUENA",
    "OBOE 3",
    "PURE HORN H",
    "FAT BRASS",
    "PUNCH BRASS 2",
    "EP 7",
    "EP 8",
    "SYN CLAVI 2",
    "HARPSICHORD M",
    "HARPSICHORD L",
    "HARPSICHORD H",
    "E.ORGAN 13",
    "KOTO",
    "SITAR L",
    "SITAR H",
    "PICK BASS",
    "SYN BASS 5",
    "SYN BASS 6",
    "VIBRAPHONE ATTACK",
    "VIBRAPHONE 1",
    "HORN VIBE",
    "STEEL DRUM 1",
    "STEEL DRUM 2",
    "VIBRAPHONE 2",
    "MARIMBA ATTACK",
    "HARMONICA",
    "SYNTH",

    // 97 ~ 256 PCM WAVE LIST
    // DRUM & PERCUSSION GROUP
    "KICK",
    "GATED KICK",
    "SNARE TITE",
    "SNARE DEEP",
    "SNARE HI",
    "RIM SNARE",
    "RIM SHOT",
    "TOM",
    "TOM VR",
    "E.TOM",
    "HH CLOSED",
    "HH OPEN",
    "HH OPEN VR",
    "HH FOOT",
    "CRASH",
    "CRASH VR",
    "CRASH VR 2",
    "RIDE EDGE",
    "RIDE EDGE VR",
    "RIDE CUP",
    "RIDE CUP VR",
    "CLAPS",
    "COWBELL",
    "CONGA",
    "CONGA SLAP",
    "TAMBOURINE",
    "TAMBOURINE VR",
    "CLAVES",
    "TIMBALE",
    "SHAKER",
    "SHAKER VR",
    "TIMPANI",
    "TIMPANI VR",
    "SLEIBELL",
    "BELL",
    "METAL HIT",
    "CLICK",
    "POLE",
    "GLOCKEN",
    "MARIMBA",
    "PIANO ATTACK",
    "WATER DROP",
    "CHAR",

    // MULTI GROUP
    "PIANO NRML",
    "PIANO VR",
    "CELLO NRML",
    "CELLO VR1",
    "CELLO VR2",
    "CELLO 1 SHOT",
    "STRINGS NRML",
    "STRINGS VR",
    "SLAP BASS L NRML",
    "SLAP BASS L VR",
    "SLAP BASS L 1 SHOT",
    "SLAP BASS H NRML",
    "SLAP BASS H VR",
    "SLAP BASS H 1 SHOT",
    "PICK BASS NRML",
    "PICK BASS VR",
    "PICK BASS 1 SHOT",
    "WOOD BASS ATTACK",
    "WOOD BASS NRML",
    "WOOD BASS VR",
    "FRETLESS NRML",
    "FRETLESS VR",
    "SYN.BASS NRML",
    "SYN.BASS VR",
    "E.G MUTE NRML",
    "E.G MUTE VR",
    "E.G MUTE 1 SHOT",
    "DIST MUTE NRML",
    "DIST MUTE VR",
    "DIST MUTE 1 SHOT",
    "DIST LEAD NRML",
    "DIST LEAD VR",
    "E.GUITAR NRML",
    "GUT GUITAR NRML",
    "GUT GUITAR VR",
    "GUT GUITAR 1 SHOT",
    "FLUTE NRML",
    "FLUTE 1 SHOT",
    "BOTTLE BLOW NRML",
    "BOTTLE BLOW VR",
    "SAX NRML",
    "SAX VR 1",
    "SAX VR 2",
    "SAX 1 SHOT",
    "TRUMPET NRML",
    "TRUMPET VR 1",
    "TRUMPET VR 2",
    "TRUMPET 1 SHOT",
    "TROMBONE NRML",
    "TROMBONE VR",
    "TROMBONE 1 SHOT",
    "VOICE",
    "NOISE",

    // BLOCK GROUP
    "PIANO 1",
    "PIANO 2",
    "PIANO 3",
    "PIANO 4",
    "PIANO 5",
    "CELLO 1",
    "CELLO 2",
    "CELLO 3",
    "CELLO 4 1 SHOT",
    "CELLO 5 1 SHOT",
    "CELLO 6 1 SHOT",
    "STRINGS 1",
    "STRINGS 2",
    "SLAP BASS L",
    "SLAP BASS L 1 SHOT",
    "SLAP BASS H",
    "SLAP BASS H 1 SHOT",
    "PICK BASS 1",
    "PICK BASS 2 1 SHOT",
    "PICK BASS 3 1 SHOT",
    "E.G MUTE",
    "E.G MUTE 1 SHOT",
    "DIST LEAD 1",
    "DIST LEAD 2",
    "DIST LEAD 3",
    "GUT GUITAR 1",
    "GUT GUITAR 2",
    "GUT GUITAR 3 1 SHOT",
    "GUT GUITAR 4 1 SHOT",
    "FLUTE 1",
    "FLUTE 2",
    "SAX 1",
    "SAX 2",
    "SAX 3",
    "SAX 4 1 SHOT",
    "SAX 5 1 SHOT",
    "SAX 6 1 SHOT",
    "TRUMPET",
    "TRUMPET 1 SHOT",
    "VOICE 1",
    "VOICE 2",

    // REVERSE & LOOP
    "REVERSE 1",
    "REVERSE 2",
    "REVERSE 3",
    "REVERSE 4",
    "REVERSE 5",
    "REVERSE 6",
    "REVERSE 7",
    "REVERSE 8",
    "REVERSE 9",
    "REVERSE 10",
    "REVERSE 11",
    "LOOP 1",
    "LOOP 2",
    "LOOP 3",
    "LOOP 4",
    "LOOP 5",
    "LOOP 6",
    "LOOP 7",
    "LOOP 8",
    "LOOP 9",
    "LOOP 10",
    "LOOP 11",
    "LOOP 12"
];

#[derive(Copy, Clone)]
pub struct Wave {
    pub number: WaveNumber,  // 1~256
}

impl Default for Wave {
    fn default() -> Self {
        Wave {
            number: WaveNumber::new(1).unwrap()
        }
    }
}

impl Wave {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&self) -> String {
        WAVE_NAMES[self.number.into_inner() as usize].to_string()
    }
}

impl fmt::Display for Wave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.number.into_inner(),
            self.name()
        )
    }
}

impl SystemExclusiveData for Wave {
    fn from_bytes(data: Vec<u8>) -> Self {
        let high = data[0] & 0x01;  // `wave select h` is b0 of s34/s35/s36/s37
        let low = data[1] & 0x7f;   // `wave select l` is bits 0...6 of s38/s39/s40/s41
        Wave {
            number: WaveNumber::new((((high as u16) << 7) | low as u16) + 1).unwrap(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let n = self.number.into_inner() - 1;
        vec![
            (n & 0b0000_0001).try_into().unwrap(),
            (n & 0x7f).try_into().unwrap(),
        ]
    }

    fn data_size(&self) -> usize { 2 }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_wave_name() {
        let wave = Wave {
            number: WaveNumber::new(1).unwrap(),
        };

        assert_eq!(wave.name(), "SIN 1ST");
    }

    #[test]
    fn test_wave_from_bytes() {
        let w = Wave::from_bytes(vec![0x01, 0x7f]);
        assert_eq!(w.number.into_inner(), 256);
    }

    #[test]
    fn test_wave_to_bytes() {
        let wave = Wave {
            number: WaveNumber::new(256).unwrap(),
        };

        assert_eq!(wave.to_bytes(), vec![0x01, 0x7f]);
    }
}
