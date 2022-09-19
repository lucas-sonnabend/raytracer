fn main() {
    create_image(256, 256);
}

fn create_image(height: i32, width: i32) {
    println!("P3\n{width} {height}\n255");

    for i in 0..height {
        for j in 0..width {
            let red = i as f32 / (width - 1) as f32;
            let green = j as f32 / (height -1) as f32;
            let blue = 0.25;
            let ired = (255.999 * red) as i32;
            let igreen = (255.999 * green) as i32;
            let iblue = (255.999 * blue) as i32;

            println!("{ired} {igreen} {iblue}");
        } 
    }

}
