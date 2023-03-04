use log::{info, debug};
use wgpu::{Instance, Surface, Adapter, Device, Queue};
use wgpu::util::DeviceExt;
use winit::{window::Window, event::WindowEvent, dpi::PhysicalSize};
use crate::{INDICES, Vertex, VERTICES};
use crate::graphics::Graphics;
use crate::pipeline::Pipeline;
use crate::texture::Texture;

pub(crate) struct Renderer {
    window: Window,
    pub graphics: Graphics,
    render_pipeline: Pipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: Texture,
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub(crate) async fn new(window: Window) -> Self {
        let graphics = Graphics::new(&window).await;

        let diffuse_bytes = include_bytes!("assets/tree.png");
        let diffuse_texture = Texture::from_bytes(&graphics.device, &graphics.queue, diffuse_bytes, "tree.png").unwrap();

        let texture_bind_group_layout =
            graphics.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = graphics.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler)
                    }
                ],
                label: Some("diffuse_bind_group")
            }
        );

        let render_pipeline_layout = graphics.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[]
        });

        let render_pipeline = Pipeline::new(
            &graphics,
            "Main",
            "Main",
            include_str!("shaders/shader.wgsl"),
            Vertex::init_buffer_layout(),
            Some(&render_pipeline_layout)
        );

        let vertex_buffer = graphics.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = graphics.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_vertices = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;

        Self {
            window,
            graphics,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture
        }
    }

    pub(crate) fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub(crate) fn update(&mut self) {
        
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.graphics.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
    
        // submit will accept anything that implements IntoIter
        self.graphics.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}