use crate::application_event_handler::ApplicationEventHandler;

use crate::game::GameManager;
use crate::gpu::Gpu;
use crate::interface::Interface;
use crate::window::Window;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use std::sync::Arc;
use voxelcraft_mod::ModPack;

#[tokio::main(flavor = "multi_thread")]
pub async fn setup_voxelcraft<T: 'static + ModPack>(mod_pack: T) {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    log::info!("Starting game with modpack: '{}'", mod_pack.name());

    for module in mod_pack.mods() {
        log::info!("Using mod: '{}'", module.name())
    }

    let mod_pack = Arc::new(mod_pack);

    let window = Window::create("Voxelcraft").unwrap();
    let gpu = Gpu::new(window.window()).await;
    let interface = Interface::new(window.window(), &gpu);
    let game_manager = GameManager::new(
        &gpu.device,
        &gpu.queue,
        gpu.render_format,
        &gpu.size,
        mod_pack,
    )
    .await
    .unwrap();

    let event_handler = ApplicationEventHandler::new(gpu, interface, game_manager);

    window.run(event_handler);
}
