mod ray;
mod vec3;
// run with
// cargo make all
fn ray_color(ray: ray::Ray) -> vec3::Vec3 {
    let unit_dir = ray.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    let one = vec3::Vec3::new(1.0, 1.0, 1.0).scale(1.0 - t);
    let two = vec3::Vec3::new(0.5, 0.7, 1.0).scale(t);
    one + two
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Vec3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal.scale(0.5)
        - vertical.scale(0.5)
        - vec3::Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);
    let progress_prints = 25.0;

    for j in 0..image_height {
        // progress check
        if j % ((image_height as f64 / progress_prints) as i32) == 0 {
            eprintln!("{:.2}% done", (j as f64 / image_height as f64) * 100.0);
        }

        for i in 0..image_width {
            let r = {
                let u = (i as f64) / (image_width - 1) as f64;
                let v = (image_height - (j + 1)) as f64 / (image_height - 1) as f64;
                ray::Ray::new(
                    origin.clone(),
                    lower_left_corner + horizontal.scale(u) + vertical.scale(v) - origin,
                )
            };
            let color = ray_color(r);
            output = format!("{}\n{}", output, color.to_string());
        }
    }
    eprintln!("\nDone.\n");
    println!("{}", output);
}
