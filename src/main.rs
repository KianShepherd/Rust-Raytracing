// run with
// cargo make all

fn main() {
    let image_width = 256;
    let image_height = 256;
    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);
    let b = (0.25 * 255.99) as i32;
    for j in 0..image_height {
        if j % ((image_height as f64 / 11.0) as i32) == 0 {
            eprintln!("{:.2}% done", (j as f64 / image_height as f64) * 100.0);
        }
        for i in 0..image_width {
            let r = ((i as f64 / (image_width - 1) as f64) * 255.999) as i32;
            let g =
                ((((image_height - j) - 1) as f64 / (image_height - 1) as f64) * 255.999) as i32;
            output = format!("{}\n{} {} {}", output, r, g, b);
        }
    }
    eprintln!("\nDone.\n");
    println!("{}", output);
}
