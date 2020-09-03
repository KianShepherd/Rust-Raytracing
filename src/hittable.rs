use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Option<Vec3>,
    pub normal: Option<Vec3>,
    pub t: Option<f64>,
    front_face: Option<bool>,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: None,
            normal: None,
            t: None,
            front_face: None,
        }
    }
    pub fn get_p(&self) -> Option<Vec3> {
        self.p
    }
    pub fn get_t(&self) -> Option<f64> {
        self.t
    }
    pub fn get_normal(&self) -> Option<Vec3> {
        self.normal
    }
    pub fn get_front_face(&self) -> Option<bool> {
        self.front_face
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        let r = ray.clone();
        self.front_face = Some(r.direction().dot(outward_normal) < 0.0);
        if self.front_face.unwrap() {
            self.normal = Some(outward_normal);
        } else {
            self.normal = Some(-outward_normal);
        }
    }

    pub fn set_rec(&mut self, r: &HitRecord) {
        self.p = r.p.clone();
        self.t = r.t.clone();
        self.normal = r.normal.clone();
        self.front_face = r.front_face.clone();
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
