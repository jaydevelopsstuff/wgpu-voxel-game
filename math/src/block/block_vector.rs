use crate::point::{Coord3DF, Coord3DI};
use crate::face::Face;
use crate::quad::Quad;

const OFFSETS: [[f32; 3]; 6] = [
    [0., 0.5, 0.],
    [0., -0.5, 0.],
    [-0.5, 0., 0.],
    [0.5, 0., 0.],
    [0., 0., 0.5],
    [0., 0., -0.5]];

fn get_offset(face: Face) -> [f32; 3] {
    OFFSETS[face as usize]
}

fn create_quad(x: f32, y: f32, z: f32, face: Face, texture: u32) -> Quad {
    let offset = get_offset(face);
    let pos = Coord3DF::new(x + offset[0], y + offset[1], z + offset[2]);
    Quad::new(pos, face, texture)
}

/**
 * A vector of quads that represent the faces of a block.
 */
pub struct BlockVector {
    pos: Coord3DI,
    faces: Vec<Quad>,
}

impl BlockVector {
    pub fn new(
        pos: Coord3DI,
        textures: [Option<u32>; 6],
    ) -> Self {
        let xf: f32 = pos.x as f32;
        let yf: f32 = pos.y as f32;
        let zf: f32 = pos.z as f32;

        let mut faces: Vec<Quad> = vec![];

        match textures[Face::Up as usize] {
            Some(texture) => faces.push(create_quad(xf, yf, zf, Face::Up, texture)),
            None => {}
        }

        match textures[Face::Down as usize] {
            Some(texture) => faces.push(create_quad(xf, yf, zf, Face::Down, texture)),
            None => {}
        }

        match textures[Face::Left as usize] {
            Some(texture) => faces.push(create_quad(xf, yf, zf, Face::Left, texture)),
            None => {}
        }

        match textures[Face::Right as usize] {
            Some(texture) => faces.push(create_quad(xf, yf, zf, Face::Right, texture)),
            None => {}
        }

        match textures[Face::Front as usize] {
            Some(texture) => faces.push(create_quad(xf, yf, zf, Face::Front, texture)),
            None => {}
        }

        match textures[Face::Back as usize] {
            Some(texture) => faces.push(create_quad(xf, yf, zf, Face::Back, texture)),
            None => {}
        }

        Self {
            pos,
            faces,
        }
    }

    pub fn set_face(&mut self, face: Face, texture: Option<u32>) {
        match texture {
            Some(texture) => {
                let mut set = false;

                self.faces.iter_mut().for_each(|f| {
                    if f.facing == face {
                        f.set_texture_index(texture);
                        set = true;
                    }
                });

                if !set {
                    self.faces.push(create_quad(self.pos.x as f32, self.pos.y as f32, self.pos.z as f32, face, texture));
                }
            }
            None => {
                self.faces.retain(|f| f.facing != face);
            }
        }
    }

    pub fn get_faces(&self) -> &Vec<Quad> {
        &self.faces
    }
}