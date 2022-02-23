pub use self::pages::Page;
pub use self::screen::Screen;

mod screen;
pub mod widget;
mod debug_info;
mod texture_map;
mod pipeline_map;
mod screen_context;
mod pages;

pub const HOME_ROUTE: &str = "HOME";

