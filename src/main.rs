const IONIAN: [u8; 8] = [
    0,  // Root,
    2,  // Whole,
    4,  // Whole,
    5,  // Half, 
    7,  // Whole,
    9,  // Whole,
    11, // Whole,
    12, // Half, 
];

const DORIAN: [u8; 8] = [
    0,  // Root,
    2,  // Whole,
    3,  // Half, 
    5,  // Whole,
    7,  // Whole,
    9,  // Whole,
    10, // Half, 
    12, // Whole,
];

const PHRYGIAN: [u8; 8] = [
    0,  // Root,
    1,  // Half,
    3,  // Whole,
    5,  // Whole,
    7,  // Whole,
    8,  // Half,
    10, // Whole,
    12, // Whole,
];

const LYDIAN: [u8; 8] = [
    0,  // Root,
    2,  // Whole,
    4,  // Whole,
    6,  // Whole,
    7,  // Half,
    9,  // Whole,
    11, // Whole,
    12, // Half,
];

const MIXOLYDIAN: [u8; 8] = [
    0,  // Root,
    2,  // Whole,
    4,  // Whole,
    5,  // Half,
    7,  // Whole,
    9,  // Whole,
    10, // Half,
    12, // Whole,
];

const AEOLIAN: [u8; 8] = [
    0,  // Root,
    2,  // Whole,
    3,  // Half,
    5,  // Whole,
    7,  // Whole,
    8,  // Half,
    10, // Whole,
    12, // Whole,
];

