use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

impl Vec3 {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Vec3 {
        Vec3 {
            x: _x,
            y: _y,
            z: _z,
        }
    }
    #[allow(dead_code)]
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    #[allow(dead_code)]
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    #[allow(dead_code)]
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3 {
        self.clone() * (1.0 / self.length())
    }

    #[allow(dead_code)]
    pub fn to_string(&self, samples_per_pixel: usize) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (256.0 * clamp((self.x * scale).sqrt(), 0.0, 0.999)) as u8;
        let g = (256.0 * clamp((self.y * scale).sqrt(), 0.0, 0.999)) as u8;
        let b = (256.0 * clamp((self.z * scale).sqrt(), 0.0, 0.999)) as u8;
        format!("{} {} {}", r, g, b)
    }

    pub fn to_rgb(&self, samples_per_pixel: usize) -> Vec<u8> {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (256.0 * clamp((self.x * scale).sqrt(), 0.0, 0.999)) as u8;
        let g = (256.0 * clamp((self.y * scale).sqrt(), 0.0, 0.999)) as u8;
        let b = (256.0 * clamp((self.z * scale).sqrt(), 0.0, 0.999)) as u8;

        vec![r, g, b]
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x(),
            y: self * rhs.y(),
            z: self * rhs.z(),
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self / rhs.x(),
            y: self / rhs.y(),
            z: self / rhs.z(),
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
