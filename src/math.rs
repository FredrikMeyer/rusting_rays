use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() -> Self {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        self.sq_length().sqrt()
    }

    pub fn sq_length(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        x * x + y * y + z * z
    }

    pub fn as_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let diff = Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };

        diff.length()
    }

    pub fn sq_distance(&self, other: &Point) -> f64 {
        let diff = Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };

        diff.sq_length()
    }

    pub fn as_vector(&self) -> Vector3 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        Vector3 { x, y, z }
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;
    fn sub(self, other: Point) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: f64) -> Self::Output {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[test]
fn test_add_vector() {
    let v1 = Vector3 {
        x: 1.,
        y: 2.,
        z: 3.,
    };

    let v2 = Vector3 {
        x: 3.,
        y: 2.,
        z: 1.,
    };

    let v = v1 + v2;

    assert!((v.x - 4.).abs() < 0.0001);
    assert!((v.y - 4.).abs() < 0.0001);
    assert!((v.z - 4.).abs() < 0.0001);
}

#[test]
fn test_multiply_vector_with_scalar() {
    let v1 = Vector3 {
        x: 1.,
        y: 2.,
        z: 3.,
    };

    let s = 2.;

    let v = v1 * s;

    assert!((v.x - 2.).abs() < 0.0001);
    assert!((v.y - 4.).abs() < 0.0001);
    assert!((v.z - 6.).abs() < 0.0001);
}
