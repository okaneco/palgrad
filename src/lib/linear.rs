use palette::{Gradient, LinSrgb, Pixel, Srgb};

use crate::{generate_filename, Config};

pub fn linear_gradient_continuous(config: Config) -> std::io::Result<()> {
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

    let title = generate_filename();
    imgbuf.save(title).expect("Could not save file");

    Ok(())
}

pub fn linear_gradient_stepped(config: Config) -> std::io::Result<()> {
    let num_steps;
    let vec_len = config.grad_vec.len();
    if vec_len == 2 {
        if config.steps > 1 {
            num_steps = vec_len + config.steps as usize;
        } else if config.steps == 1 {
            num_steps = (vec_len * config.steps as usize) + 1;
        } else {
            num_steps = vec_len;
        }
    } else {
        num_steps = (vec_len - 1) * (config.steps as usize) + 1;
    }

    let grad1 = Gradient::new(config.grad_vec);
    let grad2 = grad1.take(num_steps);

    let mut grad_vec = Vec::with_capacity(num_steps);
    for color in grad2 {
        let pix: [u8; 3] = Srgb::from_linear(LinSrgb::from(color))
            .into_format()
            .into_raw();
        grad_vec.push(pix);
    }

    let img_x = config.swatch_size.0;
    let img_y = config.swatch_size.1;
    let mut imgbuf: image::RgbImage =
        image::ImageBuffer::new(img_x * num_steps as u32, img_y as u32);

    for s in 0..num_steps {
        let pix: [u8; 3] = grad_vec[s];
        for y in 0..img_y {
            for x in (s as u32 * img_x)..((s as u32 + 1) * img_x) {
                let pixel = imgbuf.get_pixel_mut(x, y);
                *pixel = image::Rgb(pix);
            }
        }
    }

    let title = generate_filename();
    imgbuf.save(title).expect("Could not save file");

    Ok(())
}
