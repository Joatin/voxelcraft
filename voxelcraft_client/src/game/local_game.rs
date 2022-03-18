use crate::chunk::ChunkMesh;
use crate::game::game::Game;
use crate::gpu::RenderContext;
use crate::interface::{Message, IN_GAME_HUD_PAGE_ROUTE};
use crate::primitives::Size;
use futures::{stream, StreamExt, TryStreamExt};
use std::error::Error;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use uuid::Uuid;
use voxelcraft_server::client::Client;
use voxelcraft_server::local::new_local_world;
use voxelcraft_server::local::LocalClient;

use crate::game::resources::GameResources;
use iced_wgpu::wgpu::CommandEncoder;
use wgpu::{
    CommandBuffer, CommandEncoderDescriptor, Device, Operations, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, RenderPipeline,
};

#[derive(Debug)]
pub struct LocalGame {
    client: Arc<LocalClient>,
    messages: Arc<std::sync::Mutex<Vec<Message>>>,
    is_loading: Arc<AtomicBool>,
    device: Arc<Device>,
    chunk_meshes: Arc<Mutex<Vec<ChunkMesh>>>,
}

impl LocalGame {
    pub fn new(device: Arc<Device>) -> Self {
        let client = Arc::new(new_local_world(0, Uuid::new_v4()));
        let messages = Arc::new(std::sync::Mutex::new(vec![]));
        let chunk_meshes = Arc::new(Mutex::new(vec![]));
        let is_loading = Arc::new(AtomicBool::new(true));

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

        let chunks_to_mesh = player_position.surrounding_chunks(6);
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
                    let handle = tokio::spawn(async move {
                        client
                            .get_chunk(position, |chunk| async move {
                                ChunkMesh::new(&device, &chunk, &position).await
                            })
                            .await
                    });
                    let mesh = handle.await???;
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

    fn render_chunk_meshes(
        &self,
        render_context: &RenderContext,
        resources: &mut GameResources,
        encoder: &mut CommandEncoder,
    ) {
        let mut chunk_meshes = self.chunk_meshes.blocking_lock();

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Chunk render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &render_context.view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: resources.geometry_buffer.depth_view(),
                depth_ops: Some(Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&resources.block_pipeline);
        render_pass.set_bind_group(0, resources.face_texture_map.bind_group(), &[]);
        render_pass.set_bind_group(1, resources.camera.bind_group(), &[]);

        chunk_meshes.iter_mut().for_each(|mesh| {
            mesh.render(&render_context, &mut render_pass);
        });
    }
}

impl Game for LocalGame {
    fn update(&mut self) {
        todo!()
    }

    fn render(
        &mut self,
        render_context: &RenderContext,
        resources: &mut GameResources,
    ) -> Vec<CommandBuffer> {
        if !self.is_loading.load(Ordering::Relaxed) {
            let mut encoder =
                render_context
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("Camera Uniform Encoder"),
                    });

            resources
                .camera
                .write_camera(&render_context.device, &mut encoder);

            self.render_chunk_meshes(render_context, resources, &mut encoder);

            vec![encoder.finish()]
        } else {
            vec![]
        }
    }

    fn cleanup(&mut self) {}

    fn get_messages(&mut self) -> Vec<Message> {
        let mut list = self.messages.lock().unwrap();
        let messages = list.clone();
        list.clear();
        messages
    }

    fn resize(&mut self, size: Size) {}

    fn on_mouse_moved(&mut self, x: f64, y: f64) {
        // self.camera.
    }
}
