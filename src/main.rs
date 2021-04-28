mod camera;
mod hittable;
mod hittables;
use rand::Rng;
use crate::hittable::Hittable;
use image::RgbImage;
use crate::hittables::Hittables;
use std::time::Instant;
use crate::camera::Camera;
use crate::rectangle::Rectangle;
use crate::vec3::Vec3;

mod material;
mod ray;
mod sphere;
mod vec3;
mod triangle;
mod terrain;
mod noise;
mod colour_map;
mod rectangle;

// run with
// cargo make run
fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}
fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
#[allow(dead_code)]
fn random_vec3(min: f64, max: f64) -> vec3::Vec3 {
    vec3::Vec3::new(
        random_f64(min, max),
        random_f64(min, max),
        random_f64(min, max),
    )
}
// Quick Diffusion
#[allow(dead_code)]
fn random_unit_vec3() -> vec3::Vec3 {
    let mut p: vec3::Vec3;
    loop {
        p = random_vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}
// Lambertian Diffuse
#[allow(dead_code)]
fn lamber_unit_vec3() -> vec3::Vec3 {
    let a = random_f64(0.0, 2.0 * std::f64::consts::PI);
    let z = random_f64(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    vec3::Vec3::new(r * a.cos(), r * a.sin(), z)
}
// Alternative Diffuse
#[allow(dead_code)]
fn random_in_hemisphere(normal: vec3::Vec3) -> vec3::Vec3 {
    let in_unit_sphere = random_unit_vec3();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

fn create_world(aspect_ratio: f64) ->  (Hittables<dyn Hittable>, Camera) {
    let v_fov = 90.0;
    let look_from = vec3::Vec3::new(0.0, 0.0, -2.5);
    let look_at = vec3::Vec3::new(0.0, 0.0, 0.0);
    let v_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let focal_distance = (look_from - look_at).length();
    let aperture = 0.01;

    let camera = camera::Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio, aperture, focal_distance);

    let mut world_objects: Vec<Box<dyn Hittable>> = vec![];
    world_objects.push(Box::new(
        rectangle::Rectangle::new(
            Vec3::new(-2.0, -2.0, 0.0),
            Vec3::new(2.0, -2.0, 0.0),
            Vec3::new(-2.0, 2.0, 0.0),
            Vec3::new(2.0, 2.0, 0.0),
            material::Material::Lambertian(Vec3::new(0.0, 0.6, 0.0)),
            false
        )));
    world_objects.push(Box::new(
        rectangle::Rectangle::new(
            Vec3::new(-2.0, -2.0, 0.0),
            Vec3::new(-2.0, -2.0, -2.0),
            Vec3::new(-2.0, 2.0, 0.0),
            Vec3::new(-2.0, 2.0, -2.0),
            material::Material::Lambertian(Vec3::new(0.6, 0.0, 0.0)),
            false
        )));
    world_objects.push(Box::new(
        rectangle::Rectangle::new(
            Vec3::new(2.0, -2.0, 0.0),
            Vec3::new(2.0, -2.0, -2.0),
            Vec3::new(2.0, 2.0, 0.0),
            Vec3::new(2.0, 2.0, -2.0),
            material::Material::Lambertian(Vec3::new(0.9, 0.9, 0.9)),
            false
        )));
    world_objects.push(Box::new(
        rectangle::Rectangle::new(
            Vec3::new(-2.0, 2.0, 2.0),
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(-2.0, 2.0, -2.0),
            Vec3::new(2.0, 2.0, -2.0),
            material::Material::Lambertian(Vec3::new(0.0, 0.0, 0.9)),
            false
        )));
    world_objects.push(Box::new(
        rectangle::Rectangle::new(
            Vec3::new(-2.0, -2.0, 2.0),
            Vec3::new(2.0, -2.0, 2.0),
            Vec3::new(-2.0, -2.0, -2.0),
            Vec3::new(2.0, -2.0, -2.0),
            material::Material::Lambertian(Vec3::new(0.9, 0.9, 0.0)),
            false
        )));

    let world = Hittables {
        lights: vec![],
        hittables: world_objects,
    };

    (world, camera)
}

fn create_procedural_world(aspect_ratio: f64) ->  (Hittables<dyn Hittable>, Camera) {
    let terrain_size = 32.0;
    let terrain_resolution = 70;
    let height_scale = terrain_size / 6.0;
    let v_fov = 90.0;
    let look_from = vec3::Vec3::new(0.0, 3.5 * height_scale, terrain_size * 0.6);
    let look_at = vec3::Vec3::new(0.0, 0.0, 0.0);
    let v_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let focal_distance = (look_from - look_at).length();
    let aperture = 0.01;
    let octaves = 2;
    let frequency = 0.045;
    let lacunarity = 0.5;

    let camera = camera::Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio, aperture, focal_distance);
    let noise = noise::Noise::new(terrain_resolution + 1, octaves, frequency, lacunarity);
    let colour_map = colour_map::ColourMap::new_default();

    let mut terrain = terrain::Terrain::new(terrain_size, terrain_size, terrain_resolution);
    let mut world: Hittables<dyn Hittable> = terrain.get_triangles(Some(noise), Some(colour_map), height_scale);
    let light = vec3::Vec3::new(-1500.0, 900.0, 1200.0);

    world.push_light(light);

    (world, camera)
}

fn ray_color(
    ray: ray::Ray,
    world: &hittables::Hittables<dyn Hittable>,
    depth: i32,
) -> vec3::Vec3 {
    let mut hit_rec = hittable::HitRecord::new();
    let bias = 0.01;

    if depth <= 0 {
        return vec3::Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_rec) {
        let color = &mut vec3::Vec3::new(0.0, 0.0, 0.0);
        let res = material::scatter(ray, hit_rec, color, hit_rec.material.unwrap());
        match res {
            Some(result) => {
                let mut in_shadow = vec3::Vec3::new(1.0, 1.0, 1.0);
                for i in 0..world.lights.len() {
                    let light_direction = (world.lights[i] - hit_rec.p.unwrap()).unit_vector();
                    let point_of_intersection = hit_rec.p.unwrap() + (light_direction * bias);

                    if world.hit(ray::Ray::new(point_of_intersection, light_direction), 0.001, f64::INFINITY, &mut hittable::HitRecord::new()) {
                        in_shadow = in_shadow * vec3::Vec3::new(0.3, 0.3, 0.3);
                    }
                }

                *color * ray_color(result, world, depth - 1) * in_shadow
            },
            None => vec3::Vec3::new(0.0, 0.0, 0.0),
        }
    } else {
        let unit_dir = ray.direction().unit_vector();
        let t = 0.5 * (unit_dir.y() + 1.0);
        let one = vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
        let two = vec3::Vec3::new(0.5, 0.7, 1.0) * t;
        one + two
    }
}

#[allow(unused_variables)]
fn main() {
    let testing_scene = true;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 300;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let mut image = RgbImage::new(image_width as u32, image_height as u32);
    let samples_per_pixel = 40;
    let max_depth = 50;

    let (world, camera) =  if testing_scene { create_world(aspect_ratio) } else { create_procedural_world(aspect_ratio) };

    let now = Instant::now();
    let progress_prints = image_width as f64 / 16.0;
    for j in 0..image_height {
        // progress check
        if j % ((image_height as f64 / progress_prints) as i32) == 0 {
            eprintln!("{:.2}% Done", (j as f64 / image_height as f64) * 100.0);
        }

        for i in 0..image_width {
            let mut pixel_color = vec3::Vec3::new(0.0, 0.0, 0.0);
            for _k in 0..samples_per_pixel {
                let r = {
                    let u = ((i as f64) + random()) / (image_width - 1) as f64;
                    let v =
                        ((image_height - (j + 1)) as f64 + random()) / (image_height - 1) as f64;
                        camera.get_ray(u, v)
                };
                pixel_color = pixel_color + ray_color(r, &world, max_depth);

            }
            image.put_pixel(i as u32, j as u32, pixel_color.to_rgb(samples_per_pixel));
        }
    }
    let mut seconds = now.elapsed().as_secs();
    let mut minutes = seconds / 60;
    seconds = seconds % 60;
    let hours = minutes / 60;
    minutes = minutes % 60;
    eprintln!("100.00% Done\n\nTime taken: {}h : {}m : {}s\n\n", hours, minutes, seconds);
    image.save("image.jpg").unwrap();
}
