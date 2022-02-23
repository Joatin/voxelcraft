use wgpu::{Device, TextureView, TextureFormat, CommandBuffer, RenderPipeline};
use wgpu_glyph::{ab_glyph, Section, Text, GlyphBrush, GlyphBrushBuilder};
use winit::dpi::PhysicalSize;
use wgpu::util::StagingBelt;
use std::collections::HashMap;
use crate::context::Context;
use crate::gpu::{Gpu, RenderContext};
use std::sync::Arc;
use pollster::FutureExt;
use crate::interface::debug_info::DebugInfo;
use crate::gpu::primitives::TexturedVertex;
use crate::interface::texture_map::TextureMap;
use crate::interface::pipeline_map::PipelineMap;
use crate::interface::screen_context::ScreenContext;
use crate::interface::{Page, HOME_ROUTE};
use crate::interface::pages::HomePage;

pub struct Screen {
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
    pages: HashMap<String, Box<dyn Page>>,
    debug_info: DebugInfo,
    pipeline_map: PipelineMap,
    texture_map: TextureMap
}

impl Screen {

    pub fn new(context: &Arc<Context>, state: &Gpu, pages: HashMap<String, Box<dyn Page>>) -> Self {
        let inconsolata = ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../gpu/Inconsolata-Regular.ttf"
        )).unwrap();

        let mut glyph_brush = GlyphBrushBuilder::using_font(inconsolata)
            .build(&state.device, state.render_format);

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let debug_info = DebugInfo::new(context);

        let mut pipeline_map = PipelineMap::new();
        pipeline_map.create_default_pipelines(&state);

        let mut texture_map = TextureMap::new();
        {
            let (_, texture_bind_group_layout) = pipeline_map.get("default:pipeline").expect("This pipeline is always created");
            texture_map.insert_default_textures(&state, &texture_bind_group_layout);
        }


        Self {
            glyph_brush,
            staging_belt,
            pages,
            debug_info,
            pipeline_map,
            texture_map
        }
    }


    fn get_current_page<'a>(pages: &'a mut HashMap<String, Box<dyn Page>>, context: &Arc<Context>) -> &'a mut Box<dyn Page> {
        let current_path = context.get_current_route();
        pages.get_mut(&current_path).unwrap()
    }

    pub fn render(&mut self, context: &Arc<Context>, render_context: &RenderContext) -> CommandBuffer {
        let mut encoder = render_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Interface Render"),
        });

        let mut screen_context = ScreenContext {
            encoder: &mut encoder,
            glyph_brush: &mut self.glyph_brush,
            texture_map: &self.texture_map,
            pipeline_map: &self.pipeline_map
        };

        let page = Self::get_current_page(&mut self.pages, &context);
        page.render(&render_context, &mut screen_context);
        if context.show_debug() {
            self.debug_info.render(&render_context, &mut screen_context);
        }

        let command_buffer = encoder.finish();
        command_buffer
    }

    pub fn cleanup(&mut self) {
        self.debug_info.cleanup();

        for (_, page) in &mut self.pages {
            page.cleanup()
        }

        tokio::spawn(self.staging_belt.recall());
    }

    pub fn default_pages(gpu: &Gpu) -> HashMap<String, Box<dyn Page>> {
        let mut map = HashMap::new();
        map.insert(HOME_ROUTE.to_string(), Box::new(HomePage::new(&gpu)) as Box<dyn Page>);
        map
    }
}