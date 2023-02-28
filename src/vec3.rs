use std::{ops::{Add, Sub, Mul, Div}, fmt::Display};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {

    pub fn zero() -> Vec3 {
        Vec3 { x: (0.0), y: (0.0), z: (0.0) }
    }

    pub fn new<T1: Into<f64>, T2: Into<f64>, T3: Into<f64>>(x: T1, y: T2, z: T3) -> Vec3 {
        Vec3 {x: x.into(), y: y.into(), z: z.into()}
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn norm(&self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 { 
            x: (self.y * rhs.z - self.z * rhs.y),
            y: (self.z * rhs.x - self.x * rhs.z),
            z: (self.x * rhs.y - self.y * rhs.x) 
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3 ({}, {}, {})", self.x, self.y, self.z)
    }
}

// Operator definitions for vec3

// Addition of vec3 with vec3, and potential ref combinations
impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x, 
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<'a, 'b> Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        *self + *rhs
    }
}

impl<'a> Add<Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        *self + rhs
    }
}

impl<'a> Add<&'a Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        self + *rhs
    }
}


// Addition of vec3 with f64, and potential ref combinations

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs, 
            y: self.y + rhs,
            z: self.z + rhs 
        }
    }
}

impl Add<&f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &f64) -> Self::Output {
        *self + *rhs
    }
}

impl Add<&f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &f64) -> Self::Output {
        self + *rhs
    }
}

impl Add<f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        *self + rhs
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl Add<&Vec3> for &f64 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        rhs + self
    }
}

impl Add<Vec3> for &f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl Add<&Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        rhs + self
    }
}

// Subtraction of vec3 with vec3, and potential ref combinations

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x, 
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<'a, 'b> Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        *self - *rhs
    }
}

impl<'a> Sub<Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - rhs
    }
}

impl<'a> Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        self - *rhs
    }
}

// Subtraction of vec3 with f64, and potential ref combinations

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x - rhs, 
            y: self.y - rhs,
            z: self.z - rhs 
        }
    }
}

impl Sub<&f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &f64) -> Self::Output {
        *self - *rhs
    }
}

impl Sub<f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        *self - rhs
    }
}

impl Sub<&f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &f64) -> Self::Output {
        self - *rhs
    }
}

// Subtraction of f64 with vec3, and potential ref combinations TODO

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self - rhs.x, 
            y: self - rhs.y,
            z: self - rhs.z 
        }
    }
}

// Multiplication of f64 with vec3, and potential ref combinations TODO

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

}

// Multiplication of vec3 with f64, and potential ref combinations TODO

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }

    
}

// Division of vec3 by f64, and potential ref combinations

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0/scalar)
    }
}

impl Div<&f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, scalar: &f64) -> Self::Output {
        *self / *scalar
    }
}

impl Div<&f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: &f64) -> Self::Output {
        self / *scalar
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Self::Output {
        *self / scalar
    }
}


pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn to_rgba(&self, alpha: u8, samples_per_pixel: u64) -> [u8;4] {
        
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        let scale = 1.0 / (samples_per_pixel as f64);
        r *= scale;
        g *= scale;
        b *= scale;

        let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;

        [ir, ig, ib, alpha]
    }
}