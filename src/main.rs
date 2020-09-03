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
#[allow(dead_code)]
fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

fn ray_color(ray: ray::Ray, world: &hittables::Hittables<sphere::Sphere>) -> vec3::Vec3 {
    let mut hit_rec = hittable::HitRecord::new();

    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_rec) {
        (hit_rec.normal.clone().unwrap() + vec3::Vec3::new(1.0, 1.0, 1.0)).scale(0.5)
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
                pixel_color = pixel_color + ray_color(r, &world);
            }
            output = format!("{}\n{}", output, pixel_color.to_string(samples_per_pixel));
        }
    }
    eprintln!("100.00% Done\n\n");
    println!("{}", output);
}
