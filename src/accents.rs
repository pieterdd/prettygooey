use std::fmt::Display;

use iced::{Background, Color};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq)]
pub enum AccentColor {
    Magenta,
    Green,
    Cyan,
}

pub trait ColorExt {
    fn to_background(&self) -> Background;
}

impl ColorExt for Color {
    fn to_background(&self) -> Background {
        Background::Color(*self)
    }
}

pub enum PrimaryFillColorVariant {
    Regular,
    Hovered,
    Pressed,
}

pub enum BorderColorVariant {
    RegularGrayscale,
    RegularColored,
    HoveredGrayscale,
    HoveredColored,
    Focused,
}

impl AccentColor {
    pub fn primary_container_background(&self) -> Background {
        match self {
            Self::Magenta => Background::Color(Color::from_rgb(0.12, 0.11, 0.13)),
            Self::Green => Background::Color(Color::from_rgb(0.11, 0.11, 0.12)),
            Self::Cyan => Background::Color(Color::from_rgb(0.11, 0.12, 0.12)),
        }
    }

    pub fn primary_fill_color(&self, variant: PrimaryFillColorVariant) -> Color {
        match variant {
            PrimaryFillColorVariant::Regular => match self {
                Self::Magenta => Color::from_rgb(0.27, 0.02, 0.15),
                Self::Green => Color::from_rgb(0.03, 0.16, 0.1),
                Self::Cyan => Color::from_rgb(0.09, 0.24, 0.21),
            },
            PrimaryFillColorVariant::Hovered => match self {
                Self::Magenta => Color::from_rgb(0.29, 0.04, 0.17),
                Self::Green => Color::from_rgb(0.05, 0.18, 0.12),
                Self::Cyan => Color::from_rgb(0.11, 0.26, 0.23),
            },
            PrimaryFillColorVariant::Pressed => match self {
                Self::Magenta => Color::from_rgb(0.25, 0.00, 0.13),
                Self::Green => Color::from_rgb(0.01, 0.14, 0.08),
                Self::Cyan => Color::from_rgb(0.07, 0.22, 0.19),
            },
        }
    }

    pub fn secondary_fill_color(&self) -> Color {
        match self {
            Self::Magenta => Color::from_rgb(0.34, 0.09, 0.2),
            Self::Green => Color::from_rgb(0.08, 0.21, 0.15),
            Self::Cyan => Color::from_rgb(0.16, 0.31, 0.28),
        }
    }

    pub fn border_color(&self, variant: BorderColorVariant) -> Color {
        match variant {
            BorderColorVariant::RegularGrayscale => Color::from_rgb(0.23, 0.23, 0.23),
            BorderColorVariant::RegularColored => match self {
                Self::Magenta => Color::from_rgb(0.34, 0.09, 0.2),
                Self::Green => Color::from_rgb(0.1, 0.23, 0.15),
                Self::Cyan => Color::from_rgb(0.16, 0.31, 0.28),
            },
            BorderColorVariant::HoveredGrayscale => Color::from_rgba(0.3, 0.3, 0.3, 0.6),
            BorderColorVariant::HoveredColored => match self {
                Self::Magenta => Color::from_rgb(0.36, 0.11, 0.22),
                Self::Green => Color::from_rgb(0.12, 0.25, 0.17),
                Self::Cyan => Color::from_rgb(0.18, 0.33, 0.30),
            },
            BorderColorVariant::Focused => match self {
                Self::Magenta => Color::from_rgb(0.40, 0.15, 0.26),
                Self::Green => Color::from_rgb(0.16, 0.29, 0.21),
                Self::Cyan => Color::from_rgb(0.22, 0.37, 0.34),
            },
        }
    }
}

impl Display for AccentColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Magenta => write!(f, "Magenta"),
            Self::Green => write!(f, "Green"),
            Self::Cyan => write!(f, "Cyan"),
        }
    }
}
