fn mandelbrot_point(a: f64, b: f64, max_iter: u32) -> u32 {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    for i in 0..max_iter {
        let x2 = x * x;
        let y2 = y * y;
        if x2 + y2 > 4.0 {
            return i;
        }

        let new_x = x2 - y2 + a;
        let new_y = 2.0 * x * y + b;
        x = new_x;
        y = new_y;
    }

    max_iter
}

pub fn render_mandelbrot_scalar(
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
        let b = b_min + (y as f64) * b_step;
        for x in 0..width {
            let a = a_min + (x as f64) * a_step;
            let it = mandelbrot_point(a, b, max_iter);
            out[y * width + x] = if it >= max_iter { 0 } else { 255 };
        }
    }
}
