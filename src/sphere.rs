use crate::hittable;
use crate::material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: material::Material,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(cen: Vec3, rad: f64, mat: material::Material) -> Sphere {
        Sphere {
            center: cen,
            radius: rad,
            material: mat,
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let r = ray.clone();
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp1 = (-half_b - root) / a;
            let temp2 = (-half_b + root) / a;

            if temp1 < t_max && temp1 > t_min {
                rec.t = Some(temp1);
                rec.p = Some(r.at(rec.t.unwrap()));
                let outward_normal = (rec.p.unwrap() - self.center) * (1.0 / self.radius);
                rec.set_face_normal(r, outward_normal);
                rec.material = Some(self.material.clone());
                true
            } else if temp2 < t_max && temp2 > t_min {
                rec.t = Some(temp1);
                rec.p = Some(r.at(rec.t.unwrap()));
                let outward_normal = (rec.p.unwrap() - self.center) * (1.0 / self.radius);
                rec.set_face_normal(r, outward_normal);
                rec.material = Some(self.material.clone());
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
