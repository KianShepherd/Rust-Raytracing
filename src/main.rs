mod camera;
mod hittable;
mod hittables;
use rand::Rng;
use crate::hittable::Hittable;
use image::RgbImage;
use crate::hittables::Hittables;
use crate::sphere::Sphere;

mod material;
mod ray;
mod sphere;
mod vec3;
mod triangle;
mod terrain;
mod noise;
mod colour_map;

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
    let testing_scene = false;

    let terrain_size = 1024.0;
    let height_scale = 175.0;
    let aspect_ratio = 16.0 / 9.0;
    let v_fov = 90.0;
    let look_from = vec3::Vec3::new(0.0, 3.5 * height_scale, terrain_size * 0.6);
    let look_at = vec3::Vec3::new(0.0, 0.0, 0.0);
    let v_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let focal_distance = (look_from - look_at).length();
    let aperture = 0.01;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let progress_prints = image_width as f64 / 16.0;
    let samples_per_pixel = 25;
    let max_depth = 35;
    let mut image = RgbImage::new(image_width as u32, image_height as u32);
    let terrain_resolution = 30;
    let octaves = 2;
    let frequency = 0.15;
    let lacunarity = 0.5;
    let camera1 = camera::Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio, aperture, focal_distance);
    let noise = noise::Noise::new(terrain_resolution + 1, octaves, frequency, lacunarity);
    let colour_map = colour_map::ColourMap::new_default();

    //let mut terrain = terrain::Terrain::new(terrain_size, terrain_size, 1);
    let mut terrain = terrain::Terrain::new(terrain_size, terrain_size, terrain_resolution);
    //let mut world = terrain.get_triangles(None, None, 0.0);
    let mut world1: Hittables<dyn Hittable> = terrain.get_triangles(Some(noise), Some(colour_map), height_scale);
    world1.push(Box::new(sphere::Sphere::new(
        vec3::Vec3::new(0.0, 150.0, 50.0),
        50.0,
        material::Material::Lambertian(vec3::Vec3::new(0.2, 0.6, 0.9)),
    )));
    world1.push(Box::new(sphere::Sphere::new(
        vec3::Vec3::new(-150.0, 100.0, 0.0),
        50.0,
        material::Material::Metal(vec3::Vec3::new(0.8, 0.6, 0.2), 1.0),
    )));
    world1.push(Box::new(sphere::Sphere::new(
        vec3::Vec3::new(150.0, 250.0, 100.0),
        50.0,
        material::Material::Lambertian(vec3::Vec3::new(0.8, 0.1, 0.6)),
    )));

    world1.push_light(vec3::Vec3::new(-1500.0, 900.0, 1200.0));

    let camera2 = camera::Camera::new(vec3::Vec3::new(0.0, 0.0, 0.0), vec3::Vec3::new(0.0, 0.0, -1.0), vec3::Vec3::new(0.0, 1.0, 0.0), 90.0, aspect_ratio, 0.01, 1.0);
    let world2: Hittables<dyn Hittable> = {
        let world_objects: Vec<Box<dyn Hittable>> = {
            let mut objects: Vec<Box<dyn Hittable>> = vec![];
            objects.push(Box::new(
                sphere::Sphere::new(
                    vec3::Vec3::new(0.0, -100.5, -1.0),
                    100.0,
                    material::Material::Lambertian(vec3::Vec3::new(0.8, 0.8, 0.0)))));
            objects.push(Box::new(
                sphere::Sphere::new(
                    vec3::Vec3::new(0.0, 0.0, -1.0),
                    0.5,
                    material::Material::Lambertian(vec3::Vec3::new(0.1, 0.2, 0.5)))));
            objects.push(Box::new(
                sphere::Sphere::new(
                    vec3::Vec3::new(1.0, 0.0, -1.0),
                    0.5,
                    material::Material::Metal(vec3::Vec3::new(0.8, 0.6, 0.2), 0.1))));
            objects.push(Box::new(
                sphere::Sphere::new(
                    vec3::Vec3::new(-1.0, 0.0, -1.0),
                    0.5,
                    material::Material::Dielectric(1.5))));
            objects
        };
        Hittables {
            lights: vec![],
            hittables: world_objects,
        }
    };

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
                    if testing_scene {
                        camera2.get_ray(u, v)
                    } else {
                        camera1.get_ray(u, v)
                    }
                };
                if testing_scene {
                    pixel_color = pixel_color + ray_color(r, &world2, max_depth);
                } else {
                    pixel_color = pixel_color + ray_color(r, &world1, max_depth);
                }
            }
            image.put_pixel(i as u32, j as u32, pixel_color.to_rgb(samples_per_pixel));
        }
    }
    eprintln!("100.00% Done\n\n");
    image.save("image.jpg").unwrap();
}
