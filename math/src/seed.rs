use rand::{random, Rng};

pub struct Seed(u32);

impl Seed {
    pub fn new(seed: u32) -> Self {
        Self(seed)
    }

    pub fn random() -> Self {
        // Self((rand::thread_rng().gen::<f32>() * 100.0) as u32)
        Seed(random())
    }

    pub fn get(&self) -> u32 {
        println!("Seed: {}", self.0);
        self.0
    }
}