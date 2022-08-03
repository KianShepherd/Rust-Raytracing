use crate::camera::Camera;
use crate::configuration::RaytracerScene;
use crate::configuration::RaytracerSettings;
use crate::configuration::RonObject;
use crate::hittable::Hittable;
use crate::hittables::Hittables;
use crate::sphere::Sphere;
use crate::triangle::Triangle;
use crate::vec3::Vec3;
use material::Material;
use num_cpus;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{env, thread};

mod camera;
mod configuration;
mod hittable;
mod hittables;
mod material;
mod ray;
mod sphere;
mod triangle;
mod vec3;

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
/*
fn create_world(settings: &RaytracerSettings) -> (Hittables, Camera) {
    let camera = camera::Camera::new(
        settings.look_from,
        settings.look_at,
        settings.v_up,
        settings.v_fov,
        settings.aspect_ratio,
        settings.aperture,
        settings.focal_distance,
    );

    let mut light_objects = vec![];
    light_objects.push(Box::new(Vec3::new(-1.0, 1.5, -3.5)));

    let mut world_objects: Vec<Box<dyn Hittable + Send + Sync + 'static>> = vec![];
    world_objects.push(Box::new(rectangle::Rectangle::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(2.0, -2.0, 0.0),
        Vec3::new(-2.0, 2.0, 0.0),
        Vec3::new(2.0, 2.0, 0.0),
        material::Material::Lambertian(Vec3::new(0.0, 0.6, 0.0)),
        false,
    )));
    world_objects.push(Box::new(rectangle::Rectangle::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(-2.0, -2.0, -2.0),
        Vec3::new(-2.0, 2.0, 0.0),
        Vec3::new(-2.0, 2.0, -2.0),
        material::Material::Lambertian(Vec3::new(0.6, 0.0, 0.0)),
        false,
    )));
    world_objects.push(Box::new(rectangle::Rectangle::new(
        Vec3::new(2.0, -2.0, 0.0),
        Vec3::new(2.0, -2.0, -2.0),
        Vec3::new(2.0, 2.0, 0.0),
        Vec3::new(2.0, 2.0, -2.0),
        material::Material::Lambertian(Vec3::new(0.9, 0.9, 0.9)),
        false,
    )));
    world_objects.push(Box::new(rectangle::Rectangle::new(
        Vec3::new(-2.0, 2.0, 2.0),
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(-2.0, 2.0, -2.0),
        Vec3::new(2.0, 2.0, -2.0),
        material::Material::Lambertian(Vec3::new(0.0, 0.0, 0.9)),
        false,
    )));
    world_objects.push(Box::new(rectangle::Rectangle::new(
        Vec3::new(-2.0, -2.0, 2.0),
        Vec3::new(2.0, -2.0, 2.0),
        Vec3::new(-2.0, -2.0, -2.0),
        Vec3::new(2.0, -2.0, -2.0),
        material::Material::Lambertian(Vec3::new(0.9, 0.9, 0.0)),
        false,
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

    world_objects.push(Box::new(cube::Cube::new(
        Vec3::new(-1.3, -1.5, -1.5),
        Vec3::new(-0.3, -0.5, -0.5),
        material::Material::Lambertian(Vec3::new(0.7, 0.1, 0.8)))));
    */
    world_objects.push(Box::new(Sphere::new(
        Vec3::new(0.6, 0.0, -1.5),
        0.5,
        material::Material::Metal(Vec3::new(0.7, 0.6, 0.2), 0.3),
    )));
    world_objects.push(Box::new(Sphere::new(
        Vec3::new(-0.9, 1.0, -1.2),
        0.5,
        material::Material::Mirror,
    )));

    let world = Hittables {
        lights: light_objects,
        hittables: world_objects,
    };

    (world, camera)
}
*/
fn ray_color(ray: ray::Ray, world: &hittables::Hittables, depth: i32) -> vec3::Vec3 {
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
                    if world.hit(
                        ray::Ray::new(point_of_intersection, light_direction),
                        0.01,
                        max_dist / 2.0,
                        &mut hittable::HitRecord::new(),
                    ) {
                        in_shadow = in_shadow * vec3::Vec3::new(0.3, 0.3, 0.3);
                    }
                }

                *color * ray_color(result, world, depth - 1) * in_shadow
            }
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

#[derive(Debug, Clone)]
struct Work {
    x: usize,
    y: usize,
    colour: Option<Vec<u8>>,
}

