use wgpu::{Instance, Surface, Adapter, Device, Queue};
use winit::{window::Window, event::WindowEvent, dpi::PhysicalSize};

pub(crate) struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub(crate) async fn new(window: Window) -> Self {
        let size: PhysicalSize<u32>  = window.inner_size();

        let instance: Instance = State::create_instance();

        let surface: Surface = State::create_surface(&window, &instance);

        let adapter: Adapter = State::create_adapter(&instance, &surface).await;

        let (device, queue) = State::create_dev_queue(&adapter).await;

        let surface_caps = surface.get_capabilities(&adapter);
        
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    fn create_instance() -> Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        })
    }

    fn create_surface(window: &Window, instance: &Instance) -> Surface {
        unsafe { instance.create_surface(window) }.expect("Failed to create surface")
    }

    async fn create_adapter(instance: &Instance, surface: &Surface) -> Adapter {
        instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap()
    }

    async fn create_dev_queue(adapter: &Adapter) -> (Device, Queue) {
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default()
            },
            None,
        ).await.unwrap()
    }

    pub(crate) fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn size(&self) -> &winit::dpi::PhysicalSize<u32> {
        &self.size
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub(crate) fn update(&mut self) {
        
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}