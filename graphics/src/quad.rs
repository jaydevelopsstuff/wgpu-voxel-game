use nalgebra::Translation3;
use math::quad::Quad;

use crate::instance::InstanceRaw;
use crate::vertex::Vertex;

pub const VERTICES: &[Vertex] = &[
    // TL
    Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0] },
    // TR
    Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0] },
    // BL
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
    // BR
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] }
];

pub const INDICES: &[u16] = &[
    0, 2, 3,
    3, 1, 0
];

pub trait Raw {
    fn to_raw(&self) -> InstanceRaw;
}

impl Raw for Quad {
    fn to_raw(&self) -> InstanceRaw {
        let translation = Translation3::from([self.pos.x.clone(), self.pos.y.clone(), self.pos.z.clone()]);
        let t_matrix: [[f32; 4]; 4] = (translation.to_homogeneous() * self.rot.matrix().to_homogeneous()).into();
        InstanceRaw {
            model: t_matrix,
            texture_index: self.texture_index,
        }
    }
}