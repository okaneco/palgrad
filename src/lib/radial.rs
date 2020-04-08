use std::error::Error;
use std::path::PathBuf;

use palette::{Blend, Gradient, LinSrgb, LinSrgba, Pixel, Srgb, Srgba};

use crate::{generate_filename, print_colors, save_image_alpha, Config};

/// Finds the midpoint between a point in the image and the center of the image
/// with width `size_x` and height `size_y`.
fn midpoint_xy_dist(size_x: u32, size_y: u32, x2: u32, y2: u32) -> [f32; 2] {
    let mut result: [f32; 2] = [0.0, 0.0];
    result[0] = (x2 as f32) - ((size_x as f32 / 2.0) - 1.0);
    result[1] = (y2 as f32) - ((size_y as f32 / 2.0) - 1.0);
    result
}

/// Creates an image of a circular, continuous gradient. The steps between each
/// color will be indiscernible given a large enough image size.
pub fn radial_gradient_continuous(config: Config) -> Result<(), Box<dyn Error>> {
    let grad = Gradient::new(config.grad_vec);
    let img_x = config.size;
    let img_y = config.size;
    let mut imgbuf: image::RgbaImage = image::ImageBuffer::new(img_x, img_y);
    let rad_squared = (config.size as f32 * 0.5).powi(2);
    let rad_inner = (config.size as f32 * config.radius_inner).powi(2);
    let tau = core::f32::consts::PI * 2.0;
    let angle_offset = config.angle_offset;

    let mut pix: [u8; 4];
    let mut dist: [f32; 2];
    let mut dist_squared;
    let mut arctan_res;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        dist = midpoint_xy_dist(img_x, img_y, x, y);
        dist_squared = dist[0] * dist[0] + dist[1] * dist[1];
        if dist_squared >= rad_inner && dist_squared <= rad_squared {
            arctan_res = dist[1].atan2(dist[0]);
            if arctan_res.is_sign_negative() {
                arctan_res += tau;
            }
            arctan_res += angle_offset;
            if arctan_res > tau {
                arctan_res = arctan_res.rem_euclid(tau);
            }
            pix = Srgba::from_linear(LinSrgba::from(grad.get(arctan_res / tau)))
                .into_format()
                .into_raw();
        } else {
            pix = [0, 0, 0, 0];
        }
        *pixel = image::Rgba(pix);
    }

    let mut title = PathBuf::from(generate_filename()?);
    title.set_extension("png");

    save_image_alpha(&imgbuf, &title)
}

/// Creates an image of a circular, stepped gradient. The steps between each
/// color are discrete and noticeable compared to a continuous gradient.
pub fn radial_gradient_stepped(config: Config) -> Result<(), Box<dyn Error>> {
    // We need to add +1 because the starting color is appended to the end
    // for radial gradients. Otherwise, we're left with `steps - 1` colors.
    let steps = config.steps + 1;

    let grad1 = Gradient::new(config.grad_vec);
    let grad2 = grad1.take(steps);

    let mut grad_vec = Vec::with_capacity(steps);
    grad2
        .into_iter()
        .for_each(|c| grad_vec.push(Srgb::from_linear(LinSrgb::from(c))));

    if config.print_grad {
        print_colors(&grad_vec);
    }
    if config.no_file {
        return Ok(());
    }

    let img_x = config.size;
    let img_y = config.size;
    let mut imgbuf: image::RgbaImage = image::ImageBuffer::new(img_x, img_y);
    let rad_squared = (config.size as f32 * 0.5).powi(2);
    let rad_inner = (config.size as f32 * config.radius_inner).powi(2);
    let tau = core::f32::consts::PI * 2.0;
    let angle_offset = config.angle_offset;
    let grad_len = (grad_vec.len() - 1) as f32;

    let mut pix: [u8; 4];
    let mut dist: [f32; 2];
    let mut dist_squared;
    let mut arctan_res;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        dist = midpoint_xy_dist(img_x, img_y, x, y);
        dist_squared = dist[0] * dist[0] + dist[1] * dist[1];
        if dist_squared >= rad_inner && dist_squared <= rad_squared {
            arctan_res = dist[1].atan2(dist[0]);
            if arctan_res.is_sign_negative() {
                arctan_res += tau;
            }
            arctan_res += angle_offset;
            if arctan_res > tau {
                arctan_res = arctan_res.rem_euclid(tau);
            }
            pix = Srgba::from(grad_vec[((arctan_res / tau) * grad_len).round() as usize])
                .into_format()
                .into_raw();
        } else {
            pix = [0, 0, 0, 0];
        }
        *pixel = image::Rgba(pix);
    }

    let mut title = PathBuf::from(generate_filename()?);
    title.set_extension("png");

    save_image_alpha(&imgbuf, &title)
}

/// Creates an image of a circular, continuous gradient with a color overlay.
/// The color is overlayed using the blending mode `atop`. A blending factor
/// can be used to adjust the radius of the overlay blending.
pub fn radial_gradient_with_overlay(config: Config) -> Result<(), Box<dyn Error>> {
    let grad = Gradient::new(config.grad_vec);
    let angle_offset = config.angle_offset;
    let factor = config.overlay_factor;
    let img_x = config.size;
    let img_y = config.size;
    let mut imgbuf: image::RgbaImage = image::ImageBuffer::new(img_x, img_y);
    let rad_squared = (config.size as f32 * 0.5).powi(2);
    let two_pi = core::f32::consts::PI * 2.0;

    let mut pix: [u8; 4];
    let mut dist: [f32; 2];
    let mut arctan_res;
    let mut temp;
    let mut dist_squared;
    let mut overlay = config.overlay;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        dist = midpoint_xy_dist(img_x, img_y, x, y);
        dist_squared = dist[0] * dist[0] + dist[1] * dist[1];
        if (dist_squared) <= rad_squared {
            arctan_res = dist[1].atan2(dist[0]);
            if arctan_res.is_sign_negative() {
                arctan_res += two_pi;
            }
            arctan_res += angle_offset;
            if arctan_res > two_pi {
                arctan_res = arctan_res.rem_euclid(two_pi);
            }
            temp = LinSrgba::from(grad.get(arctan_res / two_pi));
            overlay.alpha = (1.0 - (dist_squared / rad_squared)) * factor;
            temp = overlay.atop(temp);
            pix = Srgba::from_linear(temp).into_format().into_raw();
        } else {
            pix = [0, 0, 0, 0];
        }
        *pixel = image::Rgba(pix);
    }

    let mut title = PathBuf::from(generate_filename()?);
    title.set_extension("png");

    save_image_alpha(&imgbuf, &title)
}
