use crate::material::Material;
use crate::vec3::Vec3;
use crate::hittable;
use crate::ray::Ray;

pub struct Triangle {
    points: Vec<Vec3>,
    normal: Vec3,
    material: Material,
}

impl Triangle {
    pub fn new(point1: Vec3, point2: Vec3, point3: Vec3, mat: Material) -> Triangle {
        let points_ = {
            let mut points = vec![];
            points.push(point1);
            points.push(point2);
            points.push(point3);
            points
        };
        let normal_ = {
            let a = point2 - point1;
            let b = point3 - point1;
            a.cross(b).unit_vector()
        };

        Triangle {
            points: points_,
            normal: normal_,
            material: mat,
        }
    }
}

impl hittable::Hittable for Triangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let vertex0 = self.points.get(0).unwrap().clone();
        let vertex1 = self.points.get(1).unwrap().clone();
        let vertex2 = self.points.get(2).unwrap().clone();

        let edge1 = vertex1 - vertex0;
        let edge2 = vertex2 - vertex0;

        let h = ray.direction().cross(edge2);
        let a = edge1.dot(h);
        if a < t_min { return false; }

        let f = 1.0 / a;
        let s = ray.origin() - vertex0;
        let u = f * (s.dot(h));
        if u < 0.0 || u > 1.0 { return false; }

        let q = s.cross(edge1);
        let v = f * (ray.direction().dot(q));
        if v < 0.0 || u + v > 1.0 { return false; }

        let t = f * (edge2.dot(q));
        if t > t_max || t < t_min { return false; }
        let intersection_point = ray.origin() + ray.direction() * t;

        rec.t = Some(t);
        rec.p = Some(intersection_point.clone());
        rec.set_face_normal(ray, self.normal.clone());
        rec.material = Some(self.material.clone());
        
        true
    }
}