use crate::gpu::primitives::SmallTexturedArrayVertex;
use wgpu::{
    BindGroupLayout, CompareFunction, DepthStencilState, Device, IndexFormat, RenderPipeline,
    TextureFormat,
};
use wgpu_tokio::DeviceAsyncExt;

pub async fn construct_block_pipeline(
    device: &Device,
    texture_format: TextureFormat,
    bind_group_layout: &BindGroupLayout,
    camera_bindgroup_layout: &BindGroupLayout,
) -> RenderPipeline {
    let render_pipeline_layout = device
        .create_pipeline_layout_async(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout, camera_bindgroup_layout],
            push_constant_ranges: &[],
        })
        .await;

    let shader = device
        .create_shader_module_async(&wgpu::include_wgsl!("../shaders/block_shader.wgsl"))
        .await;

    device
        .create_render_pipeline_async(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[SmallTexturedArrayVertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
        .await
}
