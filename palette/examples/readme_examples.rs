use image::{GenericImage, GenericImageView, RgbImage, SubImage};

#[allow(unused_variables)]
fn converting() {
    use palette::{FromColor, Hsl, IntoColor, Lch, Srgb};

    let my_rgb = Srgb::new(1.0, 0.76, 0.27);

    let mut my_lch = Lch::from_color(my_rgb);
    my_lch.hue += 180.0;

    let mut my_hsl: Hsl = my_lch.into_color();
    my_hsl.saturation *= 0.3;

    let my_new_rgb = Srgb::from_color(my_hsl);

    // Write example image
    display_colors(
        "example-data/output/readme_converting.png",
        &[DisplayType::Discrete(&[
            my_rgb.into_format(),
            Srgb::from_linear(my_lch.into_color()).into_format(),
            Srgb::from_color(my_hsl).into_format(),
            // my_new_rgb is the same as my_hsl
        ])],
    );
}

fn pixels_and_buffers() {
    use palette::{Pixel, Srgb};

    // The input to this function could be data from an image file or
    // maybe a texture in a game.
    fn swap_red_and_blue(my_rgb_image: &mut [u8]) {
        // Convert `my_rgb_image` into `&mut [Srgb<u8>]` without copying.
        let my_rgb_image = Srgb::from_raw_slice_mut(my_rgb_image);

        for color in my_rgb_image {
            std::mem::swap(&mut color.red, &mut color.blue);
        }
    }

    // Write example image
    let mut image = image::open("example-data/input/fruits.png")
        .expect("could not open 'example-data/input/fruits.png'")
        .to_rgb8();
    swap_red_and_blue(&mut image);
    let filename = "example-data/output/readme_pixels_and_buffers.png";
    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("could not write '{}': {}", filename, e),
    }
}

fn color_operations_1() {
    use palette::{Hsl, Hsv, Hue, Mix, Shade};

    fn transform_color<C>(color: &C, amount: f32) -> C
    where
        C: Hue + Shade<Scalar = f32> + Mix<Scalar = f32> + Clone,
        f32: Into<C::Hue>,
    {
        let new_color = color.shift_hue(100.0).darken(0.5);

        // Interpolate between the old and new color.
        color.mix(&new_color, amount)
    }

    // Write example image
    let hsl_color = Hsl::new(90.0, 0.70, 0.54);
    let hsl_color_at = |amount| {
        use palette::FromColor;

        let color = transform_color(&hsl_color, amount);
        palette::Srgb::from_color(color).into_format()
    };

    let hsv_color = Hsv::new(90.0, 0.74, 0.86);
    let hsv_color_at = |amount| {
        use palette::FromColor;

        let color = transform_color(&hsv_color, amount);
        palette::Srgb::from_color(color).into_format()
    };

    display_colors(
        "example-data/output/readme_color_operations_1.png",
        &[
            DisplayType::Continuous(&hsl_color_at),
            DisplayType::Continuous(&hsv_color_at),
        ],
    );
}

fn color_operations_2() {
    use palette::{blend::Blend, Srgba};

    let color_a = Srgba::new(0.8, 0.2, 0.2, 1.0).into_linear();
    let color_b = Srgba::new(0.1, 0.1, 0.5, 0.5).into_linear();

    let result = color_b.over(color_a); // Regular alpha-over blending.

    // Write example image
    display_colors(
        "example-data/output/readme_color_operations_2.png",
        &[DisplayType::Discrete(&[
            color_a.color.into_encoding().into_format(),
            color_b.color.into_encoding().into_format(),
            result.color.into_encoding().into_format(),
        ])],
    );
}

#[cfg(feature = "std")]
fn gradients_1() {
    use palette::{Gradient, LinSrgb};

    let gradient = Gradient::new(vec![
        LinSrgb::new(0.8, 0.2, 0.2),
        LinSrgb::new(0.1, 0.1, 0.5),
        LinSrgb::new(0.1, 0.3, 0.8),
    ]);

    let taken_colors: Vec<_> = gradient.take(10).collect();

    // Write example image
    let taken_srgb8_colors: Vec<_> = taken_colors
        .into_iter()
        .map(|color| color.into_encoding().into_format())
        .collect();
    display_colors(
        "example-data/output/readme_gradients_1.png",
        &[
            DisplayType::Continuous(&|i| gradient.get(i).into_encoding().into_format()),
            DisplayType::Discrete(&taken_srgb8_colors),
        ],
    );
}

#[cfg(feature = "std")]
fn gradients_2() {
    use palette::{Gradient, LinSrgb};

    let gradient = Gradient::from([
        (0.0, LinSrgb::new(0.8, 0.2, 0.2)), // A pair of position and color.
        (0.2, LinSrgb::new(0.1, 0.1, 0.5)),
        (1.0, LinSrgb::new(0.1, 0.3, 0.8)),
    ]);

    let taken_colors: Vec<_> = gradient.take(10).collect();

    // Write example image
    let taken_srgb8_colors: Vec<_> = taken_colors
        .into_iter()
        .map(|color| color.into_encoding().into_format())
        .collect();
    display_colors(
        "example-data/output/readme_gradients_2.png",
        &[
            DisplayType::Continuous(&|i| gradient.get(i).into_encoding().into_format()),
            DisplayType::Discrete(&taken_srgb8_colors),
        ],
    );
}

enum DisplayType<'a> {
    Discrete(&'a [palette::Srgb<u8>]),
    Continuous(&'a dyn Fn(f32) -> palette::Srgb<u8>),
}

fn display_colors(filename: &str, displays: &[DisplayType]) {
    const WIDTH: u32 = 500;
    const ROW_HEIGHT: u32 = 50;

    let row_height = if displays.len() > 1 {
        ROW_HEIGHT
    } else {
        ROW_HEIGHT * 2
    };

    let mut image = RgbImage::new(WIDTH, displays.len() as u32 * row_height);

    for (i, display) in displays.into_iter().enumerate() {
        let image = image.sub_image(0, i as u32 * row_height, WIDTH, row_height);
        match *display {
            DisplayType::Discrete(colors) => {
                display_discrete(image, colors);
            }
            DisplayType::Continuous(color_at) => {
                display_continuous(image, color_at);
            }
        }
    }

    let _ = std::fs::create_dir("example-data/output");
    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("could not write '{}': {}", filename, e),
    }
}

fn display_discrete(mut image: SubImage<&mut RgbImage>, colors: &[palette::Srgb<u8>]) {
    use palette::Pixel;

    let (width, height) = image.dimensions();
    let swatch_size = width as f32 / colors.len() as f32;
    for (i, &color) in colors.iter().enumerate() {
        let swatch_begin = (i as f32 * swatch_size) as u32;
        let swatch_end = ((i + 1) as f32 * swatch_size) as u32;
        let mut sub_image = image.sub_image(swatch_begin, 0, swatch_end - swatch_begin, height);
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                sub_image.put_pixel(x, y, image::Rgb(*color.as_raw()));
            }
        }
    }
}

fn display_continuous(
    mut image: SubImage<&mut RgbImage>,
    color_at: &dyn Fn(f32) -> palette::Srgb<u8>,
) {
    use palette::Pixel;

    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            image.put_pixel(
                x,
                y,
                image::Rgb(color_at(x as f32 / width as f32).into_raw()),
            );
        }
    }
}

fn main() {
    converting();
    pixels_and_buffers();
    color_operations_1();
    color_operations_2();
    #[cfg(feature = "std")]
    gradients_1();
    #[cfg(feature = "std")]
    gradients_2();
}
