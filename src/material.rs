use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{hittable::HitRecord, random_unit_vec3};

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
}

pub fn scatter(ray: Ray, rec: HitRecord, color: &mut Vec3, material: Material) -> Option<Ray> {
    match material {
        Material::Lambertian(col) => lambertian_scatter(ray, rec, color, col),
        Material::Metal(col, fuzz) => metal_scatter(ray, rec, color, col, fuzz),
    }
}
fn lambertian_scatter(
    _ray: Ray,
    rec: HitRecord,
    color: &mut Vec3,
    material_color: Vec3,
) -> Option<Ray> {
    let scatter_direction = rec.normal.unwrap() + random_unit_vec3();
    color.clone_from(&material_color);
    Some(Ray::new(rec.p.unwrap(), scatter_direction))
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}
fn metal_scatter(
    ray: Ray,
    rec: HitRecord,
    color: &mut Vec3,
    material_color: Vec3,
    fuzz: f64,
) -> Option<Ray> {
    let reflected = reflect(ray.direction().unit_vector(), rec.normal.unwrap());
    let scattered = Ray::new(rec.p.unwrap(), reflected + random_unit_vec3() * fuzz);
    color.clone_from(&material_color);
    if scattered.direction().dot(rec.normal.unwrap()) > 0.0 {
        Some(scattered)
    } else {
        None
    }
}
