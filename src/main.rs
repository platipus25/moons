use image::{GenericImageView, ImageReader, Pixel};
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use std::error;

static MOONS: [&str; 8] = ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");
    let img = ImageReader::open("moonception.png")?.decode()?;
    let resized = img.resize(30, 30, image::imageops::FilterType::Triangle);

    let gradient = resized.filter3x3(&[0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
    gradient.save("grad.png")?;

    let luma = resized.into_luma8();
    let moon_pixels: Vec<&str> = luma
        .par_enumerate_pixels()
        .map(|(x, y, value)| {
            let gradient = gradient.get_pixel(x, y).to_luma_alpha().0[0];
            let luma = value.to_luma().0[0];
            println!("{}", gradient);

            select_moon(luma, gradient)
        })
        .collect();

    let (width, _height) = luma.dimensions();

    let result = moon_pixels
        .chunks_exact(width as usize)
        .map(|row| row.join(""))
        .join("\n");

    println!("{}", result);

    Ok(())
}

fn select_moon(luma: u8, gradient: u8) -> &'static str {
    // Adjust gamma
    let gamma = 1.2;
    let float_luma = luma as f32 / u8::MAX as f32;
    let adjusted_luma = float_luma.powf(gamma);

    // Split into 5 brightness levels
    let brightness = (adjusted_luma * 5.0).round() as u8;

    enum Brightness {
        Zero,
        One,
        Two,
        Three,
        Bright,
    }

    let brightness: Brightness = match brightness {
        0 => Brightness::Zero,
        1 => Brightness::One,
        2 => Brightness::Two,
        3 => Brightness::Three,
        4 => Brightness::Bright,
        _ => Brightness::Bright,
    };

    // take the (first 2?) MSB to check if it's positive or negative
    // one of these assumptions (-127, 128 or 0, 255) will be wrong and I'm excited to find out which!
    let rightward_brightness = (gradient >> 6) > 0;

    match (brightness, rightward_brightness) {
        (Brightness::Zero, true) => MOONS[0],
        (Brightness::One, true) => MOONS[1],
        (Brightness::Two, true) => MOONS[2],
        (Brightness::Three, true) => MOONS[3],
        (Brightness::Bright, true) => MOONS[4],
        (Brightness::Zero, false) => MOONS[0],
        (Brightness::One, false) => MOONS[7],
        (Brightness::Two, false) => MOONS[6],
        (Brightness::Three, false) => MOONS[5],
        (Brightness::Bright, false) => MOONS[4],
    }
}
