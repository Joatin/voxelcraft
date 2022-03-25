use crate::gpu::RenderContext;
use crate::interface::Message;
use crate::primitives::Size;
use std::fmt::Debug;

use crate::game::resources::GameResources;
use wgpu::CommandBuffer;

pub trait Game: Debug {
    fn update(&mut self);
    fn render(
        &mut self,
        render_context: &RenderContext,
        resource: &mut GameResources,
    ) -> Vec<CommandBuffer>;
    fn cleanup(&mut self);
    fn get_messages(&mut self) -> Vec<Message>;
    fn resize(&mut self, size: Size);
}
