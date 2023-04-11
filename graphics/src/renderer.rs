use std::num::NonZeroU32;
use std::time::Duration;

use wgpu::BindingResource::TextureViewArray;
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};
use winit::event::{ElementState, KeyboardInput, MouseButton};

use math::block::block_map::BlockMap;
use math::block::block_vector::BlockVector;
use math::coord::{Coord2DI, Coord3DI};
use math::seed::Seed;
use world::chunk::{Chunk, ChunkGenerator, VanillaGenerator};

use crate::camera::{Camera, CameraUniform};
use crate::graphics::Graphics;
use crate::pipeline::Pipeline;
use crate::quad;
use crate::quad::Raw;
use crate::texture::Texture;
use crate::Vertex;

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
    depth_texture: Texture,
    pub camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    block_map: BlockMap,
    instance_buffer: wgpu::Buffer,
    pub(crate) mouse_pressed: bool,
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub(crate) async fn new(window: Window) -> Self {
        let graphics = Graphics::new(&window).await;

        let grass_side_diffuse_texture = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("assets/grass_block_side.png"), "grass_block_side.png").unwrap();
        let grass_top_diffuse_texture = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("assets/grass_block_top.png"), "grass_block_top.png").unwrap();
        let dirt_diffuse_texture = Texture::from_bytes(&graphics.device, &graphics.queue, include_bytes!("assets/dirt.png"), "dirt.png").unwrap();

        let depth_texture = Texture::create_depth_texture(&graphics.device, &graphics.config, "depth_texture");

        let camera = Camera::new(&graphics);
        let (camera_uniform, camera_buffer, camera_bind_group, camera_bind_group_layout) = camera.bind(&graphics);

        let mut block_map: BlockMap = BlockMap::new();

        let chunk: Chunk = VanillaGenerator::new(Seed::random().get()).generate_chunk(Coord2DI::new(0, 0));

        for block in chunk.blocks {
            block_map.push(block.to_vector([true, true, true, true, true, true]));
        }

        let mut instance_data = vec![];

        for quad in block_map.quads() {
            instance_data.push(quad.to_raw());
        }

        let instance_buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let texture_bind_group_layout =
            graphics.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: NonZeroU32::new(3),
                    }
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = graphics.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&grass_side_diffuse_texture.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: TextureViewArray(vec![&grass_side_diffuse_texture.view, &grass_top_diffuse_texture.view, &dirt_diffuse_texture.view].as_ref()),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let render_pipeline_layout = graphics.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = Pipeline::new(
            &graphics,
            "Main",
            "Main",
            include_str!("shaders/shader.wgsl"),
            Vertex::init_buffer_layout(),
            Some(&render_pipeline_layout),
        );

        let vertex_buffer = graphics.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(quad::VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = graphics.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(quad::INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_vertices = quad::VERTICES.len() as u32;
        let num_indices = quad::INDICES.len() as u32;

        Self {
            window,
            graphics,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture: grass_side_diffuse_texture,
            depth_texture,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            block_map,
            instance_buffer,
            mouse_pressed: false,
        }
    }

    pub(crate) fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                },
                ..
            } => self.camera.controller.process_keyboard_input(state, Some(*key)),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.controller.process_mouse_wheel(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.graphics.size = new_size;
            self.graphics.config.width = new_size.width;
            self.graphics.config.height = new_size.height;
            self.camera.resize(&self.graphics);
            self.depth_texture = Texture::create_depth_texture(&self.graphics.device, &self.graphics.config, "depth_texture");
            self.graphics.surface.configure(&self.graphics.device, &self.graphics.config);
        }
    }

    pub(crate) fn update(&mut self, dt: Duration) {
        self.camera.update();
        self.camera_uniform.update_view_proj(&self.camera);
        self.graphics.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);

            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.block_map.quad_len() as _);
        }

        // submit will accept anything that implements IntoIter
        self.graphics.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}