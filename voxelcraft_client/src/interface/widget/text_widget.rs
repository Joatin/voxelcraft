use crate::interface::widget::Widget;
use wgpu::util::StagingBelt;
use crate::gpu::RenderContext;
use wgpu_glyph::{GlyphBrush, Section, Text};
use wgpu::CommandEncoder;
use crate::interface::texture_map::TextureMap;
use crate::interface::screen_context::ScreenContext;

pub struct TextWidget {
    staging_belt: StagingBelt,
    text: String,
    x: f32,
    y: f32,
    scale: f32,
    color: [f32; 4]
}

impl TextWidget {
    pub fn new(text: &str, x: f32, y: f32, scale: f32, color: [f32; 4]) -> Self {

        Self {
            staging_belt: StagingBelt::new(1024),
            text: text.to_owned(),
            x,
            y,
            scale,
            color
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Widget for TextWidget {
    fn widgets(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn render(&mut self, render_context: &RenderContext, screen_context: &mut ScreenContext, offset: (f32, f32)) {
        screen_context.glyph_brush.queue(Section {
            screen_position: (self.x + offset.0, self.y + offset.1),
            bounds: (render_context.size.width as f32, render_context.size.height as f32),
            text: vec![Text::new(&self.text)
                .with_color(self.color)
                .with_scale(self.scale)],
            ..Section::default()
        });

        screen_context.glyph_brush
            .draw_queued(
                &render_context.device,
                &mut self.staging_belt,
                &mut screen_context.encoder,
                &render_context.view,
                render_context.size.width,
                render_context.size.height,
            )
            .expect("Draw queued");

        self.staging_belt.finish();
    }

    fn cleanup(&mut self) {
        tokio::spawn(self.staging_belt.recall());
    }
}