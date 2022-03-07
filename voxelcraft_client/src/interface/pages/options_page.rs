use crate::interface::page::Page;
use iced::{Element, Column, Length, Text};
use crate::interface::message::Message;

pub const OPTIONS_PAGE_ROUTE: &str = "OPTIONS";

pub struct OptionsPage {

}

impl OptionsPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Page for OptionsPage {
    fn name(&self) -> &str {
        OPTIONS_PAGE_ROUTE
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .width(Length::Fill)
            .push(Text::new("Options"))
            .into()
    }
}