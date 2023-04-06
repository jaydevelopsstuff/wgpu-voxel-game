use crate::block::block_vector::BlockVector;

pub struct BlockMap {
    vectors: Vec<BlockVector>,
}

impl BlockMap {
    pub fn new() -> Self {
        Self {
            vectors: vec![]
        }
    }

    pub fn push(&mut self, vector: BlockVector) {
        self.vectors.push(vector);
    }

    pub fn get(&self, index: usize) -> &BlockVector {
        &self.vectors[index]
    }

    pub fn replace(&mut self, vector: BlockVector, index: usize) {
        self.vectors[index] = vector;
    }

    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    pub fn quad_len(&self) -> usize {
        let mut len = 0;
        for vector in self.vectors.iter() {
            len += vector.get_faces().len();
        }
        len
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BlockVector> {
        self.vectors.iter()
    }
}