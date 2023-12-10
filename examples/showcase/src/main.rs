#![windows_subsystem = "windows"]
use prettygooey::accents::AccentColor;

use prettygooey::theme::{TextExt, TextVariant, Theme};

use iced::alignment::Alignment;
use iced::widget::{column, container, horizontal_space, row};
use iced::window::{self, Position};
use iced::{Element, Length, Pixels, Sandbox, Settings};

fn main() -> iced::Result {
    let target_size = (400, 480);
    ShowcaseSandbox::run(Settings {
        window: window::Settings {
            min_size: Some(target_size),
            size: target_size,
            position: Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum ShowcaseSandboxMessage {
    CheckboxValueChanged(bool),
    ButtonPressed,
    RadioButtonChanged(AccentColor),
    TextInputChanged(String),
    SliderChanged(u32),
}

struct ShowcaseSandbox {
    inputs_enabled: bool,
    text_input_value: String,
    button_pressed: bool,
    power_level: u32,
    theme: Theme,
}

impl Sandbox for ShowcaseSandbox {
    type Message = ShowcaseSandboxMessage;

    fn new() -> Self {
        Self {
            inputs_enabled: true,
            text_input_value: String::from(""),
            button_pressed: false,
            power_level: 9001,
            theme: Theme {
                accent_color: AccentColor::Magenta,
            },
        }
    }

    fn title(&self) -> String {
        String::from("Prettygooey Showcase")
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = self.theme.text("Header").size(Pixels(30.0));
        let header_wrap = self.theme.header(
            row![
                title,
                horizontal_space(Length::Fill),
                self.theme.text("Right-aligned widget").size(Pixels(12.0)),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        );

        let table_headers = column![
            self.theme
                .text("Enable all inputs")
                .variant(TextVariant::Dimmed),
            self.theme
                .text("Button clicked")
                .variant(TextVariant::Dimmed),
            self.theme
                .text("Theme selection")
                .variant(TextVariant::Dimmed),
            self.theme.text("Text input").variant(TextVariant::Dimmed),
            self.theme.text("Power level").variant(TextVariant::Dimmed),
        ]
        .spacing(2);

        let table_values = column![
            self.theme.text(match self.inputs_enabled {
                true => "Yes",
                false => "No",
            }),
            self.theme.text(match self.button_pressed {
                true => "Yes",
                false => "No",
            }),
            self.theme.text(self.theme.accent_color),
            self.theme.text(&self.text_input_value),
            self.theme.text(&self.power_level),
        ]
        .spacing(2);

        let mut button = self.theme.button("Click me");
        let mut text_input = self
            .theme
            .text_input("Put some text here", &self.text_input_value);
        if self.inputs_enabled {
            button = button.on_press(ShowcaseSandboxMessage::ButtonPressed);
            text_input = text_input.on_input(ShowcaseSandboxMessage::TextInputChanged);
        }

        let body = container(
            column![
                self.theme.checkbox(
                    "Enable more inputs",
                    self.inputs_enabled,
                    Self::Message::CheckboxValueChanged
                ),
                text_input,
                row![table_headers, table_values].spacing(20),
                self.theme
                    .radio_group(self.theme.accent_color, Self::Message::RadioButtonChanged),
                button,
                self.theme
                    .slider(8995..=9005, self.power_level, Self::Message::SliderChanged),
            ]
            .spacing(20),
        )
        .padding(20);

        let contents = column![header_wrap, body].spacing(5);

        self.theme.primary_container(contents).into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::CheckboxValueChanged(value) => {
                self.inputs_enabled = value;
                if !value {
                    self.button_pressed = false;
                }
            }
            Self::Message::ButtonPressed => {
                self.button_pressed = true;
            }
            Self::Message::RadioButtonChanged(option) => {
                self.theme.accent_color = option;
            }
            Self::Message::TextInputChanged(value) => {
                self.text_input_value = value;
            }
            Self::Message::SliderChanged(value) => {
                self.power_level = value;
            }
        }
    }
}
