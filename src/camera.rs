use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::random_f64;

fn random_in_unit_disk() -> Vec3{
    loop {
        let p = Vec3::new(random_f64(-1.0, 1.0), random_f64(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f64, aspect_ratio: f64, aperature: f64, focus_dist: f64) -> Camera {
        let theta = (v_fov * 3.14159) / 180.0;
        let h = (theta / 2.0).tan();

        let _viewport_height = 2.0 * h;
        let _viewport_width = aspect_ratio * _viewport_height;
        let _focal_length = 1.0;


        let w = (look_from - look_at).unit_vector();
        let u = (v_up.cross(w)).unit_vector();
        let v = w.cross(u);

        let _origin = look_from;
        let _horizontal = u * focus_dist * _viewport_width;
        let _vertical = v * focus_dist * _viewport_height;
        let _lower_left_corner =
            _origin - _horizontal * 0.5 - _vertical * 0.5 - w * focus_dist;

        let lens_radius = aperature / 2.0;

        Camera {
            origin: _origin,
            horizontal: _horizontal,
            vertical: _vertical,
            lower_left_corner: _lower_left_corner,
            w,
            u,
            v,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = (self.u * rd.x()) + (self.v * rd.y());
        Ray::new(
            self.origin.clone() + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
