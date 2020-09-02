mod vec3;
// run with
// cargo make all

fn main() {
    let image_width = 256;
    let image_height = 256;
    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        // progress check
        if j % ((image_height as f64 / 11.0) as i32) == 0 {
            eprintln!("{:.2}% done", (j as f64 / image_height as f64) * 100.0);
        }

        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = (image_height - (j + 1)) as f64 / (image_height - 1) as f64;
            let color = vec3::Vec3::new(r, g, 0.25);
            output = format!("{}\n{}", output, color.to_string());
        }
    }
    eprintln!("\nDone.\n");
    println!("{}", output);
}
