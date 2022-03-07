use crate::interface::message::Message;
use iced::{Command, Element};

pub trait Page {
    fn name(&self) -> &str;
    fn view(&mut self) -> Element<'_, Message>;
    fn update(&mut self, message: &Message) -> Vec<Message> {
        vec![]
    }
    fn should_grab_cursor(&self) -> bool {
        false
    }
}
