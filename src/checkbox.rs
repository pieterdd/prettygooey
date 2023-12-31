use embed_doc_image::embed_doc_image;
use iced::{widget::checkbox, Background, BorderRadius, Color};

use crate::accents::{BorderColorVariant, ColorExt, PrimaryFillColorVariant};
use crate::common::{TEXT_COLOR_DEFAULT, TEXT_COLOR_HOVER};
use crate::theme::Theme;

static CHECKBOX_DARKER_GRAY: Color = Color::from_rgb(0.11, 0.11, 0.11);
static CHECKBOX_LIGHTER_GRAY: Color = Color::from_rgb(0.12, 0.12, 0.12);

impl checkbox::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: match is_checked {
                true => self
                    .accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Regular)
                    .to_background(),
                false => Background::Color(CHECKBOX_DARKER_GRAY),
            },
            border_color: match is_checked {
                true => self
                    .accent_color
                    .border_color(BorderColorVariant::RegularColored),
                false => self
                    .accent_color
                    .border_color(BorderColorVariant::RegularGrayscale),
            },
            icon_color: Color::WHITE,
            border_radius: BorderRadius::from(5.0),
            border_width: 1.0,
            text_color: Some(TEXT_COLOR_DEFAULT),
        }
    }

    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: match is_checked {
                true => self.accent_color.secondary_fill_color().to_background(),
                false => Background::Color(CHECKBOX_LIGHTER_GRAY),
            },
            border_color: match is_checked {
                true => self
                    .accent_color
                    .border_color(BorderColorVariant::HoveredColored),
                false => self
                    .accent_color
                    .border_color(BorderColorVariant::HoveredGrayscale),
            },
            icon_color: Color::WHITE,
            border_radius: BorderRadius::from(5.0),
            border_width: 1.0,
            text_color: Some(TEXT_COLOR_HOVER),
        }
    }
}

impl Theme {
    /// Instantiates a checkbox widget.
    ///
    /// The checkbox will take its value from a variable. When the checkbox's state
    /// changes, it will emit an event defined by you. You can hook into this event
    /// via the Sandbox's update function, and update the variable accordingly.
    ///
    /// A checkbox is always enabled. Due to limitations of
    /// [`iced::widget::checkbox::Checkbox`], a disabled state is currently
    /// [unavailable](https://github.com/iced-rs/iced/pull/2109).
    ///
    /// ![Checkbox][checkbox]
    /// ```
    /// use prettygooey::accents::AccentColor;
    /// use prettygooey::theme::Theme;
    ///
    /// // Your Sandbox's Message enum
    /// enum Message {
    ///     // Checkbox will emit an event
    ///     MyCheckboxChanged(bool),
    /// }
    ///
    /// // Put this in your Sandbox struct
    /// let enable_more_inputs = true;
    ///
    /// // In your Sandbox's view function
    /// let theme = Theme {
    ///     accent_color: AccentColor::Magenta,
    /// };
    /// let my_checkbox = theme.checkbox("Enable more inputs", enable_more_inputs, Message::MyCheckboxChanged);
    /// ```
    #[embed_doc_image("checkbox", "docs/img/checkbox.png")]
    pub fn checkbox<'a, Message>(
        &self,
        label: impl Into<String>,
        is_checked: bool,
        f: impl Fn(bool) -> Message + 'a,
    ) -> iced::widget::Checkbox<'a, Message> {
        checkbox(label, is_checked, f)
            .spacing(8)
            .style(iced::theme::Checkbox::Custom(Box::new(*self)))
    }
}
