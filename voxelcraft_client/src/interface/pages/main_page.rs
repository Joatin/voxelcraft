use crate::interface::components;
use crate::interface::components::styles;
use crate::interface::message::Message;
use crate::interface::page::Page;
use crate::interface::pages::options_page::OPTIONS_PAGE_ROUTE;
use crate::interface::pages::WORLD_SELECTION_PAGE_ROUTE;
use iced::alignment::Horizontal;
use iced::widget::Button;
use iced::widget::Text;
use iced::{button, Alignment, Color, Column, Element, Length, Row, Space};
use iced_native::Font;

pub const MAIN_PAGE_ROUTE: &str = "MAIN";

pub struct MainPage {
    play_button: button::State,
    options_button: button::State,
    quit_button: button::State,
}

impl MainPage {
    pub fn new() -> Self {
        let play_button = button::State::new();
        let options_button = button::State::new();
        let quit_button = button::State::new();
        Self {
            play_button,
            options_button,
            quit_button,
        }
    }
}

impl Page for MainPage {
    fn name(&self) -> &str {
        MAIN_PAGE_ROUTE
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(Space::new(Length::Shrink, Length::Units(100)))
            .push(
                Text::new("Voxelcraft")
                    .font(Font::External {
                        name: "LuckiestGuy",
                        bytes: include_bytes!("../LuckiestGuy-Regular.ttf"),
                    })
                    .size(80)
                    .color(Color::WHITE)
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Column::new()
                    .width(Length::Units(350))
                    .spacing(8)
                    .push(
                        components::button(&mut self.play_button, "PLAY")
                            .width(Length::Fill)
                            .on_press(Message::Navigate {
                                page: WORLD_SELECTION_PAGE_ROUTE.to_string(),
                            })
                            .style(styles::Button::Primary),
                    )
                    .push(
                        components::button(&mut self.options_button, "OPTIONS")
                            .width(Length::Fill)
                            .on_press(Message::Navigate {
                                page: OPTIONS_PAGE_ROUTE.to_string(),
                            })
                            .style(styles::Button::Secondary),
                    )
                    .push(
                        components::button(&mut self.quit_button, "EXIT GAME")
                            .width(Length::Fill)
                            .on_press(Message::QuitApplication)
                            .style(styles::Button::Secondary),
                    )
                    .align_items(Alignment::Center),
            )
            .into()
    }
}
