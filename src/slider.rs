use embed_doc_image::embed_doc_image;
use iced::{widget::slider, BorderRadius, Color};

use crate::accents::{BorderColorVariant, PrimaryFillColorVariant};
use crate::theme::Theme;

impl Theme {
    fn _default_slider_appearance(&self) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (
                    self.accent_color
                        .primary_fill_color(PrimaryFillColorVariant::Regular),
                    Color::WHITE,
                ),
                width: 12.0,
                border_radius: BorderRadius::from(5.0),
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 12.0 },
                color: Color::WHITE,
                border_width: 1.0,
                border_color: self.accent_color.border_color(BorderColorVariant::Regular),
            },
        }
    }
}

impl slider::StyleSheet for Theme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            ..self._default_slider_appearance()
        }
    }

    fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (
                    self.accent_color
                        .primary_fill_color(PrimaryFillColorVariant::Pressed),
                    Color::WHITE,
                ),
                width: 12.0,
                border_radius: BorderRadius::from(5.0),
            },
            ..self._default_slider_appearance()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (
                    self.accent_color
                        .primary_fill_color(PrimaryFillColorVariant::Hovered),
                    Color::WHITE,
                ),
                width: 12.0,
                border_radius: BorderRadius::from(5.0),
            },
            ..self._default_slider_appearance()
        }
    }
}

impl Theme {
    /// Instantiates a slider widget.
    ///
    /// A slider is always enabled. Due to limitations of [`iced::widget::slider::Slider`],
    /// a disabled state is currently
    /// [not available](https://github.com/iced-rs/iced/issues/1066).
    ///
    /// ![Slider][slider]
    /// ```
    /// use prettygooey::accents::AccentColor;
    /// use prettygooey::theme::Theme;
    ///
    /// // Your Sandbox's Message enum
    /// #[derive(Clone)]
    /// enum Message {
    ///     MySliderChanged(u32),
    /// }
    ///
    /// // In your Sandbox's view function
    /// let theme = Theme {
    ///     accent_color: AccentColor::Magenta,
    /// };
    /// let slider_value = 50;
    /// let my_slider = theme.slider(0..=100, slider_value, Message::MySliderChanged);
    /// ```
    #[embed_doc_image("slider", "docs/img/slider.png")]
    pub fn slider<'a, T, Message>(
        &self,
        range: std::ops::RangeInclusive<T>,
        value: T,
        on_change: impl Fn(T) -> Message + 'a,
    ) -> iced::widget::Slider<'a, T, Message>
    where
        T: Copy + From<u8> + std::cmp::PartialOrd,
        Message: Clone,
    {
        slider(range, value, on_change).style(iced::theme::Slider::Custom(Box::new(*self)))
    }
}
