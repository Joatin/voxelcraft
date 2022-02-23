use crate::interface::widget::{TextWidget, Widget};
use crate::gpu::RenderContext;
use wgpu::CommandEncoder;
use wgpu_glyph::GlyphBrush;
use crate::context::Context;
use std::sync::Arc;
use tokio::time::Instant;
use std::time::Duration;
use crate::interface::texture_map::TextureMap;
use crate::interface::screen_context::ScreenContext;

pub struct DebugInfo {
    fps_text: TextWidget,
    render_duration_text: TextWidget,
    context: Arc<Context>,
    last_update_time: Instant
}

impl DebugInfo {

    pub fn new(context: &Arc<Context>) -> Self {
        Self {
            fps_text: TextWidget::new("", 20.0, 20.0, 30.0, [1.0, 1.0, 1.0, 1.0]),
            render_duration_text: TextWidget::new("", 20.0, 45.0, 30.0, [1.0, 1.0, 1.0, 1.0]),
            context: Arc::clone(&context),
            last_update_time: Instant::now()
        }
    }

    pub fn render(&mut self, render_context: &RenderContext, screen_context: &mut ScreenContext) {
        if Instant::now().duration_since(self.last_update_time) > (Duration::from_millis(100)) {
            self.fps_text.set_text(&self.format_fps_text());
            self.render_duration_text.set_text(&self.format_render_duration_text());
            self.last_update_time = Instant::now();
        }



        self.fps_text.render(render_context, screen_context, (0.0, 0.0));
        self.render_duration_text.render(render_context, screen_context, (0.0, 0.0));
    }

    pub fn cleanup(&mut self) {
        self.fps_text.cleanup()
    }

    fn format_render_duration_text(&self) -> String {
        format!("Draw time: {:?}", self.context.get_time_to_draw_frame())
    }

    fn format_fps_text(&self) -> String {
        format!("FPS: {:.2}", self.context.get_current_fps())
    }
}