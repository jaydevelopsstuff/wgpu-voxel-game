#[derive(Clone, Copy, Debug)]
pub struct Coord3DI {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord3DI {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

pub struct Coord3DF {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coord3DF {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}