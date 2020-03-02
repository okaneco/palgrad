# palgrad
[![Build Status](https://travis-ci.com/okaneco/palgrad.svg?branch=master)](https://travis-ci.com/okaneco/palgrad)

![Radial gradients and linear gradien](gfx/hero.png)

`palgrad` is a command line utility that creates color gradients and palettes from user input colors.

Gradients are created in `Lch` color space (also known as `HCL`), the cylindrical representaiton of the `Lab` or `L*a*b*` color space. Using this color space, gradients are generally more vibrant and visually appealing than other spaces like `HSV` or `RGB`.

## Features

- Radial and linear continuous gradients
- Radial continuous gradients with overlay
- Radial and linear stepped gradients
- Declare colors in `HSV`, `RGB`, and `Lch`

Some ideas for using the output:
- gamut masking
- pixelization to make more color swatches
- bring into digital painting programs to color pick from as limited palettes and apply more filters
- website/application color themes

## Documentation

*To be updated*

## Examples

*To be updated*

## License

This crate is licensed under either
- the [MIT License](LICENSE-MIT), or
- the [Apache License (Version 2.0)](LICENSE-APACHE)

at your option.
