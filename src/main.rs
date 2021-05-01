use rand::Rng;
use crate::hittable::Hittable;
use image::{RgbImage, Rgb, ImageBuffer};
use crate::hittables::Hittables;
use std::time::Instant;
use crate::camera::Camera;
use crate::vec3::Vec3;
use std::sync::{Mutex, Arc};
use num_cpus;
use std::thread;
use crate::sphere::Sphere;

mod camera;
mod hittable;
mod hittables;
mod material;
mod ray;
mod sphere;
mod vec3;
mod triangle;
mod terrain;
mod noise;
mod colour_map;
mod rectangle;
mod axis_aligned_cube;

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

fn create_world(aspect_ratio: f64) ->  (Hittables, Camera) {
    let v_fov = 90.0;
    let look_from = vec3::Vec3::new(0.0, 0.0, -3.5);
    let look_at = vec3::Vec3::new(0.0, 0.0, 0.0);
    let v_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let focal_distance = (look_from - look_at).length();
    let aperture = 0.01;

    let camera = camera::Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio, aperture, focal_distance);
    let mut light_objects = vec![];
    light_objects.push(Box::new(Vec3::new(-1.0, -1.5, -3.5)));

    let mut world_objects: Vec<Box<dyn Hittable + Send + Sync + 'static>> = vec![];
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

    /*
    world_objects.push(Box::new(Cube::new(Vec3::new(-1.5,-2.0, -1.0),
                                          Vec3::new(-0.5, 1.0, -0.5),
                                          material::Material::Lambertian(Vec3::new(0.6, 0.6, 0.6))
        )));
    world_objects.push(Box::new(Cube::new(Vec3::new(1.8,-2.0, -1.5),
                                          Vec3::new(0.2, 0.0, -1.0),
                                          material::Material::Lambertian(Vec3::new(0.6, 0.6, 0.6))
    )));
    */
    world_objects.push(Box::new(Sphere::new(
        Vec3::new(-0.3, -1.0, -1.0),
        0.5,
        material::Material::Lambertian(Vec3::new(0.7, 0.1, 0.8)))));

    world_objects.push(Box::new(Sphere::new(
        Vec3::new(0.6, 0.0, -1.5),
        0.5,
        material::Material::Metal(Vec3::new(0.7, 0.6, 0.2), 1.0))));
    world_objects.push(Box::new(Sphere::new(
        Vec3::new(-0.9, 1.0, -1.2),
        0.5,
        material::Material::Mirror)));


    let world = Hittables {
        lights: light_objects,
        hittables: world_objects,
    };

    (world, camera)
}

fn create_procedural_world(aspect_ratio: f64) ->  (Hittables, Camera) {
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
    let mut world: Hittables = terrain.get_triangles(Some(noise), Some(colour_map), height_scale);
    let light = vec3::Vec3::new(-1500.0, 900.0, 1200.0);

    world.push_light(light);

    (world, camera)
}

