use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{hittable::HitRecord, random_unit_vec3, random_f64};
use std::cmp::min;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

pub fn scatter(ray: Ray, rec: HitRecord, color: &mut Vec3, material: Material) -> Option<Ray> {
    match material {
        Material::Lambertian(col) => lambertian_scatter(ray, rec, color, col),
        Material::Metal(col, fuzz) => metal_scatter(ray, rec, color, col, fuzz),
        Material::Dielectric(refractive_index) => dielectric_scatter(ray, rec, color, refractive_index),
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Option<Vec3> {
    let uv_ = uv.unit_vector();
    let dt = uv_.dot(n);
    let discriminant = 1.0 - etai_over_etat * etai_over_etat * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(((uv_ - n * dt) * etai_over_etat) - (n * discriminant.sqrt()))
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = {
        let a = (1.0 - ref_idx) / (1.0 + ref_idx);
        a * a
    };
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
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

fn dielectric_scatter(
    ray: Ray,
    rec: HitRecord,
    color: &mut Vec3,
    refractive_index: f64,
) -> Option<Ray> {
    color.clone_from(&Vec3::new(1.0, 1.0, 1.0));
    let reflected = reflect(ray.direction().unit_vector(), rec.normal.unwrap());
    let outward_normal: Vec3;
    let ni_over_nt: f64;
    let mut cosine: f64;

    if ray.direction().unit_vector().dot(rec.normal.unwrap()) > 0.0 {
        outward_normal = -rec.normal.unwrap();
        ni_over_nt = refractive_index;
        cosine = (ray.direction().dot(rec.normal.unwrap()) * refractive_index) / ray.direction().length();
        //cosine = (1.0 - refractive_index * refractive_index * (1.0 - cosine * cosine)).sqrt();
    } else {
        outward_normal = rec.normal.unwrap();
        ni_over_nt = 1.0 / refractive_index;
        cosine = -ray.direction().dot(rec.normal.unwrap()) / ray.direction().length();
    }

    match refract(ray.direction(), outward_normal, ni_over_nt) {
        Some(ray) => {
            if random_f64(0.0, 1.0) > schlick(cosine, refractive_index) {
                return Some(Ray::new(rec.p.unwrap().clone(), ray.clone()))
            }
        },
        None => { },
    }

    Some(Ray::new(rec.p.unwrap().clone(), reflected))
}
