use crate::accents::AccentColor;
pub use crate::checkbox::*;
pub use crate::primary_container::*;
pub use crate::text::*;

/// Allows you to create any of the widgets supported by Prettygooey.
///
/// You may want to instantiate `Theme` in your Sandbox's constructor or
/// in its `view` function:
///
/// ```
/// use prettygooey::accents::AccentColor;
/// use prettygooey::theme::Theme;
///
/// let theme = Theme {
///     accent_color: AccentColor::Magenta,
/// };
/// ```
#[derive(Clone, Copy)]
pub struct Theme {
    /// Controls the look of widgets that use accent colors.
    pub accent_color: AccentColor,
}
