//! Our own color file!

use std::fmt;

const RED: [f32; 4] = [0.905, 0.298, 0.235, 1.0];
const BLUE: [f32; 4] = [0.556, 0.266, 0.678, 1.0];
const GREEN: [f32; 4] = [0.152, 0.682, 0.376, 1.0];
const MELON: [f32; 4] = [0.945, 0.768, 0.058, 1.0];
const END: [f32; 4] = [0.172, 0.243, 0.313, 1.0];

pub enum Color {
    Red,
    Blue,
    Green,
    Melon,
    GameEndText,
}

impl Color {
    pub fn get(&self) -> &[f32; 4] {
        match self {
            Self::Blue => &BLUE,
            Self::Red => &RED,
            Self::Green => &GREEN,
            Self::Melon => &MELON,
            Self::GameEndText => &END,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blue => "Blue",
                Self::Red => "Red",
                _ => panic!("We shall print only snake colors"),
            }
        )
    }
}
