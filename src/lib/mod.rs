use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use palette::{Lch, LinSrgba, Srgb};

pub use linear::{linear_gradient_continuous, linear_gradient_stepped};
pub use radial::{
    radial_gradient_continuous, radial_gradient_stepped, radial_gradient_with_overlay,
};

mod err;
mod linear;
mod radial;

pub(crate) enum Work {
    LinGradCont,
    LinGradStep,
    RadGradCont,
    RadGradContOverlay,
    RadGradStep,
}

pub struct Config {
    pub angle_offset: f32,
    pub grad_vec: Vec<Lch>,
    pub linear: bool,
    pub radius_inner: f32,
    pub no_file: bool,
    pub overlay: LinSrgba,
    pub overlay_factor: f32,
    pub output_file: Option<PathBuf>,
    pub print_grad: bool,
    pub size: u32,
    pub steps: usize,
    pub swatch_size: (u32, u32),
}

/// Generate a filename with a Unix timestamp.
pub(crate) fn generate_filename() -> Result<String, Box<dyn Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let secs = now.as_secs();
    let millis = format!("{:03}", now.subsec_millis());
    Ok(secs.to_string() + &millis)
}

/// Displays the colors in a stepped gradient.
pub(crate) fn print_colors(colors: &Vec<Srgb>) {
    if let Some((last, elements)) = colors.split_last() {
        for c in elements {
            print!("{:x},", c.into_format::<u8>());
        }
        print!("{:x}\n", last.into_format::<u8>());
    }
}

/// Saves `RgbImage` provided a buffer and file path.
pub(crate) fn save_image(imgbuf: &image::RgbImage, title: &PathBuf) -> Result<(), Box<dyn Error>> {
    match imgbuf.save(&title) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}.", err);
            std::fs::remove_file(&title)?;
        }
    }

    Ok(())
}

/// Saves `RgbaImage` provided a buffer and file path.
pub(crate) fn save_image_alpha(
    imgbuf: &image::RgbaImage,
    title: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    match imgbuf.save(&title) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}.", err);
            std::fs::remove_file(&title)?;
        }
    }

    Ok(())
}
