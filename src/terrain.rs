use crate::vec3::Vec3;
use std::ops::Add;
use crate::hittable::Hittable;
use crate::hittables::Hittables;
use crate::triangle::Triangle;
use crate::material::Material::Lambertian;
use crate::noise::Noise;
use crate::colour_map::ColourMap;

pub(crate) struct Terrain {
    ground_points: Vec<Vec3>,
    vertex_resolution: usize,
}

impl Terrain {
    pub fn new(width: f64, depth: f64, res: usize) -> Self {
        let verts = {
            let r1 = res + 1;
            let mut verts_ = vec![];
            let mut loc = Vec3::new(-width / 2.0, 0.0, -depth / 2.0 );
            for _i in 0..r1 {
                for _j in 0..r1 {
                    verts_.push(loc);
                    loc = loc.add(Vec3::new(width / res as f64, 0.0, 0.0));
                }
                loc = Vec3::new(-width / 2.0, 0.0, loc.z() + (depth / (res as f64)));
            }
            verts_
        };

        Terrain {
            ground_points: verts,
            vertex_resolution: res,
        }
    }

    pub fn get_triangles(&mut self, noise: Option<Noise>, colour_map: Option<ColourMap>, height_scale: f64) -> Hittables<dyn Hittable> {
        match noise {
            Some(noise_) => {
                for i in 0..self.ground_points.len() {
                    self.ground_points[i] = Vec3::new(self.ground_points[i].x(), noise_.noise_map[i] * height_scale, self.ground_points[i].z());
                }
            },
            None => {},
        }

        let hittables_: Vec<Box<dyn Hittable>> = {
            let r1 = &self.vertex_resolution + 1;
            let mut hittables: Vec<Box<dyn Hittable>> = vec![];
            for i in 0..self.vertex_resolution {
                for j in 0..self.vertex_resolution {
                    let i0j0 = self.ground_points[(((i + 0) * r1) + (j + 0)) as usize];
                    let i0j1 = self.ground_points[(((i + 0) * r1) + (j + 1)) as usize];
                    let i1j0 = self.ground_points[(((i + 1) * r1) + (j + 0)) as usize];
                    let i1j1 = self.ground_points[(((i + 1) * r1) + (j + 1)) as usize];
                    let color1: Vec3;
                    let color2: Vec3;

                    match &colour_map {
                        Some(colour_map_) => {
                            let height1 = (i0j1.y() / height_scale + i0j0.y() / height_scale + i1j0.y() / height_scale) / 3.0;
                            let height2 = (i1j0.y() / height_scale + i1j1.y() / height_scale + i0j1.y() / height_scale) / 3.0;
                            color1 = colour_map_.to_colour(height1);
                            color2 = colour_map_.to_colour(height2);
                            //eprintln!("height1:{},    colour1:{}", height1, color1.to_string(1));
                            //eprintln!("height2:{},    colour2:{}", height2, color2.to_string(1));
                        },
                        None => {
                            color1 = Vec3::new(0.2, 0.8, 0.4);
                            color2 = Vec3::new(0.2, 0.8, 0.4);
                        },
                    }
                    hittables.push(Box::new(Triangle::new(
                        i0j1, i0j0, i1j0, Lambertian(color1), false
                    )));
                    hittables.push(Box::new(Triangle::new(
                        i1j0, i1j1, i0j1, Lambertian(color2), false
                    )));
                }
            }
            hittables
        };

        Hittables {
            hittables: hittables_,
            lights: vec![],
        }
    }
}