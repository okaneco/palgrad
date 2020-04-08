use std::error::Error;
use std::path::PathBuf;
use std::process;

use clap::{crate_version, App, Arg};
use palette::{Hsv, Lch, LinSrgba, Srgb};

mod lib;
use lib::*;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let m = App::new("palgrad")
        .version(crate_version!())
        .about("Create gradients and palettes from the command-line")
        .arg(
            Arg::with_name("colors")
                .min_values(2)
                .max_values(32)
                .empty_values(false)
                .short("c")
                .long("colors")
                .help("Specify the colors in `R,G,B` format delimted by `;`")
                .value_name("COLORS")
                .default_value("228,68,21;236,228,38;46,137,209")
                .require_delimiter(true)
                .value_delimiter(";"),
        )
        .arg(
            Arg::with_name("decimal colors")
                .min_values(2)
                .max_values(32)
                .empty_values(false)
                .short("d")
                .long("dec")
                .help("Specify the colors in `R,G,B` format delimted by `;` in the range of 0.0 to 1.0")
                .value_name("DECIMAL_COLORS")
                .default_value("1.0,0.6,0.0;0.0,0.2,1.0")
                .require_delimiter(true)
                .value_delimiter(";"),
        )
        // .arg(
        //     Arg::with_name("hex colors")
        //         .min_values(2)
        //         .max_values(32)
        //         .empty_values(false)
        //         .short("x")
        //         .long("hex")
        //         .help("Specify the colors in hex format `#RRGGBB` delimited by `;`")
        //         .value_name("HEX_COLORS")
        //         .default_value("#e85348;#468f46;#22106e")
        //         .require_delimiter(true)
        //         .value_delimiter(";"),
        // )
        .arg(
            Arg::with_name("hsv colors")
                .min_values(2)
                .max_values(32)
                .empty_values(false)
                .long("hsv")
                .help("Specify the colors in `H,S,V` format delimited by `;`")
                .value_name("HSV_COLORS")
                .default_value("0,80,60;120,70,60;240,80,60")
                .require_delimiter(true)
                .value_delimiter(";"),
        )
        .arg(
            Arg::with_name("lch colors")
                .min_values(2)
                .max_values(32)
                .empty_values(false)
                .long("lch")
                .help("Specify the colors in `L,C,h` format delimited by `;`")
                .value_name("LCH_COLORS")
                .default_value("100.0,75.0,0.0;20.0,25.0,200.0")
                .require_delimiter(true)
                .value_delimiter(";"),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .help("Diameter of the round palette created in pixels or each linear swatch size")
                .takes_value(true)
                .default_value("512")
        )
        .arg(
            Arg::with_name("radius")
                .short("r")
                .long("radius")
                .alias("ir")
                .help("Inner radius factor between 0.0 and 0.5")
                .takes_value(true)
                .required(false)
                .default_value("0.05"),
        )
        .arg(
            Arg::with_name("steps")
                .short("n")
                .long("steps")
                .alias("st")
                .help("Number of color steps in the gradient")
                .takes_value(true)
                .required(false)
                .default_value("11"),
        )
        .arg(
            Arg::with_name("overlay")
                .short("o")
                .long("overlay")
                .min_values(1)
                .max_values(1)
                .empty_values(false)
                .help("Color of overlay in R,G,B")
                .takes_value(true)
                .default_value("120,120,120")
                .require_delimiter(false),
        )
        .arg(
            Arg::with_name("output")
                .help("Name of the output file")
                .last(true)
                .required(false),
        )
        .arg(
            Arg::with_name("linear")
                .short("l")
                .long("linear")
                .help("Create a linear gradient")
                .required(false)
                .conflicts_with_all(&["overlay", "size"]),
        )
        .arg(
            Arg::with_name("swatch size")
                .long("ss")
                .help("Set the dimensions of a linear swatch sample")
                .min_values(1)
                .max_values(2)
                .required(false)
                .takes_value(true)
                .default_value("40x40")
                .value_delimiter("x"),
        )
        .arg(
            Arg::with_name("print")
            .short("p")
            .long("print")
            .help("Print colors produced by stepped gradients")
        )
        .arg(
            Arg::with_name("no file")
            .long("no-file")
            .help("Don't output file, for use with printing stepped gradient colors")
        )
        .get_matches();

    let mut grad_vec = Vec::with_capacity(32);

    if m.occurrences_of("colors") > 0 {
        if let Some(colors) = m.values_of("colors") {
            for color in colors {
                let c = color.split(",").collect::<Vec<_>>();
                let r: u8 = c[0].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Red in {}, value should be 0-255", color)
                });
                let g: u8 = c[1].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Green in {}, value should be 0-255", color)
                });
                let b: u8 = c[2].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Blue in {}, value should be 0-255", color)
                });
                grad_vec.push(Lch::from(
                    Srgb::new(r, g, b).into_format::<f32>().into_linear(),
                ));
            }
        }
    }

    if m.occurrences_of("decimal colors") > 0 {
        if let Some(colors) = m.values_of("decimal colors") {
            for color in colors {
                let c = color.split(",").collect::<Vec<_>>();
                let r: f32 = c[0].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Red in {}, value should be 0.0-1.0", color)
                });
                let g: f32 = c[1].parse().unwrap_or_else(|_| {
                    panic!(
                        "Could not parse Green in {}, value should be 0.0-1.0",
                        color
                    )
                });
                let b: f32 = c[2].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Blue in {}, value should be 0.0-1.0", color)
                });
                grad_vec.push(Lch::from(Srgb::new(r, g, b).into_linear()));
            }
        }
    }

    // if m.occurrences_of("hex colors") > 0 {
    //     if let Some(colors) = m.values_of("hex colors") {
    //         for color in colors {
    //             let rgb = Rgb::<encoding::Srgb, u8>::from_str(color).unwrap();
    //             grad_vec.push(Lch::from(rgb.into_format::<f32>().into_linear()));
    //         }
    //     }
    // }

    if m.occurrences_of("hsv colors") > 0 {
        if let Some(colors) = m.values_of("hsv colors") {
            for color in colors {
                let c = color.split(",").collect::<Vec<_>>();
                let h: f32 = c[0].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Hue in {}, value should be 0-360", color)
                });
                let s: f32 = c[1].parse().unwrap_or_else(|_| {
                    panic!(
                        "Could not parse Saturation in {}, value should be 0-100",
                        color
                    )
                });
                let v: f32 = c[2].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Value in {}, value should be 0-100", color)
                });
                grad_vec.push(Lch::from(Hsv::new(h, s / 100.0, v / 100.0)));
            }
        }
    }

    if m.occurrences_of("lch colors") > 0 {
        if let Some(colors) = m.values_of("lch colors") {
            for color in colors {
                let c = color.split(",").collect::<Vec<_>>();
                let l: f32 = c[0].parse().unwrap_or_else(|_| {
                    panic!(
                        "Could not parse Lightness in {}, value should be 0-100",
                        color
                    )
                });
                let chroma: f32 = c[1].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Chroma in {}, value should be 0-100", color)
                });
                let hue: f32 = c[2].parse().unwrap_or_else(|_| {
                    panic!("Could not parse Hue in {}, value should be 0-360", color)
                });
                grad_vec.push(Lch::new(l, chroma, hue));
            }
        }
    }

    let linear;
    if m.is_present("linear") {
        linear = true;
    } else {
        linear = false;
        grad_vec.push(grad_vec[0]);
    }

    let output_file;
    if m.is_present("output") {
        output_file = Some(PathBuf::from(m.value_of("output").unwrap()));
    } else {
        output_file = None;
    }

    let mut overlay = LinSrgba::from(
        Srgb::new(120u8, 120, 120)
            .into_format::<f32>()
            .into_linear(),
    );
    if m.is_present("overlay") {
        if let Some(color) = m.value_of("overlay") {
            let c = color.split(",").collect::<Vec<_>>();
            let r: u8 = c[0].parse().unwrap_or_else(|_| {
                panic!("Could not parse Red in {}, value should be 0-255", color)
            });
            let g: u8 = c[1].parse().unwrap_or_else(|_| {
                panic!("Could not parse Green in {}, value should be 0-255", color)
            });
            let b: u8 = c[2].parse().unwrap_or_else(|_| {
                panic!("Could not parse Blue in {}, value should be 0-255", color)
            });
            overlay = LinSrgba::from(Srgb::new(r, g, b).into_format::<f32>().into_linear());
        }
    }

    let mut swatch_size = (40, 40);
    if m.is_present("swatch size") {
        let mut swatch = m.values_of("swatch size").unwrap();
        match swatch.len() {
            1 => {
                let sx = swatch.next().unwrap().parse::<u32>()?;
                swatch_size = (sx, sx);
            }
            2 => {
                swatch_size = (
                    swatch.next().unwrap().parse::<u32>()?,
                    swatch.next().unwrap().parse::<u32>()?,
                );
            }
            _ => {
                panic!("Could not parse swatch size arguments, only 1 or 2 arguments accepted");
            }
        }
    }

    if swatch_size.0 == 0 || swatch_size.1 == 0 {
        panic!(
            "Swatch dimensions cannot be 0 sized: {}x{}",
            swatch_size.0, swatch_size.1
        );
    }

    let mut radius_inner = m.value_of("radius").unwrap().parse::<f32>()?;
    if radius_inner >= 0.5 {
        radius_inner = 0.49;
    } else if radius_inner < 0.0 {
        radius_inner = 0.0;
    }

    let angle_offset = core::f32::consts::FRAC_PI_2;
    let overlay_factor = 0.9;
    let size = m.value_of("size").unwrap().parse::<u32>()?;
    let steps = m.value_of("steps").unwrap().parse::<usize>()?;

    let print_grad;
    if m.is_present("print") {
        print_grad = true;
    } else {
        print_grad = false;
    }
    let no_file;
    if m.is_present("no file") {
        no_file = true;
    } else {
        no_file = false;
    }

    let config = Config {
        angle_offset,
        grad_vec,
        linear,
        radius_inner,
        output_file,
        overlay,
        overlay_factor,
        no_file,
        print_grad,
        size,
        steps,
        swatch_size,
    };

    let program_type;

    if config.linear {
        if m.occurrences_of("steps") > 0 {
            program_type = Work::LinGradStep;
        } else {
            program_type = Work::LinGradCont;
        }
    } else {
        if m.occurrences_of("steps") > 0 {
            program_type = Work::RadGradStep;
        } else if m.occurrences_of("overlay") > 0 {
            program_type = Work::RadGradContOverlay;
        } else {
            program_type = Work::RadGradCont;
        }
    }

    match program_type {
        Work::LinGradCont => linear_gradient_continuous(config)?,
        Work::LinGradStep => linear_gradient_stepped(config)?,
        Work::RadGradCont => radial_gradient_continuous(config)?,
        Work::RadGradContOverlay => radial_gradient_with_overlay(config)?,
        Work::RadGradStep => radial_gradient_stepped(config)?,
    }

    Ok(())
}
