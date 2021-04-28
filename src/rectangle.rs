use crate::material::Material;
use crate::vec3::Vec3;
use crate::hittable;
use crate::ray::Ray;
use crate::triangle::Triangle;

pub struct Rectangle {
    triangles: [Triangle; 2],
}

impl Rectangle {
    pub fn new(bottom_left: Vec3, bottom_right: Vec3, top_left: Vec3, top_right: Vec3, mat: Material, cull_back_face: bool) -> Rectangle {
        let triangle_1 = Triangle::new(bottom_left, top_left, bottom_right, mat, cull_back_face);
        let triangle_2 = Triangle::new(top_right, bottom_right, top_left, mat, cull_back_face);
        Rectangle {
            triangles: [triangle_1, triangle_2],
        }
    }
}

impl hittable::Hittable for Rectangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        self.triangles[0].hit(ray, t_min, t_max, rec) || self.triangles[1].hit(ray, t_min, t_max, rec)
    }
}