use std::fmt::Debug;
use std::sync::Arc;
use crate::context::Context;
use crate::gpu::RenderContext;

pub trait Game: Debug {
    fn update(&self);
    fn render(&self, context: &Arc<Context>, render_context: &RenderContext);
    fn cleanup(&self);
}