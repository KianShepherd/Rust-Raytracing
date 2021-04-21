use crate::ray::Ray;
use crate::vec3::Vec3;

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f64, aspect_ratio: f64) -> Camera {
        let theta = (v_fov * 3.14159) / 180.0;
        let h = (theta / 2.0).tan();

        let _viewport_height = 2.0 * h;
        let _viewport_width = aspect_ratio * _viewport_height;
        let _focal_length = 1.0;


        let w = (look_from - look_at).unit_vector();
        let u = (v_up.cross(w)).unit_vector();
        let v = w.cross(u);

        let _origin = look_from;
        let _horizontal = u * _viewport_width;
        let _vertical = v * _viewport_height;
        let _lower_left_corner =
            _origin - _horizontal * 0.5 - _vertical * 0.5 - w;

        Camera {
            origin: _origin,
            horizontal: _horizontal,
            vertical: _vertical,
            lower_left_corner: _lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
