use nalgebra::Rotation3;

use crate::coord::Point3DF;
use crate::face::Face;

#[derive(Clone, Debug)]
pub struct Quad {
    pub facing: Face,
    pub pos: Point3DF,
    pub rot: Rotation3<f32>,
    pub texture_index: u32,
}

impl Quad {
    pub fn new(position: Point3DF, facing: Face, texture_index: u32) -> Self {
        let rotation: Rotation3<f32>;
        match facing {
            Face::Up => {
                let rot: f32 = -90.;
                rotation = Rotation3::from_euler_angles(rot.to_radians(), 0., 0.);
            }
            Face::Down => {
                let rot: f32 = 90.;
                rotation = Rotation3::from_euler_angles(rot.to_radians(), 0., 0.);
            }
            Face::Left => {
                let rot: f32 = -90.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
            Face::Right => {
                let rot: f32 = 90.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
            Face::Front => {
                rotation = Rotation3::from_euler_angles(0., 0., 0.);
            }
            Face::Back => {
                let rot: f32 = 180.;
                rotation = Rotation3::from_euler_angles(0., rot.to_radians(), 0.);
            }
        }
        Quad {
            facing,
            pos: position,
            rot: rotation,
            texture_index,
        }
    }

    pub fn set_texture_index(&mut self, texture_index: u32) {
        self.texture_index = texture_index;
    }
}