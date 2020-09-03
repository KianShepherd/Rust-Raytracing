use crate::ray::Ray;
use crate::vec3::Vec3;

#[allow(dead_code)]
pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let _aspect_ratio = 16.0 / 9.0;
        let _viewport_height = 2.0;
        let _viewport_width = _aspect_ratio * _viewport_height;
        let _focal_length = 1.0;

        let _origin = Vec3::new(0.0, 0.0, 0.0);
        let _horizontal = Vec3::new(_viewport_width, 0.0, 0.0);
        let _vertical = Vec3::new(0.0, _viewport_height, 0.0);
        let _lower_left_corner = _origin
            - _horizontal.scale(0.5)
            - _vertical.scale(0.5)
            - Vec3::new(0.0, 0.0, _focal_length);
        Camera {
            aspect_ratio: _aspect_ratio,
            viewport_height: _viewport_height,
            viewport_width: _viewport_width,
            focal_length: _focal_length,
            origin: _origin,
            horizontal: _horizontal,
            vertical: _vertical,
            lower_left_corner: _lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
                - self.origin,
        )
    }
}
