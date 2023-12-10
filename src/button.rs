use embed_doc_image::embed_doc_image;
use iced::{widget::button, BorderRadius, Color, Element};

use crate::accents::{BorderColorVariant, ColorExt, PrimaryFillColorVariant};
use crate::common::{
    FILL_DISABLED, TEXT_COLOR_DEFAULT, TEXT_COLOR_DISABLED, TEXT_COLOR_HOVER, TEXT_COLOR_PRESSED,
};
use crate::theme::Theme;

impl Theme {
    fn _default_button_appearance(&self) -> button::Appearance {
        button::Appearance {
            background: Some(
                self.accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Regular)
                    .to_background(),
            ),
            border_radius: BorderRadius::from(5.0),
            border_width: 1.0,
            border_color: self
                .accent_color
                .border_color(BorderColorVariant::RegularColored),
            text_color: TEXT_COLOR_DEFAULT,
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self._default_button_appearance()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(FILL_DISABLED),
            border_color: Color::from_rgba(0.2, 0.2, 0.2, 0.5),
            text_color: TEXT_COLOR_DISABLED,
            ..self._default_button_appearance()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(
                self.accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Hovered)
                    .to_background(),
            ),
            border_color: self
                .accent_color
                .border_color(BorderColorVariant::HoveredColored),
            text_color: TEXT_COLOR_HOVER,
            ..self._default_button_appearance()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(
                self.accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Pressed)
                    .to_background(),
            ),
            border_color: self
                .accent_color
                .border_color(BorderColorVariant::RegularColored),
            text_color: TEXT_COLOR_PRESSED,
            ..self._default_button_appearance()
        }
    }
}

impl Theme {
    /// Instantiates a button widget.
    ///
    /// The button is disabled by default. You can enable it by chaining an `on_press`
    /// call with a button press event to the button function.
    ///
    /// ![Button][button]
    /// ```
    /// use prettygooey::accents::AccentColor;
    /// use prettygooey::theme::Theme;
    ///
    /// // Your Sandbox's Message enum
    /// enum Message {
    ///     MyButtonPressed,
    /// }
    ///
    /// // In your Sandbox's view function
    /// let theme = Theme {
    ///     accent_color: AccentColor::Magenta,
    /// };
    /// let my_button = theme.button("Click me").on_press(Message::MyButtonPressed);
    /// ```
    #[embed_doc_image("button", "docs/img/button.png")]
    pub fn button<'a, B>(&self, content: impl Into<Element<'a, B>>) -> iced::widget::Button<'a, B> {
        button(content)
            .padding([10.0, 20.0])
            .style(iced::theme::Button::Custom(Box::new(*self)))
    }
}
