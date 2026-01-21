
use std::simd::{cmp::SimdPartialOrd, f64x4};

fn mandelbrot_point(a: f64x4, b: f64x4, max_iter: u32) -> [u32; 4] {
    let mut x: f64x4 = f64x4::splat(0.0);
    let mut y: f64x4 = f64x4::splat(0.0);
    let escape_limit: f64x4 = f64x4::splat(2.0);
    let mut iters = [0u32; 4];

    for _ in 0..max_iter {
        let x2 = x * x;
        let y2 = y * y;
        let escape_check = x2 + y2;
        let still_active = escape_check.simd_lt(escape_limit*escape_limit);

        if !still_active.any(){
            break;
        }

        let new_x = x2 - y2 + a;
        let new_y = x * y * escape_limit + b;
        x = new_x;
        y = new_y;

        for point in 0..4 {
            if still_active.test(point) {
                iters[point] += 1
            }
        }
    }

    iters
}

pub fn render_mandelbrot_simd(
    out: &mut [u8],
    width: usize,
    height: usize,
    max_iter: u32,
    a_min: f64,
    a_max: f64,
    b_min: f64,
    b_max: f64,
) {
    let a_step = (a_max - a_min) / (width as f64);
    let b_step = (b_max - b_min) / (height as f64);

    for y in 0..height {
        let b = f64x4::splat(b_min + (y as f64) * b_step);

        for x in (0..width).step_by(4) {
            let a = f64x4::from_array([
                a_min + (x as f64 + 0.0) * a_step,
                a_min + (x as f64 + 1.0) * a_step,
                a_min + (x as f64 + 2.0) * a_step,
                a_min + (x as f64 + 3.0) * a_step,
            ]);

            let iters = mandelbrot_point(a, b, max_iter);

            for point in 0..4 {
                let idx = y * width + x + point;
                out[idx] = if iters[point] >= max_iter { 0 } else { 255 };
            }
        }
    }
}
