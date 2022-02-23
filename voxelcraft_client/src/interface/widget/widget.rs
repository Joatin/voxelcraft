use crate::gpu::RenderContext;
use wgpu_glyph::GlyphBrush;
use wgpu::CommandEncoder;
use crate::interface::texture_map::TextureMap;
use crate::interface::screen_context::ScreenContext;

#[async_trait::async_trait]
pub trait Widget {


    fn widgets(&mut self) -> Vec<&mut dyn Widget>;

    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn render(&mut self, render_context: &RenderContext, screen_context: &mut ScreenContext, offset: (f32, f32)) {
        let x = self.x();
        let y = self.y();

        let mut widgets = self.widgets();
        for mut widget in widgets {
            widget.render(render_context, screen_context, (x + offset.0, y + offset.1))
        }
    }

    fn cleanup(&mut self) {
        let widgets = self.widgets();
        for mut widget in widgets {
            widget.cleanup()
        }
    }
}