const LOCRIAN: [u8; 8] = [
    0,  // Root,
    1,  // Half,
    3,  // Whole,
    5,  // Whole,
    6,  // Half,
    8,  // Whole,
    10, // Whole,
    12, // Whole,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

pub use Mode::*;

impl Mode {
    fn get_steps(&self) -> [u8; 8] {
        match self {
            Ionian     => IONIAN,
            Dorian     => DORIAN,
            Phrygian   => PHRYGIAN,
            Lydian     => LYDIAN,
            Mixolydian => MIXOLYDIAN,
            Aeolian    => AEOLIAN,
            Locrian    => LOCRIAN,
        }
    }
}

impl<'a> From<&'a str> for Mode {
    fn from(input: &'a str) -> Self {
        match input {
            "Ionian"     => Ionian,
            "Dorian"     => Dorian,
            "Phrygian"   => Phrygian,
            "Lydian"     => Lydian,
            "Mixolydian" => Mixolydian,
            "Aeolian"    => Aeolian,
            "Locrian"    => Locrian,
            invalid            => unreachable!("Invalid mode: {invalid:?}")
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Sharp {
    Bsharp = 0,
    Csharp = 1,
    D      = 2,
    Dsharp = 3,
    E      = 4,
    Esharp = 5,
    Fsharp = 6,
    G      = 7,
    Gsharp = 8,
    A      = 9,
    Asharp = 10,
    B      = 11,
}

impl Sharp {
    // fn distance(&self, other: &Self) -> u8 {
    //     let r = *self as u8;
    //     let mut o = *other as u8;

    //     if o < r {
    //         o += 12
    //     }

    //     o-r
    // }

    fn as_str<'a>(&'a self) -> &'a str {
        match self {
            Self::Bsharp => "B#",
            Self::Csharp => "C#",
            Self::D      => "D",
            Self::Dsharp => "D#",
            Self::E      => "E",
            Self::Esharp => "E#",
            Self::Fsharp => "F#",
            Self::G      => "G",
            Self::Gsharp => "G#",
            Self::A      => "A",
            Self::Asharp => "A#",
            Self::B      => "B",
        }
    }

    fn to_flat(&self) -> Flat {
        match self {
            Self::Bsharp => Flat::C,
            Self::Csharp => Flat::Dflat,
            Self::D      => Flat::D,
            Self::Dsharp => Flat::Eflat,
            Self::E      => Flat::Fflat,
            Self::Esharp => Flat::F,
            Self::Fsharp => Flat::Gflat,
            Self::G      => Flat::G,
            Self::Gsharp => Flat::Aflat,
            Self::A      => Flat::A,
            Self::Asharp => Flat::Bflat,
            Self::B      => Flat::Cflat,
        }
    }
}

impl From<u8> for Sharp {
    fn from(value: u8) -> Self {
        match value {
            0  => Self::Bsharp,
            1  => Self::Csharp,
            2  => Self::D,
            3  => Self::Dsharp,
            4  => Self::E,
            5  => Self::Esharp,
            6  => Self::Fsharp,
            7  => Self::G,
            8  => Self::Gsharp,
            9  => Self::A,
            10 => Self::Asharp,
            11 => Self::B,
            n  => {
                println!("invalid: {n}");
                unreachable!()
            }
        }
    }
}

impl std::ops::Add<u8> for Sharp {
    type Output = Self;

    fn add(self, step: u8) -> Self::Output {
        let next = (self as u8 + step) % 12;
        next.into()
    }
}

impl std::ops::Sub<u8> for Sharp {
    type Output = Self;

    fn sub(self, step: u8) -> Self::Output {
        let prev = (12 + self as u8 - step) % 12;
        prev.into()
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Flat {
    C      = 0,
    Dflat  = 1,
    D      = 2,
    Eflat  = 3,
    Fflat  = 4,
    F      = 5,
    Gflat  = 6,
    G      = 7,
    Aflat  = 8,
    A      = 9,
    Bflat  = 10,
    Cflat  = 11,
}

impl Flat {
    // fn distance(&self, other: &Self) -> u8 {
    //     let r = *self as u8;
    //     let mut o = *other as u8;

    //     if o < r {
    //         o += 12
    //     }

    //     o-r
    // }

    fn as_str<'a>(&'a self) -> &'a str {
        match self {
            Self::C      => "C",
            Self::Dflat  => "Db",
            Self::D      => "D",
            Self::Eflat  => "Eb",
            Self::Fflat  => "Fb",
            Self::F      => "F",
            Self::Gflat  => "Gb",
            Self::G      => "G",
            Self::Aflat  => "Ab",
            Self::A      => "A",
            Self::Bflat  => "Bb",
            Self::Cflat  => "Cb",
        }
    }

    fn to_sharp(&self) -> Sharp {
        match self {
            Self::C      => Sharp::Bsharp,
            Self::Dflat  => Sharp::Csharp,
            Self::D      => Sharp::D,
            Self::Eflat  => Sharp::Dsharp,
            Self::Fflat  => Sharp::E,
            Self::F      => Sharp::Esharp,
            Self::Gflat  => Sharp::Fsharp,
            Self::G      => Sharp::G,
            Self::Aflat  => Sharp::Gsharp,
            Self::A      => Sharp::A,
            Self::Bflat  => Sharp::Asharp,
            Self::Cflat  => Sharp::B,
        }
    }
}

impl From<u8> for Flat {
    fn from(value: u8) -> Self {
        match value {
            0  => Self::C,
            1  => Self::Dflat,
            2  => Self::D,
            3  => Self::Eflat,
            4  => Self::Fflat,
            5  => Self::F,
            6  => Self::Gflat,
            7  => Self::G,
            8  => Self::Aflat,
            9  => Self::A,
            10 => Self::Bflat,
            11 => Self::Cflat,
            n  => {
                println!("invalid: {n}");
                unreachable!()
            }
        }
    }
}

impl std::ops::Add<u8> for Flat {
    type Output = Self;

    fn add(self, step: u8) -> Self::Output {
        let next = (self as u8 + step) % 12;
        next.into()
    }
}

impl std::ops::Sub<u8> for Flat {
    type Output = Self;

    fn sub(self, step: u8) -> Self::Output {
        let prev = (12 + self as u8 - step) % 12;
        prev.into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Base {
    Sharp(Sharp),
    Flat(Flat),
}

impl Base {
    fn enharmonic(&self) -> Self {
        match self {
            Base::Sharp(sharp) => Self::Flat(sharp.to_flat()),
            Base::Flat(flat) => Self::Sharp(flat.to_sharp()),
        }
    }

    fn distance(&self, other: &Self) -> u8 {
        let r = match self {
            Base::Sharp(sharp) => *sharp as u8,
            Base::Flat(flat) => flat.to_sharp() as u8,
        };

        let mut o = match other {
            Base::Sharp(sharp) => *sharp as u8,
            Base::Flat(flat) => flat.to_sharp() as u8,
        };

        if o < r {
            o += 12
        }

        o-r
    }

    fn as_str<'a>(&'a self) -> &'a str {
        match self {
            Base::Sharp(sharp) => sharp.as_str(),
            Base::Flat(flat) => flat.as_str(),
        }
    }

    // fn as_u8(&self) -> u8 {
    //     match self {
    //         Base::Sharp(sharp) => *sharp as u8,
    //         Base::Flat(flat) => *flat as u8,
    //     }
    // }
}

impl<'a> From<&'a str> for Base {
    fn from(value: &'a str) -> Self {
        match value {
            "C"  => Self::Flat(Flat::C),
            "C#" => Self::Sharp(Sharp::Csharp),
            "Db" => Self::Flat(Flat::Dflat),
            "D"  => Self::Flat(Flat::D),
            "D#" => Self::Sharp(Sharp::Dsharp),
            "Eb" => Self::Flat(Flat::Eflat),
            "E"  => Self::Sharp(Sharp::E),
            "F"  => Self::Flat(Flat::F),
            "F#" => Self::Sharp(Sharp::Fsharp),
            "Gb" => Self::Flat(Flat::Gflat),
            "G"  => Self::Flat(Flat::G),
            "G#" => Self::Sharp(Sharp::Gsharp),
            "Ab" => Self::Flat(Flat::Aflat),
            "A"  => Self::Flat(Flat::A),
            "A#" => Self::Sharp(Sharp::Asharp),
            "Bb" => Self::Flat(Flat::Bflat),
            "B"  => Self::Sharp(Sharp::B),
            n    => { unreachable!("invalid: {n}") }
        }
    }
}

impl From<Sharp> for Base {
    fn from(sharp: Sharp) -> Self {
        Self::Sharp(sharp)
    }
}

impl From<Flat> for Base {
    fn from(flat: Flat) -> Self {
        Self::Flat(flat)
    }
}

impl std::fmt::Debug for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::ops::Add<u8> for Base {
    type Output = Self;

    fn add(self, step: u8) -> Self::Output {
        match self {
            Base::Sharp(sharp) => Self::Sharp(sharp + step),
            Base::Flat(flat) => Self::Flat(flat + step),
        }
    }
}

impl std::ops::Sub<u8> for Base {
    type Output = Self;

    fn sub(self, step: u8) -> Self::Output {
        match self {
            Base::Sharp(sharp) => Self::Sharp(sharp - step),
            Base::Flat(flat) => Self::Flat(flat - step),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChordQuality {
    Major,
    Minor,
    Diminished,
}

use ChordQuality::*;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Chord {
    base: Base,
    quality: ChordQuality,
}

impl Chord {
    fn new(base: Base, quality: ChordQuality) -> Self {
        Self {
            base,
            quality,
        }
    }
}

impl std::fmt::Debug for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q = match self.quality {
            Major => "",
            Minor => "m",
            Diminished => "º",
        };

        write!(f, "{:?}{q}", self.base)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Key {
    root: Base,
    mode: Mode,
}

impl Key {
    fn new(root: impl Into<Base>, mode: Mode) -> Self {
        Self { root: root.into(), mode }
    }

    fn get_notes(&self) -> Notes {
        let steps = self.mode.get_steps();
        let chords = steps.map(|step| self.root + step);

        let mut notes = Notes(chords);
        notes.refine();
        notes
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.root, self.mode)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Notes([Base; 8]);

impl Notes {
    /// root is 1
    // fn get_note(&self, num: u8) -> Base {
    //     self.0[num as usize % 8]
    // }

    fn refine(&mut self) {
        for i in 1..7 {
            let curr = self.0[i].as_str().as_bytes()[0];
            let prev = self.0[i - 1].as_str().as_bytes()[0];

            let distance = if curr < prev {
                (b'G' + (curr - b'A')) - prev
            } else {
                curr - prev
            };

            if curr == prev || distance > 1 {
                self.0[i] = self.0[i].enharmonic();
            }
        }
    }

    fn get_chords(&self) -> Chords {
        let mut chords = self.0.map(|base| Chord::new(base, Major));

        for i in 0..8 {
            let root = self.0[i];
            let third = self.0[(i + 2) % 7];
            let fifth = self.0[(i + 4) % 7];

            let triad = Triad::new(root, third, fifth);
            chords[i].quality = triad.determine_quality();
        }

        Chords(chords)
    }
}

impl std::fmt::Debug for Notes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Chords([Chord; 8]);

impl std::fmt::Debug for Chords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[allow(unused)]
impl Chords {
    fn root(&self)    -> Chord { self.0[0] }
    fn second(&self)  -> Chord { self.0[1] }
    fn third(&self)   -> Chord { self.0[2] }
    fn fourth(&self)  -> Chord { self.0[3] }
    fn fifth(&self)   -> Chord { self.0[4] }
    fn sixth(&self)   -> Chord { self.0[5] }
    fn seventh(&self) -> Chord { self.0[6] }
    fn octave(&self)  -> Chord { self.0[7] }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Triad([Base; 3]);

impl Triad {
    fn new(root: Base, third: Base, fifth: Base) -> Self {
        Self([root, third, fifth])
    }

    fn determine_quality(&self) -> ChordQuality {
        let root_to_third_distance = self.0[0].distance(&self.0[1]);
        let root_to_fifth_distance = self.0[0].distance(&self.0[2]);


        if root_to_fifth_distance == 6 {
            Diminished
        } else if root_to_third_distance == 3 {
            Minor
        } else {
            Major
        }
    }
}

use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    let mut stdin = std::io::stdin();

    loop {
        let len = stdin.read(&mut buf)?;
        let cmd = &buf[..len - 1];

        match cmd {
            k if k.starts_with(b"key") => {
                let mut iter = k.split(|bytes| *bytes == b' ');
                let base = str::from_utf8(iter.nth(1).unwrap()).unwrap();
                let mode = iter.nth(2).map_or_else(|| Ionian, |m| Mode::from(str::from_utf8(m).unwrap()));

                let key = Key::new(base, mode);
                let notes = key.get_notes();
                let chords = notes.get_chords();

                println!("notes  : {notes:?}");
                println!("chords : {chords:?}");
            },
            b"exit" => break,
            _ => {},
        }
    }

    Ok(())
}
