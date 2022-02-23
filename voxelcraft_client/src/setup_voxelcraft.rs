use crate::window::Window;
use crate::context::Context;
use std::sync::Arc;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};
use log::LevelFilter;
use voxelcraft_mod::ModPack;


pub async fn setup_voxelcraft<T: ModPack>(mod_pack: T) {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    );

    let context = Arc::new(Context::new());

    let window = Window::new(&context).await.unwrap();

    window.run(context).await;
}