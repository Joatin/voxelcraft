use crate::chunk::{ChunkMesh, MeshableChunk};
use crate::context::Context;
use crate::game::block_texture_map::BlockTextureMap;
use crate::game::camera_pipeline_utils::CameraPipelineUtils;
use crate::game::game::Game;
use crate::gpu::camera::{Camera, Projection};
use crate::gpu::RenderContext;
use crate::interface::{Message, IN_GAME_HUD_PAGE_ROUTE};
use crate::primitives::Size;
use futures::{stream, StreamExt, TryStreamExt};
use pollster::FutureExt;
use std::collections::HashMap;
use std::error::Error;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use uuid::Uuid;
use voxelcraft_server::client::Client;
use voxelcraft_server::local::new_local_world;
use voxelcraft_server::local::LocalClient;
use wgpu::util::DeviceExt;
use wgpu::{
    Buffer, CommandBuffer, CommandEncoderDescriptor, Device, RenderPassDescriptor, RenderPipeline,
};
use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub struct LocalGame {
    client: Arc<LocalClient>,
    messages: Arc<std::sync::Mutex<Vec<Message>>>,
    is_loading: Arc<AtomicBool>,
    device: Arc<Device>,
    chunk_meshes: Arc<Mutex<Vec<ChunkMesh>>>,
    pipelines: Arc<HashMap<String, RenderPipeline>>,
    block_texture_map: Arc<BlockTextureMap>,
    camera_utils: Arc<CameraPipelineUtils>,
    camera: Camera,
    projection: Projection,
}

impl LocalGame {
    pub fn new(
        device: Arc<Device>,
        pipelines: &Arc<HashMap<String, RenderPipeline>>,
        block_texture_map: &Arc<BlockTextureMap>,
        camera_utils: Arc<CameraPipelineUtils>,
    ) -> Self {
        let client = Arc::new(new_local_world(0, Uuid::new_v4()));
        let messages = Arc::new(std::sync::Mutex::new(vec![]));
        let chunk_meshes = Arc::new(Mutex::new(vec![]));
        let is_loading = Arc::new(AtomicBool::new(true));
        let camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = Projection::new(800, 600, cgmath::Deg(45.0), 0.1, 100.0);

        tokio::spawn(Self::process_events(Arc::clone(&client)));
        tokio::spawn(Self::start_connection_process(
            Arc::clone(&client),
            Arc::clone(&device),
            Arc::clone(&chunk_meshes),
            Arc::clone(&messages),
            Arc::clone(&is_loading),
        ));

        Self {
            client,
            messages,
            is_loading,
            device,
            chunk_meshes,
            pipelines: Arc::clone(&pipelines),
            block_texture_map: Arc::clone(&block_texture_map),
            camera_utils,
            camera,
            projection,
        }
    }

    async fn start_connection_process(
        client: Arc<LocalClient>,
        device: Arc<Device>,
        chunk_meshes: Arc<Mutex<Vec<ChunkMesh>>>,
        messages: Arc<std::sync::Mutex<Vec<Message>>>,
        is_loading: Arc<AtomicBool>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Self::send_loading_message(&messages, "Preparing world", None);

        client.begin_joining_world().await;

        let player_position = client.position().await;

        let chunks_to_mesh = player_position.surrounding_chunks(1);
        let chunks_to_mesh_count = chunks_to_mesh.len();

        Self::send_loading_message(&messages, "Building chunks in the player vicinity", None);

        log::info!("Meshing chunks");
        let instant_start = Instant::now();
        let count_processed = Arc::new(AtomicU32::new(0));
        let mut meshes = stream::iter(chunks_to_mesh)
            .map(|position| {
                let client = Arc::clone(&client);
                let device = Arc::clone(&device);
                let count_processed = Arc::clone(&count_processed);
                let messages = Arc::clone(&messages);

                async move {
                    let chunk = client.get_chunk(position).await?;
                    let handle = tokio::spawn(async move { chunk.create_mesh(&device).await });
                    let mesh = handle.await.unwrap();
                    let current_count = count_processed.fetch_add(1, Ordering::Relaxed);
                    let progress = (100.0 / chunks_to_mesh_count as f32) * current_count as f32;
                    Self::send_loading_message(
                        &messages,
                        "Building chunks in the player vicinity",
                        Some(progress),
                    );
                    Result::<_, Box<dyn Error + Send + Sync>>::Ok(mesh)
                }
            })
            .buffer_unordered(num_cpus::get() * 2)
            .try_collect::<Vec<_>>()
            .await?;

        log::info!(
            "Successfully meshed {} chunks in {:?}",
            chunks_to_mesh_count,
            Instant::now().duration_since(instant_start)
        );

        {
            let mut lock = chunk_meshes.lock().await;
            lock.append(&mut meshes);
        }

        Self::finnish_loading(&messages);
        is_loading.store(false, Ordering::Relaxed);

        Ok(())
    }

