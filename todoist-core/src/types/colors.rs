//! Enum for supported colors
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Colors {
    BerryRed = 30,
    Red = 31,
    Orange = 32,
    Yellow = 33,
    OliveGreen = 34,
    LimeGreen = 35,
    Green = 36,
    MintGreen = 37,
    Teal = 38,
    SkyBlue = 39,
    LightBlue = 40,
    Blue = 41,
    Grape = 42,
    Violet = 43,
    Lavender = 44,
    Magenta = 45,
    Salmon = 46,
    Charcoal = 47,
    Grey = 48,
    Taupe = 49
}

impl Default for Colors {
    fn default() -> Self {
        Self::Grey
    }
}
