use crate::interface::page::Page;
use crate::interface::pages::in_game_menu::{InGameMenuPage, IN_GAME_MENU_PAGE_ROUTE};
use crate::interface::pages::{
    GameLoadingPage, InGameHUDPage, MainPage, OptionsPage, WorldSelectionPage,
    GAME_LOADING_PAGE_ROUTE, IN_GAME_HUD_PAGE_ROUTE, MAIN_PAGE_ROUTE, OPTIONS_PAGE_ROUTE,
    WORLD_SELECTION_PAGE_ROUTE,
};
use std::collections::HashMap;

pub fn get_default_pages() -> HashMap<String, Box<dyn Page>> {
    let mut pages = HashMap::new();
    pages.insert(
        MAIN_PAGE_ROUTE.to_string(),
        Box::new(MainPage::new()) as Box<dyn Page>,
    );
    pages.insert(
        WORLD_SELECTION_PAGE_ROUTE.to_string(),
        Box::new(WorldSelectionPage::new()),
    );
    pages.insert(OPTIONS_PAGE_ROUTE.to_string(), Box::new(OptionsPage::new()));
    pages.insert(
        GAME_LOADING_PAGE_ROUTE.to_string(),
        Box::new(GameLoadingPage::new()),
    );
    pages.insert(
        IN_GAME_HUD_PAGE_ROUTE.to_string(),
        Box::new(InGameHUDPage::new()),
    );
    pages.insert(
        IN_GAME_MENU_PAGE_ROUTE.to_string(),
        Box::new(InGameMenuPage::new()),
    );

    pages
}
