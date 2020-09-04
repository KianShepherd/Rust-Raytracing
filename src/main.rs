mod camera;
mod hittable;
mod hittables;
use rand::Rng;
mod ray;
mod sphere;
mod vec3;
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
    world: &hittables::Hittables<sphere::Sphere>,
    depth: i32,
) -> vec3::Vec3 {
    let mut hit_rec = hittable::HitRecord::new();

    if depth <= 0 {
        return vec3::Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_rec) {
        // Quick
        // let target = hit_rec.p.clone().unwrap() + hit_rec.normal.clone().unwrap() + random_unit_vec3();
        // Lambertian
        let target =
            hit_rec.p.clone().unwrap() + hit_rec.normal.clone().unwrap() + lamber_unit_vec3();
        // Hemisphere
        // let target = hit_rec.p.clone().unwrap() + hit_rec.normal.clone().unwrap() + random_in_hemisphere(hit_rec.normal.clone().unwrap());
        ray_color(
            ray::Ray::new(
                hit_rec.p.clone().unwrap(),
                target - hit_rec.p.clone().unwrap(),
            ),
            world,
            depth - 1,
        )
        .scale(0.5)
    } else {
        let unit_dir = ray.direction().unit_vector();
        let t = 0.5 * (unit_dir.y() + 1.0);
        let one = vec3::Vec3::new(1.0, 1.0, 1.0).scale(1.0 - t);
        let two = vec3::Vec3::new(0.5, 0.7, 1.0).scale(t);
        one + two
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = hittables::Hittables {
        hittables: vec![
            sphere::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5),
            sphere::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0),
        ],
    };

    // Camera
    let cam = camera::Camera::new();

    // Render
    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);
    let progress_prints = 25.0;

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
                    cam.get_ray(u, v)
                };
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            output = format!("{}\n{}", output, pixel_color.to_string(samples_per_pixel));
        }
    }
    eprintln!("100.00% Done\n\n");
    println!("{}", output);
}
