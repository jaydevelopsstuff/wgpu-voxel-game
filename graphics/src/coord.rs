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