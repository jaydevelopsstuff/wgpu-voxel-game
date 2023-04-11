use math::block::block_vector::BlockVector;
use math::coord::Coord3DI;
use crate::material::Material;

pub struct Block {
    pub pos: Coord3DI,
    pub material: Material,
}

impl Block {
    pub fn to_vector(&self, faces: [bool; 6]) -> BlockVector {
        self.material.to_vector(self.pos, faces)
    }
}