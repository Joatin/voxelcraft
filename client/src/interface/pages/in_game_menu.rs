use crate::interface::components;
use crate::interface::components::styles;
use crate::interface::message::Message;
use crate::interface::page::Page;
use crate::interface::pages::OPTIONS_PAGE_ROUTE;
use crate::interface::IN_GAME_HUD_PAGE_ROUTE;

use iced::{button, Alignment, Column, Element, Length, Space};

pub const IN_GAME_MENU_PAGE_ROUTE: &str = "IN_GAME_MENU";

pub struct InGameMenuPage {
    resume_button: button::State,
    options_button: button::State,
    quit_button: button::State,
}

impl InGameMenuPage {
    pub fn new() -> Self {
        let resume_button = button::State::new();
        let options_button = button::State::new();
        let quit_button = button::State::new();

        Self {
            resume_button,
            options_button,
            quit_button,
        }
    }
}

impl Page for InGameMenuPage {
    fn name(&self) -> &str {
        IN_GAME_MENU_PAGE_ROUTE
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .width(Length::Fill)
            .push(
                Column::new()
                    .width(Length::Units(350))
                    .spacing(8)
                    .push(Space::new(Length::Shrink, Length::Fill))
                    .push(
                        components::button(&mut self.resume_button, "RESUME")
                            .width(Length::Fill)
                            .on_press(Message::Navigate {
                                page: IN_GAME_HUD_PAGE_ROUTE.to_string(),
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
                    .push(Space::new(Length::Shrink, Length::Fill))
                    .align_items(Alignment::Center),
            )
            .align_items(Alignment::Center)
            .into()
    }
}
