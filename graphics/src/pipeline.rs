use crate::graphics::Graphics;

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline
}

impl Pipeline {
    pub fn new(
        graphics: &Graphics,
        label: &str,
        shader_label: &str,
        shader_content: &str,
        vertex_layout: wgpu::VertexBufferLayout,
        layout: Option<&wgpu::PipelineLayout>
    ) -> Self {
        let shader = graphics.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{} shader", shader_label)),
            source: wgpu::ShaderSource::Wgsl(shader_content.into()),
        });

        let pipeline = graphics.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{} Render Pipeline", label)),
            layout,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_layout]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: graphics.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),
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
                conservative: false
            },
            depth_stencil: None, // Is this OK?
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });

        Self {
            pipeline
        }
    }
}