use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::{BindGroup, BindGroupLayout, Buffer};
use wgpu::util::DeviceExt;
use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode};
use crate::graphics::Graphics;

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    up: Vector3<f32>,
    pub aspect: f32,
    pub fov: f32,
    near: f32,
    far: f32,
    pub controller: CameraController,
    pub global_matrix: Matrix4<f32>,
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera {
    pub fn new(graphics: &Graphics) -> Self {
        let controller = CameraController::new();

        Self {
            eye: Point3::new(0., 0., 1.),
            target: Point3::new(0., 0., -1.),
            up: Vector3::y(),
            aspect: graphics.config.width as f32 / graphics.config.height as f32,
            fov: 60.0,
            near: 0.01,
            far: 100.0,
            controller,
            global_matrix: OPENGL_TO_WGPU_MATRIX,
        }
    }

    pub(crate) fn bind(&self, graphics: &Graphics) -> (CameraUniform, Buffer, BindGroup, BindGroupLayout) {
        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(&self);

        let buffer = graphics.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = graphics.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let bind_group = graphics.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        (uniform, buffer, bind_group, bind_group_layout)
    }

    pub fn update_global_matrix(&mut self) {
        let target = Point3::new(
            self.eye.x + self.target.x,
            self.eye.y + self.target.y,
            self.eye.z + self.target.z,
        );
        let projection =
            Matrix4::new_perspective(self.aspect, self.fov.to_degrees(), self.near, self.far);
        let view = Matrix4::look_at_rh(&self.eye, &target, &self.up);
        self.global_matrix = OPENGL_TO_WGPU_MATRIX * projection * view;
    }

    pub fn resize(&mut self, graphics: &Graphics) {
        self.aspect = graphics.config.width as f32 / graphics.config.height as f32;
    }

    pub fn update(&mut self) {
        self.fov += self.controller.fov_delta;
        self.controller.fov_delta = 0.;
        self.target = Point3::new(
            self.controller.yaw.to_radians().cos() * self.controller.pitch.to_radians().cos(),
            self.controller.pitch.to_radians().sin(),
            self.controller.yaw.to_radians().sin() * self.controller.pitch.to_radians().cos(),
        );
        let target = Vector3::new(self.target.x, 0.0, self.target.z).normalize();
        self.eye +=
            &target * self.controller.speed * (self.controller.forward - self.controller.backward);
        self.eye += &target.cross(&self.up)
            * self.controller.speed
            * (self.controller.right - self.controller.left);
        self.eye += Vector3::new(0.0, 1.0, 0.0)
            * self.controller.speed
            * (self.controller.up - self.controller.down);
        self.update_global_matrix();
    }
}

pub struct CameraController {
    speed: f32,
    sensitivity: f64,
    forward: f32,
    backward: f32,
    left: f32,
    right: f32,
    up: f32,
    down: f32,
    pub yaw: f32,
    pub pitch: f32,
    fov_delta: f32,
}

impl CameraController {
    pub fn new() -> Self {
        CameraController {
            speed: 0.2,
            sensitivity: 0.1,
            forward: 0.,
            backward: 0.,
            left: 0.,
            right: 0.,
            up: 0.,
            down: 0.,
            yaw: 270.0,
            pitch: 0.0,
            fov_delta: 0.,
        }
    }

    pub fn process_keyboard_input(&mut self, state: &ElementState, virtual_keycode: Option<VirtualKeyCode>) -> bool {
        let value: f32;
        if *state == ElementState::Pressed {
            value = 1.0;
        } else {
            value = 0.0;
        }
        match virtual_keycode.unwrap() {
            VirtualKeyCode::Space => {
                self.up = value;
            }
            VirtualKeyCode::LShift => {
                self.down = value;
            }
            VirtualKeyCode::W => {
                self.forward = value;
            }
            VirtualKeyCode::S => {
                self.backward = value;
            }
            VirtualKeyCode::A => {
                self.left = value;
            }
            VirtualKeyCode::D => {
                self.right = value;
            }
            _ => (),
        }
        true
    }

    pub fn process_mouse_motion(&mut self, delta: &(f64, f64)) {
        self.yaw += (delta.0 * self.sensitivity) as f32;
        self.pitch -= (delta.1 * self.sensitivity) as f32;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        } else if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        if self.yaw > 360.0 {
            self.yaw = 0.0;
        } else if self.yaw < 0.0 {
            self.yaw = 360.0;
        }
    }

    pub fn process_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        self.fov_delta = match delta {
            MouseScrollDelta::LineDelta(_, scroll) => *scroll,
            MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition { y, .. }) => {
                *y as f32
            }
        }
    }
}

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.global_matrix.into();
    }
}
