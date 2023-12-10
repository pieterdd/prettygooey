use std::fmt::Display;

use embed_doc_image::embed_doc_image;
use iced::widget::{container, radio, Column, Container};
use iced::{Background, Color, Pixels};
use strum::IntoEnumIterator;

use crate::accents::{BorderColorVariant, ColorExt, PrimaryFillColorVariant};
use crate::common::{TEXT_COLOR_DEFAULT, TEXT_COLOR_HOVER};
use crate::theme::Theme;

static RADIO_DARKER_GRAY: Color = Color::from_rgb(0.11, 0.11, 0.11);
static RADIO_LIGHTER_GRAY: Color = Color::from_rgb(0.12, 0.12, 0.12);

impl radio::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> radio::Appearance {
        radio::Appearance {
            background: match is_checked {
                true => self
                    .accent_color
                    .primary_fill_color(PrimaryFillColorVariant::Regular)
                    .to_background(),
                false => Background::Color(RADIO_DARKER_GRAY),
            },
            border_color: self.accent_color.border_color(BorderColorVariant::Regular),
            dot_color: Color::WHITE,
            border_width: 1.0,
            text_color: Some(TEXT_COLOR_DEFAULT),
        }
    }

    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> radio::Appearance {
        radio::Appearance {
            background: match is_checked {
                true => self.accent_color.secondary_fill_color().to_background(),
                false => Background::Color(RADIO_LIGHTER_GRAY),
            },
            border_color: self.accent_color.border_color(BorderColorVariant::Hovered),
            dot_color: Color::WHITE,
            border_width: 1.0,
            text_color: Some(TEXT_COLOR_HOVER),
        }
    }
}

impl Theme {
    /// Instantiates a radio group container.
    ///
    /// This specialized widget renders a list of radio buttons in a container
    /// and displays the value of the `choice` parameter as selected. An event
    /// is emitted whenever the radio group state changes.
    ///
    /// Any enum used with this widget should derive [`strum::EnumIter`].
    /// It ensures Prettygooey can iterate over the variants of the enum you
    /// pass to the radio group.
    ///
    /// Radio groups are always enabled. A disabled state is
    /// [unavailable](https://github.com/iced-rs/iced/issues/1066)
    /// due to limitations of [`iced::widget::radio::Radio`].
    ///
    /// ![radio group][radio_group]
    /// ```
    /// use prettygooey::accents::AccentColor;
    /// use prettygooey::theme::Theme;
    ///
    /// // Your Sandbox's Message enum
    /// #[derive(Clone)]
    /// enum Message {
    ///     // Radio group will emit an event
    ///     MyRadioGroupChanged(AccentColor),
    /// }
    ///
    /// // Put this in your Sandbox's struct
    /// let accent_color = AccentColor::Magenta;
    ///
    /// // In your Sandbox's view function
    /// let theme = Theme {
    ///     accent_color,
    /// };
    /// let my_radio_group = theme.radio_group(accent_color, Message::MyRadioGroupChanged);
    ///
    /// // In your Sandbox's update function, handle the MyRadioGroupChanged and use it
    /// // to update the `accent_color` variable.
    /// ```
    #[embed_doc_image("radio_group", "docs/img/radio_group.png")]
    pub fn radio_group<'a, MessageType, ChoiceType>(
        self,
        choice: ChoiceType,
        f: impl FnOnce(ChoiceType) -> MessageType + Copy + Clone,
    ) -> Container<'a, MessageType>
    where
        MessageType: Clone + 'a,
        ChoiceType: IntoEnumIterator + Copy + Eq + Display,
    {
        let mut columns = Column::new();

        for variant in ChoiceType::iter() {
            columns = columns
                .push(
                    radio::<MessageType, _, ChoiceType>(
                        format!("{}", variant),
                        variant,
                        Some(choice),
                        f,
                    )
                    .size(Pixels(20.0))
                    .spacing(8)
                    .style(iced::theme::Radio::Custom(Box::new(self))),
                )
                .spacing(10);
        }
        container(columns)
    }
}
