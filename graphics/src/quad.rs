use nalgebra::{Rotation3, Translation3};
use crate::coord::{Coord3DF, Coord3DI};
use crate::instance::InstanceRaw;
use crate::vertex::Vertex;

pub(crate) const VERTICES: &[Vertex] = &[
    // TL
    Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0] },
    // TR
    Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0] },
    // BL
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
    // BR
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] }
];

pub(crate) const INDICES: &[u16] = &[
    0, 2, 3,
    3, 1, 0
];

#[derive(Debug)]
pub struct Quad {
    pub position: Translation3<f32>,
    pub(crate) rotation: Rotation3<f32>,
    pub(crate) texture_index: u32
}

impl Quad  {
    pub fn new(pos: Coord3DF, facing: Rotation, texture_index: u32) -> Self {
        let rotation: Rotation3<f32>;
        match facing {
            Rotation::Up => {
                let rot: f32 = -90.;
                rotation = Rotation3::from_euler_angles(rot.to_radians(), 0., 0.);
            }
            Rotation::Down => {
                let rot: f32 = 90.;
                rotation = Rotation3::from_euler_angles(rot.to_radians(), 0., 0.);
            }
            Rotation::Left => {
                let rot: f32 = -90.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
            Rotation::Right => {
                let rot: f32 = 90.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
            Rotation::Front => {
                rotation = Rotation3::from_euler_angles(0., 0., 0.);
            }
            Rotation::Back => {
                let rot: f32 = 180.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
        }
        Quad {
            position: Translation3::from([pos.x as f32, pos.y as f32, pos.z as f32]),
            rotation,
            texture_index,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let t_matrix: [[f32; 4]; 4] = (self.position.to_homogeneous() * self.rotation.matrix().to_homogeneous()).into();
        InstanceRaw {
            model: t_matrix,
            texture_index: self.texture_index,
        }
    }
}

pub enum Rotation {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back
}
