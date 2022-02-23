use crate::interface::widget::{TextWidget, Widget};
use crate::gpu::{Gpu, RenderContext};
use crate::interface::widget::image_widget::ImageWidget;
use wgpu::CommandEncoder;
use wgpu_glyph::GlyphBrush;
use crate::interface::texture_map::TextureMap;
use crate::interface::screen_context::ScreenContext;

pub struct ButtonWidget {
    text: TextWidget,
    background: ImageWidget,
    x: f32,
    y: f32,
    scale: f32,
}

impl ButtonWidget {
    pub fn new(gpu: &Gpu, text: &str, x: f32, y: f32, scale: f32) -> Self {
        Self {
            background: ImageWidget::new(gpu, "default:button", 0.0, 0.0, 400.0, 100.0),
            text: TextWidget::new(text, 100.0, 30.0, 40.0, [1.0, 1.0, 1.0, 1.0]),
            x,
            y,
            scale
        }
    }
}

impl Widget for ButtonWidget {
    fn widgets(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            &mut self.background,
            &mut self.text,
        ]
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}