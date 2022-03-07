use std::fmt::Debug;
use std::sync::Arc;
use crate::context::Context;
use crate::gpu::RenderContext;
use wgpu::CommandBuffer;
use crate::interface::Message;
use crate::primitives::Size;
use crate::game::camera_pipeline_utils::CameraPipelineUtils;

pub trait Game: Debug {
    fn update(&mut self);
    fn render(&mut self, render_context: &RenderContext) -> Vec<CommandBuffer>;
    fn cleanup(&mut self);
    fn get_messages(&mut self) -> Vec<Message>;
    fn resize(&mut self, size: Size);
    fn on_mouse_moved(&mut self, x: f64, y: f64);
}