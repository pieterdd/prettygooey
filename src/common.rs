use iced::{Background, Color};

pub(crate) static BORDER_COLOR_DEFAULT: Color = Color::from_rgb(0.23, 0.23, 0.23);
pub(crate) static BORDER_COLOR_HOVER: Color = Color::from_rgba(0.3, 0.3, 0.3, 0.6);
pub(crate) static BORDER_COLOR_FOCUSED: Color = Color::from_rgba(0.35, 0.35, 0.35, 0.6);

pub(crate) static TEXT_COLOR_DEFAULT: Color = Color::from_rgb(0.95, 0.95, 0.95);
pub(crate) static TEXT_COLOR_HOVER: Color = Color::WHITE;
pub(crate) static TEXT_COLOR_PRESSED: Color = Color::from_rgb(0.7, 0.7, 0.7);
pub(crate) static TEXT_COLOR_DISABLED: Color = Color::from_rgb(0.4, 0.4, 0.4);

pub(crate) static FILL_DISABLED: Background = Background::Color(Color::from_rgb(0.2, 0.2, 0.2));
