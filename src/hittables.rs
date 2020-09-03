use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Hittables<T: Hittable> {
    hittables: Vec<T>,
}

#[allow(dead_code)]
impl<T> Hittables<T>
where
    T: Hittable,
{
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut temp_rec = HitRecord::new();
        let mut closest = t_max;

        for hittable in &self.hittables {
            if hittable.hit(ray, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.get_t().unwrap();
                rec.set_rec(&temp_rec);
            }
        }
        hit_anything
    }
}
