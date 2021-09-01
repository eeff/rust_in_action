//! Project mandelbrot from the book, chapter 2 code listing 2.12

use num::complex::Complex;

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);
    render_mandelbrot(mandelbrot);
}

/// Converts between the output space (a grid of rows and columns)
/// and a range that surrounds the Mandelbrot set (a continuous range near (0,0)).
///
/// `x_min`, `x_max`, `y_min` and `y_max` specify the space we're searching for to
/// look for members of the set.
/// `width` and `height` represent the size of the output in pixels.
///
///         x_min              x_max
///   y_min +------------------> x
///         |
///         |
///         |
///         |
///   y_max V
///         y
fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows = Vec::with_capacity(height);

    for img_y in 0..height {
        let mut row = Vec::with_capacity(width);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

/// Called at every pixel.
///
/// If a value has not escaped before reaching the maximum number of iterations
/// `max_iters`, it's considered to be within the Mandelbrot set.
fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

/// Renders pixels to stdout.
fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}
