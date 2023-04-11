use std::ops::{Add, Index, Sub};
use num_traits::{Num, ToPrimitive};

pub trait Point: Num + ToPrimitive {}

pub trait Indexable {
    fn index(&self) -> usize;
}

impl Point for i32 {}

impl Point for u32 {}

impl Point for f32 {}

#[derive(Clone, Copy, Debug)]
pub struct Coord2<T: Point> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Coord3<T: Point> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Point> Coord2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Point> Coord3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Point> Indexable for Coord3<T> {
    fn index(&self) -> usize {
        ((self.x.to_i32().unwrap() << 16) + (self.y.to_i32().unwrap() << 8) + self.z.to_i32().unwrap()) as usize
    }
}

pub type Coord2DI = Coord2<i32>;
pub type Coord2DF = Coord2<f32>;
pub type Coord3DI = Coord3<i32>;
pub type Coord3DF = Coord3<f32>;