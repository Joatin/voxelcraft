use crate::interface::message::Message;
use crate::interface::page::Page;
use crate::interface::pages::in_game_menu::IN_GAME_MENU_PAGE_ROUTE;

use iced::{Column, Element, Length, Space, Text};

pub const IN_GAME_HUD_PAGE_ROUTE: &str = "IN_GAME_HUD";

pub struct InGameHUDPage {}

impl InGameHUDPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for InGameHUDPage {
    fn name(&self) -> &str {
        IN_GAME_HUD_PAGE_ROUTE
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .width(Length::Fill)
            .push(Space::new(Length::Shrink, Length::Fill))
            .push(Text::new("IN GAME HUD"))
            .into()
    }

    fn update(&mut self, message: &Message) -> Vec<Message> {
        if let Message::EscapePressed = message {
            vec![Message::Navigate {
                page: IN_GAME_MENU_PAGE_ROUTE.to_string(),
            }]
        } else {
            vec![]
        }
    }

    fn should_grab_cursor(&self) -> bool {
        true
    }
}
