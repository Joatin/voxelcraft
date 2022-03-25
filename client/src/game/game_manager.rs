use crate::game::{Game, LocalGame};
use crate::gpu::primitives::SmallTexturedArrayVertex;
use crate::gpu::RenderContext;
use crate::interface::Message;
use crate::primitives::Size;
use std::error::Error;
use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::game::resources::GameResources;
use crate::input::InputManager;
use pollster::FutureExt;
use voxelcraft_mod::ModPack;
use wgpu::{
    CommandBuffer, CompareFunction, DepthStencilState, Device, Queue, RenderPipeline, TextureFormat,
};
use winit::dpi::PhysicalSize;
use winit::event::ElementState;

enum GameWrapper {
    Local(LocalGame),
    None,
}

impl GameWrapper {
    fn game(&self) -> Option<&dyn Game> {
        match &self {
            Self::Local(game) => Some(game),
            Self::None => None,
        }
    }
    fn game_mut(&mut self) -> Option<&mut dyn Game> {
        match self {
            Self::Local(game) => Some(game),
            Self::None => None,
        }
    }
}

pub struct GameManager {
    messages: Vec<Message>,
    game_join_handle: Option<JoinHandle<()>>,
    game: GameWrapper,
    device: Arc<Device>,
    resources: GameResources,
    input_manager: Arc<InputManager>,
    mod_pack: Arc<dyn ModPack>,
}

impl GameManager {
    pub async fn new(
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        texture_format: TextureFormat,
        size: &PhysicalSize<u32>,
        mod_pack: Arc<dyn ModPack>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let messages = vec![];

        let resources = GameResources::new(
            &queue,
            &device,
            texture_format,
            size.width,
            size.height,
            &mod_pack,
        )
        .await?;

        let input_manager = Arc::new(InputManager::new());

        Ok(Self {
            messages,
            game_join_handle: None,
            game: GameWrapper::None,
            device: Arc::clone(&device),
            resources,
            input_manager,
            mod_pack,
        })
    }

    pub fn render(&mut self, render_context: &RenderContext) -> Vec<CommandBuffer> {
        if let Some(game) = self.game.game_mut() {
            game.render(&render_context, &mut self.resources)
        } else {
            vec![]
        }
    }

    pub fn cleanup(&mut self) {
        self.resources.cleanup();
        if let Some(game) = self.game.game_mut() {
            game.cleanup()
        }
    }

    pub fn process_message(&mut self, message: &Message) {
        match message {
            Message::CreateNewGame => {
                let device = Arc::clone(&self.device);
                self.create_new_world(device)
            }
            _ => {}
        }
    }

    fn create_new_world(&mut self, device: Arc<Device>) {
        log::info!("Creating a new world!");
        self.messages.push(Message::GameLoadingMessage(
            "Creating new game".to_string(),
            None,
        ));
        let local_game = LocalGame::new(
            device,
            &self.input_manager,
            &self.mod_pack,
            &self.resources.face_texture_map,
            &self.resources.blocks,
        );
        self.game = GameWrapper::Local(local_game)
    }

    pub fn get_messages(&mut self) -> Vec<Message> {
        let mut messages = self.messages.clone();
        if let Some(game) = self.game.game_mut() {
            messages.append(&mut game.get_messages())
        }
        self.messages.clear();
        messages
    }

    pub fn resize(&mut self, size: Size) {
        self.resources
            .resize(&self.device, size.width as u32, size.height as u32)
            .block_on();
        if let Some(game) = self.game.game_mut() {
            game.resize(size)
        }
    }

    pub fn on_mouse_moved(&mut self, delta_x: f64, delta_y: f64) {
        self.input_manager.on_mouse_moved(delta_x, delta_y)
    }

    pub fn on_keyboard_input(&mut self, state: ElementState, scancode: u32) {
        self.input_manager.on_keyboard_input(state, scancode);
    }
}
