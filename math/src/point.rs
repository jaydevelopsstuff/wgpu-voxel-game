use std::any::TypeId;
use std::fmt::{Debug, Display};
use num_traits::{FromPrimitive, Num, NumCast, ToPrimitive};

pub trait Point<T>: Num + NumCast + ToPrimitive + FromPrimitive + Clone + Debug + Display {}

pub trait Indexable {
    fn index(&self) -> usize;
}

pub trait LossyCast<T> {
   fn cast(&self) -> T;
}

impl Point<i32> for i32 {}

impl Point<u32> for u32 {}

impl Point<f32> for f32 {}

pub fn index<T: Point<T>>(x: &T, y: &T, z: &T) -> usize {
    // flatten x, y, z into a single index for an array with an unknown length
    ((y.to_i32().unwrap() << 8) | (z.to_i32().unwrap() << 4) | x.to_i32().unwrap()) as usize
}

fn try_cast<T: Point<T>, A: Point<A> + 'static>(x: T) -> Option<A> {
    if TypeId::of::<A>() == TypeId::of::<i32>() {
        A::from(x.clone().to_i32().unwrap())
    } else if TypeId::of::<A>() == TypeId::of::<u32>() {
        A::from(x.clone().to_u32().unwrap())
    } else if TypeId::of::<A>() == TypeId::of::<f32>() {
        A::from(x.clone().to_f32().unwrap())
    } else {
        panic!("Cannot cast to unknown type");
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point2<T: Point<T>> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Point3<T: Point<T>> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Point<T>> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Point<T>> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn to_coord2(&self) -> Point2<T> {
        Point2 {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Point<T>> Indexable for Point3<T> {
    fn index(&self) -> usize {
        index(&self.x, &self.y, &self.z)
    }
}

impl<'a, A: Point<A> + 'static, T: Point<T> > LossyCast<Point2<A>> for Point2<T> {
    fn cast(&self) -> Point2<A> {
        Point2 {
            x: try_cast(self.x.clone()).expect("Failed to cast x"),
            y: try_cast(self.y.clone()).expect("Failed to cast y"),
        }
    }
}

impl<A: Point<A> + From<T>, T: Point<T>> LossyCast<Point3<A>> for Point3<T> {
    fn cast(&self) -> Point3<A> {
        Point3 {
            x: try_cast(self.x.clone()).expect("Failed to cast x"),
            y: try_cast(self.x.clone()).expect("Failed to cast y"),
            z: try_cast(self.x.clone()).expect("Failed to cast z")
        }
    }
}

pub type Point2DF = Point2<f32>;
pub type Point2DI = Point2<i32>;
pub type Point3DF = Point3<f32>;
pub type Point3DI = Point3<i32>;

pub type Coord2DI = Point2<i32>;
pub type Coord2DF = Point2<f32>;
pub type Coord3DI = Point3<i32>;
pub type Coord3DF = Point3<f32>;