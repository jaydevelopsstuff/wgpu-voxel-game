use log::error;
use noise::{NoiseFn, Perlin};

use math::{CHUNK_HEIGHT, CHUNK_SIZE};
use math::block::block_vector::BlockVector;
use math::coord::{Coord2DI, Coord3DI, index};

use crate::block::Block;
use crate::material::{DIRT, GRASS};

pub struct Chunk {
    pub pos: Coord2DI,
    pub blocks: Vec<Block>,
}

impl Chunk {
    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }
}

pub trait ChunkGenerator {
    fn generate_chunk(&self, pos: Coord2DI) -> Chunk;
}

pub struct VanillaGenerator {
    noise: Vec<f64>,
}

impl VanillaGenerator {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let mut noise = Vec::new();

        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    noise.push(perlin.get([x as f64 * 0.01, y as f64 * 0.01, z as f64 * 0.01]));
                }
            }
        }

        Self {
            noise
        }
    }

    fn get_noise(&self, x: i32, y: i32, z: i32) -> &f64 {
        let i = index(&x, &y, &z);

        if i >= self.noise.len() {
            error!("Noise index out of bounds: {} {} {} -> {}", x, y, z, i);
            return &0.0;
        }

        self.noise.get(i).unwrap()
    }
}

impl ChunkGenerator for VanillaGenerator {
    fn generate_chunk(&self, pos: Coord2DI) -> Chunk {
        let mut blocks = Vec::new();
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let xi = x as i32;
                    let yi = y as i32;
                    let zi = z as i32;

                    let pos = Coord3DI::new(xi, yi, zi);

                    let noise = self.get_noise(xi, yi, zi);

                    if noise < &0.0 {
                        // Air
                        continue;
                    }

                    // Check if the block above is air
                    if self.get_noise(xi, yi + 1, zi) < &0.0 {
                        // Grass block
                        blocks.push(Block {
                            pos,
                            material: GRASS,
                        });
                    } else {
                        // Dirt block
                        blocks.push(Block {
                            pos,
                            material: DIRT,
                        });
                    }
                }
            }
        }
        Chunk {
            pos,
            blocks,
        }
    }
}

