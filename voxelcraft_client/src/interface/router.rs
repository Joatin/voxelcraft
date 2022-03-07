use crate::interface::message::Message;
use crate::interface::message::Message::Navigate;
use crate::interface::page::Page;
use crate::interface::pages::{MainPage, GAME_LOADING_PAGE_ROUTE};
use crate::interface::router_flags::RouterFlags;
use iced::button::State;
use iced::{alignment, Application, Button, Command, Container, Element, Length, Text};
use iced_native::Widget;
use std::collections::HashMap;

pub struct Router {
    pages: HashMap<String, Box<dyn Page>>,
    current_route: String,
}

impl Router {
    pub fn should_grab_cursor(&self) -> bool {
        if let Some(page) = self.pages.get(&self.current_route) {
            page.should_grab_cursor()
        } else {
            false
        }
    }
}

impl Application for Router {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = RouterFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                pages: flags.pages,
                current_route: flags.initial_route,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Hello")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &message {
            Message::Navigate { page } => {
                self.current_route = page.to_string();
            }
            Message::CreateNewGame => self.current_route = GAME_LOADING_PAGE_ROUTE.to_string(),
            _ => {}
        }

        let new_messages = self
            .pages
            .iter_mut()
            .map(|(_, page)| page.update(&message))
            .flatten()
            .collect::<Vec<_>>();

        let commands = new_messages
            .into_iter()
            .map(|message| self.update(message))
            .collect::<Vec<_>>();

        Command::batch(commands)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        if let Some(page) = self.pages.get_mut(&self.current_route) {
            page.view()
        } else {
            panic!()
        }
    }
}
