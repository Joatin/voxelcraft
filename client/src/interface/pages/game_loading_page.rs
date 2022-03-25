use crate::interface::message::Message;
use crate::interface::page::Page;
use iced::{Column, Element, Length, ProgressBar, Space, Text};
use iced_native::Alignment;

pub const GAME_LOADING_PAGE_ROUTE: &str = "GAME_LOADING";

pub struct GameLoadingPage {
    loading_message: String,
    progress: Option<f32>,
}

impl GameLoadingPage {
    pub fn new() -> Self {
        Self {
            loading_message: "Loading...".to_string(),
            progress: None,
        }
    }
}

impl Page for GameLoadingPage {
    fn name(&self) -> &str {
        GAME_LOADING_PAGE_ROUTE
    }

    fn view(&mut self) -> Element<'_, Message> {
        let column = Column::new()
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(Space::new(Length::Shrink, Length::Fill))
            .push(Text::new(&self.loading_message).size(30));

        let column = if let Some(progress) = self.progress {
            column
                .push(Text::new(&format!("{:.2} %", progress)).size(30))
                .push(ProgressBar::new(0.0..=100.0, progress))
        } else {
            column
        };

        column.push(Space::new(Length::Shrink, Length::Fill)).into()
    }

    fn update(&mut self, message: &Message) -> Vec<Message> {
        match message {
            Message::GameLoadingMessage(loading_text, progress) => {
                self.loading_message = loading_text.to_string();
                self.progress = progress.clone();
            }
            _ => {}
        }

        vec![]
    }
}
