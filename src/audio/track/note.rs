//! # Note
//!
//! Audio notes frequency

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Note {
    Do,
    Re,
    Mi,
    Fa,
    Sol,
    La,
    Si,
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Note {
    pub fn freq(self, octave: u8) -> f32 {
        let a440 = Note::A.note_nr(4);
        2f32.powf(((self.note_nr(octave) as i16 - a440 as i16) as f32) / 12.0) * 440.0
    }

    fn note_nr(&self, octave: u8) -> u8 {
        self.pitch_class() + 12 * octave
    }

    fn pitch_class(&self) -> u8 {
        match self {
            Self::Do | Self::C => 0,
            Self::Cs => 1,
            Self::Re | Self::D => 2,
            Self::Ds => 3,
            Self::Mi | Self::E => 4,
            Self::Fa | Self::F => 5,
            Self::Fs => 6,
            Self::Sol | Self::G => 7,
            Self::Gs => 8,
            Self::La | Self::A => 9,
            Self::As => 10,
            Self::Si | Self::B => 11,
        }
    }
}
