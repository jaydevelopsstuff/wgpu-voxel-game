use crate::coord::Coord3DF;
use crate::quad::Quad;
use crate::face::Face;

/**
 * A vector of quads that represent the faces of a block.
 */
pub struct BlockVector {
    faces: Vec<Quad>,
}

impl BlockVector {
    pub fn new(
        x: i32,
        y: i32,
        z: i32,
        textures: [Option<u32>; 6],
    ) -> Self {
        let xf: f32 = x as f32;
        let yf: f32 = y as f32;
        let zf: f32 = z as f32;

        let mut faces: Vec<Quad> = vec![];

        match textures[0] {
            Some(texture) => faces.push(Quad::new(Coord3DF::new(xf, yf, zf + 0.5), Face::Front, texture)),
            None => {}
        }

        match textures[1] {
            Some(texture) => faces.push(Quad::new(Coord3DF::new(xf, yf, zf + -0.5), Face::Back, texture)),
            None => {}
        }

        match textures[2] {
            Some(texture) => faces.push(Quad::new(Coord3DF::new(xf - 0.5, yf, zf), Face::Left, texture)),
            None => {}
        }

        match textures[3] {
            Some(texture) => faces.push(Quad::new(Coord3DF::new(xf + 0.5, yf, zf), Face::Right, texture)),
            None => {}
        }

        match textures[4] {
            Some(texture) => faces.push(Quad::new(Coord3DF::new(xf, yf + 0.5, zf), Face::Up, texture)),
            None => {}
        }

        match textures[5] {
            Some(texture) => faces.push(Quad::new(Coord3DF::new(xf, yf -0.5, zf), Face::Down, texture)),
            None => {}
        }

        Self {
            faces
        }
    }

    pub fn get_faces(&self) -> &Vec<Quad> {
        &self.faces
    }
}