fn create_work_list(image_width: i32, image_height: i32, num_cpu: usize) -> Vec<Vec<Work>> {
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

fn sample_pixel(
    samples_per_pixel: usize,
    x: f64,
    y: i32,
    image_width: i32,
    image_height: i32,
    max_depth: i32,
    camera: &Camera,
    world: &Hittables,
) -> Vec3 {
    let mut pixel_color = vec3::Vec3::new(0.0, 0.0, 0.0);

    for _k in 0..samples_per_pixel {
        let r = {
            let u = (x + random()) / (image_width - 1) as f64;
            let v = ((image_height - (y + 1)) as f64 + random()) / (image_height - 1) as f64;
            camera.get_ray(u, v)
        };
        pixel_color = pixel_color + ray_color(r, world, max_depth);
    }

    pixel_color
}

fn conv_py_vec(vector: Vec<f64>) -> Vec3 {
    vec3::Vec3::new(vector[0], vector[1], vector[2])
}

fn parse_ron_material(mat: Vec<String>) -> Material {
    let material_type = &mat[0];
    match &material_type[..] {
        "Lambertian" => material::Material::Lambertian(vec3::Vec3::new(
            mat[1].parse::<f64>().unwrap(),
            mat[2].parse::<f64>().unwrap(),
            mat[3].parse::<f64>().unwrap(),
        )),
        "Metal" => material::Material::Metal(
            vec3::Vec3::new(
                mat[1].parse::<f64>().unwrap(),
                mat[2].parse::<f64>().unwrap(),
                mat[3].parse::<f64>().unwrap(),
            ),
            mat[4].parse::<f64>().unwrap(),
        ),
        "Mirror" => material::Material::Mirror,
        "Dielectric" => material::Material::Dielectric(mat[4].parse::<f64>().unwrap()),
        &_ => {
            panic!("Unknown material found")
        }
    }
}

fn parse_ron_object(obj: RonObject) -> Box<dyn Hittable + Send + Sync + 'static> {
    if obj.objtype == "Sphere" {
        return Box::new(Sphere::new(
            conv_py_vec(obj.vectors[0].clone()),
            obj.scalars[0],
            parse_ron_material(obj.material),
        ));
    } else if obj.objtype == "Triangle" {
        let cull_back = if obj.scalars[0] == 0.0 { false } else { true };
        return Box::new(Triangle::new(
            conv_py_vec(obj.vectors[0].clone()),
            conv_py_vec(obj.vectors[1].clone()),
            conv_py_vec(obj.vectors[2].clone()),
            parse_ron_material(obj.material),
            cull_back,
        ));
    }
    panic!("unknown ron object type.");
}

pub fn create_image(ron_string: String) -> Vec<u8> {
    let settings = configuration::RaytracerScene::from_ron(ron_string);

    let camera = camera::Camera::new(
        conv_py_vec(settings.camera_pos.clone()),
        conv_py_vec(settings.camera_dir.clone()),
        conv_py_vec(settings.camera_up.clone()),
        settings.v_fov,
        settings.aspect_ratio,
        settings.aperture,
        settings.focal_distance,
    );

    let mut light_objects = vec![];
    for light in settings.lights.clone() {
        light_objects.push(Box::new(conv_py_vec(light.clone())));
    }

    let mut world_objects: Vec<Box<dyn Hittable + Send + Sync + 'static>> = vec![];
    for obj in settings.objects.clone() {
        world_objects.push(parse_ron_object(obj.clone()));
    }

    let world = Hittables {
        lights: light_objects,
        hittables: world_objects,
    };

    let now = Instant::now();
    let image = if settings.multithreading {
        let image_ = Arc::new(Mutex::new({
            let mut x =
                Vec::with_capacity(settings.image_width as usize * settings.image_height as usize);
            x.resize(
                settings.image_width as usize * settings.image_height as usize,
                vec![0 as u8, 0 as u8, 0 as u8],
            );
            x
        }));
        let world_ = Arc::new(world);
        let camera_ = Arc::new(camera);
        let settings_ = Arc::new(settings);

        let cpu_count = num_cpus::get();
        let mut task_list = vec![];
        let work_list = Arc::new(create_work_list(
            settings_.image_width,
            settings_.image_height,
            cpu_count,
        ));

        for cpu in 0..cpu_count {
            let scoped_image = image_.clone();
            let scoped_world = world_.clone();
            let scoped_camera = camera_.clone();
            let scoped_work_list = work_list.clone();
            let scoped_settings = settings_.clone();

            task_list.push(thread::spawn(move || {
                let work_list_for_cpu = scoped_work_list.get(cpu).unwrap();
                let mut inner_work_vec = vec![];

                for work in work_list_for_cpu {
                    let pixel_color = sample_pixel(
                        scoped_settings.samples_per_pixel,
                        work.x as f64,
                        work.y as i32,
                        scoped_settings.image_width,
                        scoped_settings.image_height,
                        scoped_settings.max_depth,
                        &scoped_camera,
                        &scoped_world,
                    );

                    inner_work_vec.push(Work {
                        x: work.x,
                        y: work.y,
                        colour: Some(pixel_color.to_rgb(scoped_settings.samples_per_pixel)),
                    });
                }

                let mut image_data = scoped_image.lock().unwrap();
                for final_work in inner_work_vec {
                    let colour = final_work.colour.unwrap().clone();
                    image_data[(final_work.x as u32
                        + (final_work.y as u32 * scoped_settings.image_width as u32))
                        as usize] = colour;
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
            Err(_) => {
                Vec::with_capacity(settings_.image_width as usize * settings_.image_height as usize)
            }
        };
        final_val
    } else {
        // Single Thread
        let mut image_ =
            Vec::with_capacity(settings.image_width as usize * settings.image_height as usize);
        let progress_prints = settings.image_width as f64 / 16.0;
        for j in 0..settings.image_height {
            // progress check
            if j % ((settings.image_height as f64 / progress_prints) as i32) == 0 {
                eprintln!(
                    "{:.2}% Done",
                    (j as f64 / settings.image_height as f64) * 100.0
                );
            }

            for i in 0..settings.image_width {
                let pixel_color = sample_pixel(
                    settings.samples_per_pixel,
                    i as f64,
                    j as i32,
                    settings.image_width,
                    settings.image_height,
                    settings.max_depth,
                    &camera,
                    &world,
                );
                image_[i as usize * (j as usize * settings.image_width as usize)] =
                    pixel_color.to_rgb(settings.samples_per_pixel);
            }
        }
        image_
    };

    let mut seconds = now.elapsed().as_secs();
    let mut minutes = seconds / 60;
    seconds = seconds % 60;
    let hours = minutes / 60;
    minutes = minutes % 60;
    eprintln!(
        "100.00% Done\n\nTime taken: {}h : {}m : {}s\n\n",
        hours, minutes, seconds
    );

    image.into_iter().flatten().collect()
}
