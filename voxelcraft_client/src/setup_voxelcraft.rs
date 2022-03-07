use crate::window::Window;
use crate::context::Context;
use std::sync::Arc;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};
use log::LevelFilter;
use voxelcraft_mod::ModPack;
use crate::gpu::Gpu;
use crate::application_event_handler::ApplicationEventHandler;
use crate::interface::Interface;
use pollster::FutureExt;
use crate::game::GameManager;


#[tokio::main(flavor = "multi_thread")]
pub async fn setup_voxelcraft<T: ModPack>(mod_pack: T) {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    );


    let window = Window::create("Voxelcraft").unwrap();
    let gpu = Gpu::new(window.window()).await;
    let interface = Interface::new(window.window(), &gpu);
    let game_manager = GameManager::new(&gpu.device, &gpu.queue, gpu.render_format, &gpu.size).await.unwrap();

    let event_handler = ApplicationEventHandler::new(gpu, interface, game_manager);

    window.run(event_handler);
}