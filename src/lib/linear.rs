use std::error::Error;
use std::path::PathBuf;

use palette::{Gradient, LinSrgb, Pixel, Srgb};

use crate::{generate_filename, print_colors, save_image, Config};

/// Creates an image of a linear, continuous gradient. The steps between each
/// color should be indiscernible given a large enough image size.
pub fn linear_gradient_continuous(config: Config) -> Result<(), Box<dyn Error>> {
    let grad = Gradient::new(config.grad_vec);
    let img_x = config.swatch_size.0;
    let img_y = config.swatch_size.1;
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(img_x, img_y);

    for x in 0..img_x {
        let pix: [u8; 3] = Srgb::from_linear(LinSrgb::from(grad.get(x as f32 / img_x as f32)))
            .into_format()
            .into_raw();
        for y in 0..img_y {
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb(pix);
        }
    }

    let mut title = PathBuf::from(generate_filename()?);
    title.set_extension("png");

    save_image(&imgbuf, &title)
}

/// Creates an image of a linear, stepped gradient. The steps between each
/// color are discrete and noticeable compared to a continuous gradient.
pub fn linear_gradient_stepped(config: Config) -> Result<(), Box<dyn Error>> {
    let grad1 = Gradient::new(config.grad_vec);
    let grad2 = grad1.take(config.steps);

    let mut grad_vec = Vec::with_capacity(config.steps);
    grad2
        .into_iter()
        .for_each(|c| grad_vec.push(Srgb::from_linear(LinSrgb::from(c))));

    if config.print_grad {
        print_colors(&grad_vec);
    }
    if config.no_file {
        return Ok(());
    }

    let img_x = config.swatch_size.0;
    let img_y = config.swatch_size.1;
    let mut imgbuf: image::RgbImage =
        image::ImageBuffer::new(img_x * config.steps as u32, img_y as u32);

    for s in 0..config.steps {
        let pix: [u8; 3] = grad_vec[s].into_format().into_raw();
        for y in 0..img_y {
            for x in (s as u32 * img_x)..((s as u32 + 1) * img_x) {
                let pixel = imgbuf.get_pixel_mut(x, y);
                *pixel = image::Rgb(pix);
            }
        }
    }

    let mut title = PathBuf::from(generate_filename()?);
    title.set_extension("png");

    save_image(&imgbuf, &title)
}
