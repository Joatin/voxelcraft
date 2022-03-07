use iced::{Element, Command};
use crate::interface::message::Message;

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