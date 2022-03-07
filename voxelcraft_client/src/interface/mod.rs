pub use self::interface::Interface;

mod interface;
mod router;
mod page;
mod message;
mod pages;
mod router_flags;
mod components;

pub use self::message::Message;
pub use self::pages::IN_GAME_HUD_PAGE_ROUTE;