    fn send_loading_message(
        messages: &Arc<std::sync::Mutex<Vec<Message>>>,
        message: &str,
        progress: Option<f32>,
    ) {
        log::info!("Sending loading message: '{}'", message);
        {
            let mut lock = messages.lock().unwrap();
            lock.push(Message::GameLoadingMessage(message.to_string(), progress))
        }
    }

    fn finnish_loading(messages: &Arc<std::sync::Mutex<Vec<Message>>>) {
        {
            let mut lock = messages.lock().unwrap();
            lock.push(Message::GameLoadingMessage("".to_string(), None));
            lock.push(Message::Navigate {
                page: IN_GAME_HUD_PAGE_ROUTE.to_string(),
            });
        }
    }

    pub fn load() -> Self {
        todo!()
    }

    pub async fn process_events(
        client: Arc<LocalClient>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut receiver = client.get_world_event_receiver().await;
        loop {
            let _event = receiver.recv().await?;
        }
    }

    async fn render_chunk_meshes(
        chunk_meshes: Arc<Mutex<Vec<ChunkMesh>>>,
        pipelines: Arc<HashMap<String, RenderPipeline>>,
        block_texture_map: Arc<BlockTextureMap>,
        camera_utils: Arc<CameraPipelineUtils>,
        render_context: RenderContext,
    ) -> Vec<CommandBuffer> {
        let mut chunk_meshes = chunk_meshes.lock().await;

        let mut encoder = render_context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Chunk command encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Chunk render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &render_context.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(pipelines.get("BLOCK").unwrap());
            render_pass.set_bind_group(0, block_texture_map.bind_group(), &[]);
            render_pass.set_bind_group(1, camera_utils.bind_group(), &[]);

            chunk_meshes.iter_mut().for_each(|mesh| {
                mesh.render(&render_context, &mut render_pass);
            });
        }

        vec![encoder.finish()]
    }
}

impl Game for LocalGame {
    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self, render_context: &RenderContext) -> Vec<CommandBuffer> {
        if !self.is_loading.load(Ordering::Relaxed) {
            let render_context = render_context.clone();
            let chunk_meshes = Arc::clone(&self.chunk_meshes);
            let pipelines = Arc::clone(&self.pipelines);
            let block_texture_map = Arc::clone(&self.block_texture_map);
            let camera_utils = Arc::clone(&self.camera_utils);

            let camera = self.camera.clone();
            let projection = self.projection.clone();

            let handle = tokio::spawn(async move {
                let mut command_buffers = {
                    let mut encoder =
                        render_context
                            .device
                            .create_command_encoder(&CommandEncoderDescriptor {
                                label: Some("Camera Uniform Encoder"),
                            });
                    camera_utils
                        .update(&render_context.device, &mut encoder, &camera, &projection)
                        .await;

                    vec![encoder.finish()]
                };

                command_buffers.append(
                    &mut Self::render_chunk_meshes(
                        chunk_meshes,
                        pipelines,
                        block_texture_map,
                        camera_utils,
                        render_context,
                    )
                    .await,
                );

                command_buffers
            });
            handle.block_on().unwrap()
        } else {
            vec![]
        }
    }

    fn cleanup(&mut self) {
        self.camera_utils.cleanup()
    }

    fn get_messages(&mut self) -> Vec<Message> {
        let mut list = self.messages.lock().unwrap();
        let messages = list.clone();
        list.clear();
        messages
    }

    fn resize(&mut self, size: Size) {
        self.projection
            .resize(size.width as u32, size.height as u32)
    }

    fn on_mouse_moved(&mut self, x: f64, y: f64) {
        // self.camera.
        self.camera.rotate(x, y);
    }
}
