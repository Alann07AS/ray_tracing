use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const VEC_NULL: Vector = Vector {
    x: 0.,
    y: 0.,
    z: 0.,
};

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64,) -> Self {
        Self{x, y, z}
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn lenth(&self) -> f64 {
        const SQR: f64 = 2.;
        (self.x.powf(SQR) + self.y.powf(SQR) + self.z.powf(SQR)).sqrt()
    }
    pub fn unit(&self) -> Self {
        let l = self.lenth();
        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }
}
// impl<'a, 'b> std::ops::Add<&'b Vector> for &'a Vector {
impl<'a> Div<f64> for &'a Vector {
    type Output = Vector;
    fn div(self, v: f64) -> Vector {
        Vector {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        }
    }
}

impl<'a, 'b> Mul<&'b Vector> for &'a Vector {
    type Output = Vector;
    fn mul(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<'a> Mul<f64> for &'a Vector {
    type Output = Vector;
    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<'a> Sub<f64> for &'a Vector {
    type Output = Vector;
    fn sub(self, other: f64) -> Vector {
        Vector {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector> for &'a Vector {
    type Output = Vector;
    fn sub(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a> Add<f64> for &'a Vector {
    type Output = Vector;
    fn add(self, other: f64) -> Vector {
        Vector {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;
    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
