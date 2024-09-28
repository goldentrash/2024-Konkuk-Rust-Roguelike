use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};

use num::pow::Pow;
use num::Complex;

use std::env;
use std::fs::File;
use std::str::FromStr;

/// Try to determine if `c` is in the Mandelbrot set,
/// using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered
/// on the origin. If `c` seems to be a member (more precisely,
/// if we reached the iteration limit without being able to prove that
/// `c` is not a member), return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut re = 0.;
    let mut im = 0.;

    for i in 0..limit {
        let tre = re;
        let tim = im;

        re = tre.pow(2) - tim.pow(2) + c.re;
        im = 2. * tre * tim + c.im;

        if re.pow(2) + im.pow(2) > 4. {
            return Some(i + 1);
        }
    }

    None
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`.
/// If it doesn't parse correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    let coords: Vec<&str> = s.split(separator).collect();

    if coords.len() != 2 {
        return None;
    }

    if let (Ok(x), Ok(y)) = (T::from_str(coords[0]), T::from_str(coords[1])) {
        return Some((x, y));
    }

    None
}

/// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    let (a, b) = parse_pair::<f64>(s, ',')?;
    Some(Complex { re: a, im: b })
}

/// Given the row and column of a pixel in the output image,
/// return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex plane
/// designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let dx = (upper_left.re - lower_right.re) / bounds.1 as f64;
    let dy = (upper_left.im - lower_right.im) / bounds.0 as f64;

    Complex {
        re: upper_left.re - pixel.1 as f64 * dx,
        im: upper_left.im - pixel.0 as f64 * dy,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    for h in 0..bounds.0 {
        for w in 0..bounds.1 {
            let c = pixel_to_point(bounds, (h, w), upper_left, lower_right);
            pixels[h * bounds.1 + w] = escape_time(c, 126).unwrap_or(127) as u8;
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) {
    let (wh, ww) = bounds;
    let f = File::create(filename).unwrap();
    let pe = PngEncoder::new(f);
    let _ = pe.write_image(pixels, ww as u32, wh as u32, ExtendedColorType::L8);
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER_LEFT LOWER_RIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.00,0.20",
            args[0]
        );

        std::process::exit(-1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds);
}
