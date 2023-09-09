use crate::physics::*;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32, pub y: f32, pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) 
         + self.y.powi(2) 
         + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn normalized(&self) -> Self {
        let mut v = self.clone();
        v.normalize();
        v
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x 
        + self.y * other.y 
        + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z 
                - self.z * other.y,
            y: -(self.x * other.z 
                 - self.z * other.x),
            z: self.x * other.y 
                - self.y * other.x,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
