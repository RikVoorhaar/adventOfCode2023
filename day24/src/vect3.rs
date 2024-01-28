use std::fmt::Debug;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vect3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Debug for Vect3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl Vect3 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vect3 {
    type Output = Vect3;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vect3 {
    type Output = Vect3;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vect3 {
    type Output = Vect3;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vect3> for f64 {
    type Output = Vect3;

    fn mul(self, other: Vect3) -> Vect3 {
        other * self
    }
}

impl Sub for Vect3 {
    type Output = Vect3;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl AddAssign for Vect3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Index<usize> for Vect3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for Vect3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}
