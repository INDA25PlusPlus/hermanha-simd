use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use hermanha_simd::scalar::render_mandelbrot_scalar;
use hermanha_simd::simd::render_mandelbrot_simd;

fn criterion_benchmark(c: &mut Criterion) {
    let width = 1920;
    let height = 1080;
    let mut buffer = vec![0u8; width * height];

    c.bench_function("mandelbrot_scalar_render", |b| {
        b.iter(|| {
            render_mandelbrot_scalar(
                black_box(&mut buffer),
                width,
                height,
                2000,
                -2.5,
                1.0,
                -1.2,
                1.2,
            );
        })
    });

    c.bench_function("mandelbrot_simd_render", |b| {
        b.iter(|| {
            render_mandelbrot_simd(
                black_box(&mut buffer),
                width,
                height,
                2000,
                -2.5,
                1.0,
                -1.2,
                1.2,
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
