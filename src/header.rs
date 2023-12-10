use embed_doc_image::embed_doc_image;
use iced::{
    widget::{column, container, row, Container},
    Element, Length, Pixels,
};

use crate::{
    accents::{ColorExt, PrimaryFillColorVariant},
    theme::Theme,
};

impl Theme {
    /// Instantiates a specialized container to display a title, an app icon or other
    /// decorative content at the top of your window.
    ///
    /// You can slot a simple text widget into it as demonstrated below. For more
    /// advanced cases, you may want to slot in a row with some inter-widget spacing
    /// and center the row widgets horizontally.
    ///
    /// ![Header][header]
    /// ```
    /// use prettygooey::accents::AccentColor;
    /// use prettygooey::theme::Theme;
    /// use iced::{Alignment, Length, Pixels};
    /// use iced::widget::{Container, horizontal_space, row};
    ///
    /// // Your Sandbox's Message enum
    /// enum Message {}
    ///
    /// // In your Sandbox's view function
    /// let theme = Theme {
    ///     accent_color: AccentColor::Magenta,
    /// };
    ///
    /// // Place one of these at the top of your primary container
    /// let simple_header: Container<'_, Message> = theme.header(theme.text("Header").size(Pixels(30.0)));
    /// let advanced_header: Container<'_, Message> = theme.header(
    ///     row![
    ///         theme.text("Header").size(Pixels(30.0)),
    ///         horizontal_space(Length::Fill),
    ///         theme.text("Right-aligned widget").size(Pixels(12.0)),
    ///     ]
    ///     .spacing(10)
    ///     .align_items(Alignment::Center),
    /// );
    /// ```
    #[embed_doc_image("header", "docs/img/header.png")]
    pub fn header<'a, T: 'a>(self, contents: impl Into<Element<'a, T>>) -> Container<'a, T> {
        let header_main = container(contents)
            .width(Length::Fill)
            .style(move |_: &_| container::Appearance {
                background: Some(
                    self.accent_color
                        .primary_fill_color(PrimaryFillColorVariant::Regular)
                        .to_background(),
                ),
                ..Default::default()
            })
            .padding([10, 20]);
        let header_separator = container(row![])
            .width(Length::Fill)
            .height(Pixels(1.0))
            .style(move |_: &_| container::Appearance {
                background: Some(self.accent_color.secondary_fill_color().to_background()),
                ..Default::default()
            });
        container(column![header_main, header_separator])
    }
}
