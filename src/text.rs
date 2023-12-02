use crate::theme::Theme;
use embed_doc_image::embed_doc_image;
use iced::{
    widget::{text, Text},
    Color,
};

static TEXT_DIMMED: Color = Color::from_rgb(0.54, 0.55, 0.54);

pub enum TextVariant {
    Regular,
    Dimmed,
}

/// Prettygooey-specific extensions for [`iced::widget::Text`] that expose
/// additional theming options.
pub trait TextExt<'a> {
    fn variant(self, variant: TextVariant) -> Text<'a>;
}

impl<'a> TextExt<'a> for Text<'a> {
    fn variant(self, variant: TextVariant) -> Self {
        self.style(match variant {
            TextVariant::Regular => Color::WHITE,
            TextVariant::Dimmed => TEXT_DIMMED,
        })
    }
}

impl Theme {
    /// Instantiates a text widget.
    ///
    /// If you import the extension trait [`TextExt`], you can control additional settings
    /// like the text variant.
    ///
    /// ![Text][text]
    /// ```
    /// use prettygooey::accents::AccentColor;
    /// use prettygooey::theme::Theme;
    /// use prettygooey::theme::{TextExt, TextVariant};
    ///
    /// // In your Sandbox's view function
    /// let theme = Theme {
    ///     accent_color: AccentColor::Magenta,
    /// };
    /// let regular_text = theme.text("Regular text");
    /// let dimmed_text = theme.text("Dimmed text").variant(TextVariant::Dimmed);
    /// ```
    #[embed_doc_image("text", "docs/img/text.png")]
    pub fn text<'a>(&self, label: impl ToString) -> iced::widget::Text<'a> {
        text(label).variant(TextVariant::Regular)
    }
}
