pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Vec3 {
        Vec3 {
            x: _x,
            y: _y,
            z: _z,
        }
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

    pub fn get(&self, idx: usize) -> f64 {
        assert!(idx <= 3);
        match idx {
            1 => self.x,
            2 => self.y,
            3 => self.z,
            _ => panic!("invalid idx: {} for vec3", idx),
        }
    }
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    pub fn scale(&mut self, scale_factor: f64) {
        self.x = self.x * scale_factor;
        self.y = self.y * scale_factor;
        self.z = self.z * scale_factor;
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            (self.x * 255.999) as u32,
            (self.y * 255.999) as u32,
            (self.z * 255.999) as u32
        )
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
