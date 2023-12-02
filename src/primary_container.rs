use iced::{
    widget::{container, Container},
    Element, Length,
};

use crate::theme::Theme;

impl Theme {
    /// Instantiates a container that applies Prettygooey's window background. Always use this
    /// as the foundation of your layout.
    pub fn primary_container<'a, T>(self, contents: impl Into<Element<'a, T>>) -> Container<'a, T> {
        container(contents)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_: &_| container::Appearance {
                background: Some(self.accent_color.primary_container_background()),
                ..Default::default()
            })
    }
}
