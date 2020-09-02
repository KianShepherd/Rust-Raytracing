use crate::vec3::Vec3;

#[allow(dead_code)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: orig,
            direction: dir,
        }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    pub fn at(&self, t: f64) -> Vec3 {
        let dir = self.direction.clone().scale(t);
        self.origin + dir
    }
}
