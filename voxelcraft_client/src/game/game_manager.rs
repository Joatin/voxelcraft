use crate::game::{Game, LocalGame};
use crate::gpu::primitives::SmallTexturedArrayVertex;
use crate::gpu::RenderContext;
use crate::interface::Message;
use crate::primitives::Size;
use std::error::Error;
use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::game::resources::GameResources;
use pollster::FutureExt;
use wgpu::{
    CommandBuffer, CompareFunction, DepthStencilState, Device, Queue, RenderPipeline, TextureFormat,
};
use winit::dpi::PhysicalSize;

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
}

impl GameManager {
    pub async fn new(
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        texture_format: TextureFormat,
        size: &PhysicalSize<u32>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let messages = vec![];
        let resources =
            GameResources::new(&queue, &device, texture_format, size.width, size.height).await?;

        Ok(Self {
            messages,
            game_join_handle: None,
            game: GameWrapper::None,
            device: Arc::clone(&device),
            resources,
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
        let local_game = LocalGame::new(device);
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

    pub fn on_mouse_moved(&mut self, x: f64, y: f64) {
        if x != 0.0 {
            self.resources.camera.increase_yaw(x, 0.1)
        }
        if y != 0.0 {
            self.resources.camera.increase_pitch(y, 0.1)
        }
        if let Some(game) = self.game.game_mut() {
            game.on_mouse_moved(x, y)
        }
    }
}
