use crate::rectangle::Rectangle;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::hittable;
use crate::ray::Ray;

#[allow(dead_code)]
pub struct Cube {
    top_right: Vec3,
    bottom_left: Vec3,
    squares: Vec<Rectangle>,
    material: Material,
}

impl Cube {
    pub fn new(bottom_front_left: Vec3, top_back_right: Vec3, mat: Material) -> Self {
        let bottom_front_right = Vec3::new(top_back_right.x(), bottom_front_left.y(), bottom_front_left.z());
        let top_front_left = Vec3::new(bottom_front_left.x(), top_back_right.y(), bottom_front_left.z());
        let top_front_right = Vec3::new(top_back_right.x(), top_back_right.y(), bottom_front_left.z());

        let bottom_back_left = Vec3::new(bottom_front_left.x(), bottom_front_left.y(), top_back_right.z());
        let bottom_back_right = Vec3::new(top_back_right.x(), bottom_front_left.y(), top_back_right.z());
        let top_back_left = Vec3::new(bottom_front_left.x(), top_back_right.y(), top_back_right.z());

        let mut squares_ = vec![];
        //Front
        squares_.push(Rectangle::new( bottom_front_left, bottom_front_right, top_front_left,top_front_right, mat, false));
        //Back
        squares_.push(Rectangle::new( bottom_back_left, bottom_back_right, top_back_left,top_back_right, mat, false));
        //Top
        squares_.push(Rectangle::new(top_front_left, top_front_right, top_back_left, top_back_right, mat, false));
        //Bottom
        squares_.push(Rectangle::new(bottom_front_left, bottom_front_right, bottom_back_left, bottom_back_right, mat, false));
        //Left
        squares_.push(Rectangle::new(bottom_back_left, bottom_front_left, top_back_left, top_front_left, mat, false));
        //Right
        squares_.push(Rectangle::new(top_back_right, bottom_back_right, top_front_right, bottom_front_right, mat, false));

        Cube {
            top_right: top_back_right,
            bottom_left: bottom_front_left,
            squares: squares_,
            material: mat,
        }
    }
}

impl hittable::Hittable for Cube {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let mut hit = false;
        for i in 0..self.squares.len() {
            hit = hit || self.squares[i].hit(ray, t_min, t_max, rec);
        }
        hit
    }
}