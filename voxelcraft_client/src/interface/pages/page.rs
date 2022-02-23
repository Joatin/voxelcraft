use wgpu_glyph::GlyphBrush;
use wgpu::CommandEncoder;
use crate::gpu::RenderContext;
use crate::interface::screen_context::ScreenContext;
use crate::interface::widget::Widget;

pub trait Page {

    fn widgets(&mut self) -> Vec<&mut dyn Widget>;
    fn render(&mut self, render_context: &RenderContext, screen_context: &mut ScreenContext) {
        let mut widgets = self.widgets();
        for mut widget in widgets {
            widget.render(render_context, screen_context, (0.0, 0.0))
        }
    }

    fn cleanup(&mut self) {
        let widgets = self.widgets();
        for mut widget in widgets {
            widget.cleanup()
        }
    }
}