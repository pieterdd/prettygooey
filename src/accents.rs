use std::fmt::Display;

use iced::{Background, Color};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq)]
pub enum AccentColor {
    Magenta,
    Green,
}

pub trait ColorExt {
    fn to_background(&self) -> Background;
}

impl ColorExt for Color {
    fn to_background(&self) -> Background {
        Background::Color(self.clone())
    }
}

impl AccentColor {
    pub fn primary_container_background(&self) -> Background {
        match self {
            Self::Magenta => Background::Color(Color::from_rgb(0.12, 0.11, 0.13)),
            Self::Green => Background::Color(Color::from_rgb(0.12, 0.11, 0.13)),
        }
    }

    pub fn primary_fill_color(&self) -> Color {
        match self {
            Self::Magenta => Color::from_rgb(0.27, 0.02, 0.15),
            Self::Green => Color::from_rgb(0.03, 0.16, 0.1),
        }
    }

    pub fn hovered_primary_fill_color(&self) -> Color {
        match self {
            Self::Magenta => Color::from_rgb(0.29, 0.04, 0.17),
            Self::Green => Color::from_rgb(0.05, 0.18, 0.12),
        }
    }

    pub fn pressed_primary_fill_color(&self) -> Color {
        match self {
            Self::Magenta => Color::from_rgb(0.25, 0.00, 0.13),
            Self::Green => Color::from_rgb(0.01, 0.14, 0.08),
        }
    }

    pub fn secondary_fill_color(&self) -> Color {
        match self {
            Self::Magenta => Color::from_rgb(0.34, 0.09, 0.2),
            Self::Green => Color::from_rgb(0.10, 0.23, 0.15),
        }
    }
}

impl Display for AccentColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Magenta => write!(f, "Magenta"),
            Self::Green => write!(f, "Green"),
        }
    }
}
