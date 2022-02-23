use wgpu::CommandEncoder;
use wgpu_glyph::GlyphBrush;
use crate::interface::pipeline_map::PipelineMap;
use crate::interface::texture_map::TextureMap;

pub struct ScreenContext<'a> {
    pub encoder: &'a mut CommandEncoder,
    pub glyph_brush: &'a mut GlyphBrush<()>,
    pub texture_map: &'a TextureMap,
    pub pipeline_map: &'a PipelineMap,
}