use palette::{Lch, LinSrgba};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub use linear::{linear_gradient_continuous, linear_gradient_stepped};
pub use radial::{
    radial_gradient_continuous, radial_gradient_stepped, radial_gradient_with_overlay,
};

mod linear;
mod radial;

pub(crate) enum Work {
    LinGradCont,
    LinGradContOverlay,
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
    pub overlay: LinSrgba,
    pub overlay_factor: f32,
    pub output_file: Option<PathBuf>,
    pub size: u32,
    pub steps: u32,
    pub swatch_size: (u32, u32),
}

pub(crate) fn generate_filename() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Could not get SystemTime");
    let secs = now.as_secs().to_string();
    let mut millis = now.subsec_millis().to_string();
    match millis.chars().count() {
        2 => {
            millis = String::from("0") + &millis;
        }
        1 => {
            millis = String::from("00") + &millis;
        }
        0 => {
            millis = String::from("000");
        }
        _ => {}
    }
    let title = secs + &millis + ".png";
    title
}
