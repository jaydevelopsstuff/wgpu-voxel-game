use math::block::block_vector::BlockVector;
use math::coord::Coord3DI;
use math::face::Face;

pub struct Material {
    textures: [u32; 6]
}

impl Material {
    pub const fn new(textures: [u32; 6]) -> Self {
        Self {
            textures
        }
    }

    pub fn get_texture(&self, face: Face) -> u32 {
        self.textures[face as usize]
    }

    pub fn to_vector(&self, pos: Coord3DI, faces: [bool; 6]) -> BlockVector {
        let textures: [Option<u32>; 6] = [
            if faces[Face::Up as usize] { Some(self.get_texture(Face::Up)) } else { None },
            if faces[Face::Down as usize] { Some(self.get_texture(Face::Down)) } else { None },
            if faces[Face::Left as usize] { Some(self.get_texture(Face::Left)) } else { None },
            if faces[Face::Right as usize] { Some(self.get_texture(Face::Right)) } else { None },
            if faces[Face::Front as usize] { Some(self.get_texture(Face::Front)) } else { None },
            if faces[Face::Back as usize] { Some(self.get_texture(Face::Back)) } else { None },
        ];

        BlockVector::new(pos, textures)
    }
}

pub const GRASS: Material = Material::new([1, 2, 0, 0, 0, 0]);
pub const DIRT: Material = Material::new([2, 2, 2, 2, 2, 2]);