use crate::game::block_texture_map::BlockTextureMap;
use crate::game::camera_pipeline_utils::CameraPipelineUtils;
use crate::game::{Game, LocalGame};
use crate::gpu::primitives::TexturedArrayVertex;
use crate::gpu::RenderContext;
use crate::interface::Message;
use crate::primitives::Size;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::task::JoinHandle;
use voxelcraft_server::local::new_local_world;
use wgpu::{
    BindGroupLayout, CommandBuffer, Device, PipelineLayout, Queue, RenderPipeline, TextureFormat,
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
    pipelines: Arc<HashMap<String, RenderPipeline>>,
    block_texture_map: Arc<BlockTextureMap>,
    camera_utils: Arc<CameraPipelineUtils>,
}

impl GameManager {
    pub async fn new(
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        texture_format: TextureFormat,
        size: &PhysicalSize<u32>,
    ) -> Result<Self, Box<dyn Error>> {
        let messages = vec![];

        let block_texture_map = Arc::new(BlockTextureMap::new(device, queue).await?);

        let camera_utils = Arc::new(CameraPipelineUtils::new(device));

        let pipelines =
            Self::construct_pipelines(device, texture_format, &block_texture_map, &camera_utils);

        Ok(Self {
            messages,
            game_join_handle: None,
            game: GameWrapper::None,
            device: Arc::clone(&device),
            pipelines,
            block_texture_map,
            camera_utils,
        })
    }

    fn construct_pipelines(
        device: &Arc<Device>,
        texture_format: TextureFormat,
        block_texture_map: &Arc<BlockTextureMap>,
        camera_utils: &Arc<CameraPipelineUtils>,
    ) -> Arc<HashMap<String, RenderPipeline>> {
        let mut map = HashMap::new();

        map.insert(
            "BLOCK".to_string(),
            Self::construct_block_pipeline(
                device,
                texture_format,
                block_texture_map.bind_group_layout(),
                camera_utils.bind_group_layout(),
            ),
        );

        Arc::new(map)
    }

    fn construct_block_pipeline(
        device: &Arc<Device>,
        texture_format: TextureFormat,
        bind_group_layout: &BindGroupLayout,
        camera_bindgroup_layout: &BindGroupLayout,
    ) -> RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[bind_group_layout, camera_bindgroup_layout],
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(&wgpu::include_wgsl!("shaders/block_shader.wgsl"));

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TexturedArrayVertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    pub fn render(&mut self, render_context: &RenderContext) -> Vec<CommandBuffer> {
        if let Some(game) = self.game.game_mut() {
            game.render(&render_context)
        } else {
            vec![]
        }
    }

    pub fn cleanup(&mut self) {
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
            &self.pipelines,
            &self.block_texture_map,
            Arc::clone(&self.camera_utils),
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
        if let Some(game) = self.game.game_mut() {
            game.resize(size)
        }
    }

    pub fn on_mouse_moved(&mut self, x: f64, y: f64) {
        if let Some(game) = self.game.game_mut() {
            game.on_mouse_moved(x, y)
        }
    }
}
