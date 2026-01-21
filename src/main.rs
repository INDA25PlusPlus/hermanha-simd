#![feature(portable_simd)]

use hermanha_simd::simd::render_mandelbrot_simd;


fn main() {
    let w: usize = 1920;
    let h: usize = 1080;
    let max_iter: u32 = 2000;

    let mut buffer = vec![0u8; w * h];
    render_mandelbrot_simd(&mut buffer, w, h, max_iter, -2.5, 2.0, -1.2, 1.2);

    image::save_buffer(
        "mandelbrot.png",
        &buffer,
        w as u32,
        h as u32,
        image::ColorType::L8,
    )
    .unwrap();
}
