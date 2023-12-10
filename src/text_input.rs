use crate::{
    accents::BorderColorVariant,
    common::{FILL_DISABLED, TEXT_COLOR_DEFAULT, TEXT_COLOR_DISABLED},
    theme::Theme,
};
use embed_doc_image::embed_doc_image;
use iced::{
    widget::{text_input, TextInput},
    BorderRadius, Color,
};

impl Theme {
    fn _default_text_input_appearance(&self) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.05)),
            border_radius: BorderRadius::from(5.0),
            border_width: 1.0,
            border_color: self.accent_color.border_color(BorderColorVariant::Regular),
            icon_color: Color::WHITE,
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self._default_text_input_appearance()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: FILL_DISABLED,
            ..self._default_text_input_appearance()
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        TEXT_COLOR_DISABLED
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border_color: self.accent_color.border_color(BorderColorVariant::Focused),
            ..self._default_text_input_appearance()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border_color: self.accent_color.border_color(BorderColorVariant::Hovered),
            ..self._default_text_input_appearance()
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.25, 0.25, 0.25)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        TEXT_COLOR_DEFAULT
    }
}

impl Theme {
    /// Instantiates a widget containing editable text.
    ///
    /// The input field is disabled by default. You can enable it by chaining an
    /// `on_input` call with a text edit event to the function.
    ///
    /// ![Text input][text_input]
    /// ```
    /// use prettygooey::accents::AccentColor;
    /// use prettygooey::theme::Theme;
    ///
    /// // Your Sandbox's Message enum
    /// #[derive(Clone)]
    /// enum Message {
    ///     MyTextInputChanged(String),
    /// }
    ///
    /// // Put this in your Sandbox's struct
    /// let text_value = String::from("");
    ///
    /// // In your Sandbox's view function
    /// let theme = Theme {
    ///     accent_color: AccentColor::Magenta,
    /// };
    /// let my_text_input = theme.text_input("Put some text here", &text_value)
    ///     .on_input(Message::MyTextInputChanged);
    ///     
    /// // In your Sandbox's update function, respond to the MyTextInputChanged event
    /// // by updating the value of `text_value`.
    /// ```
    #[embed_doc_image("text_input", "docs/img/text_input.png")]
    pub fn text_input<'a, MessageType: Clone>(
        &self,
        placeholder: &str,
        label: &str,
    ) -> TextInput<'a, MessageType> {
        text_input(placeholder, label)
            .padding([10, 15])
            .style(iced::theme::TextInput::Custom(Box::new(*self)))
    }
}