fn ray_color(
    ray: ray::Ray,
    world: &hittables::Hittables,
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
                    let light_direction = (*world.lights[i] - hit_rec.p.unwrap()).unit_vector();
                    let point_of_intersection = hit_rec.p.unwrap() + (light_direction * bias);
                    let max_dist = (point_of_intersection - *world.lights[i]).length();
                    if world.hit(ray::Ray::new(point_of_intersection, light_direction), 0.01, max_dist / 2.0, &mut hittable::HitRecord::new()) {
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

#[derive(Debug, Copy, Clone)]
struct Work {
    x: usize,
    y: usize,
    colour: Option<Rgb<u8>>,
}

fn create_work_list(image_width: i32, image_height: i32, num_cpu: usize) -> Vec<Vec<Work>>{
    let mut work_list = vec![];
    let rows = ((image_height / num_cpu as i32) + 1) as usize;
    for i in 0..num_cpu {
        let mut work_for_cpu = vec![];
        for y in (i * rows)..((i + 1) * rows) {
            for x in 0..image_width {
                if y < image_height as usize && x < image_width {
                    work_for_cpu.push(Work {
                        x: x as usize,
                        y: y as usize,
                        colour: None,
                    });
                }
            }
        }
        work_list.push(work_for_cpu);
    }
    work_list
}

fn sample_pixel(samples_per_pixel: usize, x: f64, y: i32, image_width: i32, image_height: i32, max_depth: i32, camera: &Camera, world: &Hittables) -> Vec3 {
    let mut pixel_color = vec3::Vec3::new(0.0, 0.0, 0.0);

    for _k in 0..samples_per_pixel {
        let r = {
            let u = (x + random()) / (image_width - 1) as f64;
            let v =
                ((image_height - (y + 1)) as f64 + random()) / (image_height - 1) as f64;
            camera.get_ray(u, v)
        };
        pixel_color = pixel_color + ray_color(r, world, max_depth);
    }

    pixel_color
}

#[allow(unused_variables)]
fn main() {
    let testing_scene = true;
    let using_multithreading = true;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let (world, camera) =  if testing_scene { create_world(aspect_ratio) } else { create_procedural_world(aspect_ratio) };

    let now = Instant::now();
    let image = if using_multithreading {
        let image_ = Arc::new(Mutex::new(ImageBuffer::new(image_width as u32, image_height as u32)));
        let world_ = Arc::new(world);
        let camera_ = Arc::new(camera);

        let cpu_count = num_cpus::get();
        let mut task_list = vec![];
        let work_list = Arc::new(create_work_list(image_width, image_height, cpu_count));

        for cpu in 0..cpu_count {
            let scoped_image = image_.clone();
            let scoped_world = world_.clone();
            let scoped_camera = camera_.clone();
            let scoped_work_list = work_list.clone();

            task_list.push(thread::spawn(move || {
                let work_list_for_cpu = scoped_work_list.get(cpu).unwrap();
                let mut inner_work_vec = vec![];

                for work in work_list_for_cpu {
                    let pixel_color = sample_pixel(samples_per_pixel, work.x as f64, work.y as i32, image_width, image_height, max_depth, &scoped_camera, &scoped_world);

                    inner_work_vec.push(Work {
                        x: work.x,
                        y: work.y,
                        colour: Some(pixel_color.to_rgb(samples_per_pixel)),
                    });
                }

                let mut image_data = scoped_image.lock().unwrap();
                for final_work in inner_work_vec {
                    image_data.put_pixel(final_work.x as u32, final_work.y as u32, final_work.colour.unwrap());
                    //println!("{}:{}:{:#?}\n", final_work.x, final_work.y, final_work.colour.unwrap())
                }

                println!("Cpu {} done out of {}.", cpu + 1, cpu_count);
            }));
        }

        for task in task_list {
            let _ = task.join();
        }

        let final_val = match image_.lock() {
            Ok(x) => x.clone(),
            Err(_) => {ImageBuffer::new(image_width as u32, image_height as u32)},
        };
        final_val
    } else { // Single Thread
        let mut image_ = RgbImage::new(image_width as u32, image_height as u32);
        let progress_prints = image_width as f64 / 16.0;
        for j in 0..image_height {
            // progress check
            if j % ((image_height as f64 / progress_prints) as i32) == 0 {
                eprintln!("{:.2}% Done", (j as f64 / image_height as f64) * 100.0);
            }

            for i in 0..image_width {
                let pixel_color = sample_pixel(samples_per_pixel, i as f64, j as i32, image_width, image_height, max_depth, &camera, &world);
                image_.put_pixel(i as u32, j as u32, pixel_color.to_rgb(samples_per_pixel));
            }
        }
        image_
    };

    let mut seconds = now.elapsed().as_secs();
    let mut minutes = seconds / 60;
    seconds = seconds % 60;
    let hours = minutes / 60;
    minutes = minutes % 60;
    eprintln!("100.00% Done\n\nTime taken: {}h : {}m : {}s\n\n", hours, minutes, seconds);
    image.save("image.jpg").unwrap();
}
