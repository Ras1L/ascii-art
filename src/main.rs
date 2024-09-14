use std::io::stdin;

use image::{GenericImageView, Rgba, DynamicImage};

fn get_str_ascii(intent: u8) -> &'static str
{
    let index: u8 = intent/32;
    let ascii: [&str; 8] = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image(path: &str, scale: u32)
{
    let img: DynamicImage = image::open(path).expect("Error to open image");
    let (width, height) = img.dimensions();

    for y in 0..height
    {
        for x in 0..width
        {
            if y % (scale * 2) == 0 && x % scale == 0
            {
                let pixel:  Rgba<u8> = img.get_pixel(x, y);
                let mut intent: u8 = pixel[0]/3 + pixel[1]/3 + pixel[2]/3;
                if pixel[3] == 0
                {
                    intent = 0;
                }
                print!("{}", get_str_ascii(intent));
            }
        }
        if y % (scale * 2) == 0
        {
            println!("");
        }
    }
}

fn main()
{
    let mut path: String = String::new();
    println!("Enter image dir: ");
    stdin().read_line(&mut path).expect("Error to read path from console");

    get_image(&path.trim(), 4);
}