use crate::hittable;
use crate::ray::Ray;

pub struct Hittables<T: hittable::Hittable> {
    hittables: Vec<T>,
}

#[allow(dead_code)]
impl<T> Hittables<T>
where
    T: hittable::Hittable,
{
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let mut hit_anything = false;
        let mut temp_rec = hittable::HitRecord::new();
        let mut closest = t_max;

        for hittabl in &self.hittables {
            let hit = hittabl.hit(ray, t_min, closest, &mut temp_rec);
            if hit {
                hit_anything = true;
                closest = temp_rec.get_t().unwrap();
                rec.set_rec(&temp_rec);
            }
        }
        hit_anything
    }
}
