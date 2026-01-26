# SIMD Optimization – Mandelbrot

## Overview

In this project we will try to REEEALLLYY improve the time it takes to render a mandelbrott graph on my little laptop. To do this we will first implement a simple mandelbrot application using scalar calculations. Then see that its extremly slow when increasing the resolution of the image or the iterations doing the mandelbrot mathing stuff. You then realize you want to make it faster. And come up with the ingenious idea that if you could calculate 4 pixels at once/ 4 numbers at once, that would make the process go... 4 times faster!!!! How do we do this then? you could use 4 laptops to calculate different areas of mandelbrot, which i think would be pretty cool. But as i'm not steve jobs, maybe SIMD is better...

So SIMD it is.



### Mandelbrot Iteration

According to [Wikipedia](https://sv.wikipedia.org/wiki/Mandelbrotm%C3%A4ngden) we can write the mandelbrot problem as this:

$$
\begin{aligned}
x_0 &= 0 \\
y_0 &= 0 \\
\\
x_{n+1} &= x_n^2 - y_n^2 + a \\
y_{n+1} &= 2x_n y_n + b
\end{aligned}
$$


So behind this there are some creepy complex math. But mandelbrot lives on a 2d plane. a and b is then the coordinates for that plane. we can then for each pixel coordinate map (x,y) to (a,b) with some manipulation magic, and create an image of this complex world. So the pixel coordinates x value will be scaled, and mapped to the value a. and the y coordinate will be mapped to the value b. Then we iterate for a set amount of iterations to see if we ever reaches this:

$$
\begin{aligned}
\sqrt{x_n^2 + y_n^2} > 2
\end{aligned}
$$

If we do, we exclude that point or coordinate form the mandelbrot set, and make that pixel white. If we reach the maximum iterations without reaching that, we include it in the mandelbrot set and color it black!




## SIMD Strategy

In the SIMD implementation, four pixels are processed simultaneously using f64x4 vectors. We take 4 pixels from the same row each iteration. instead of calculating on one pixel at the time. The simd implementation works just like the scaler version, and outputs the same thing. 



## Benchmarking

Benchmarking was performed using Criterion.

- Image size: 1920 × 1080
- Maximum iterations per pixel: 2000

### Results

| Implementation | Mean Time |
|---------------|-----------|
| Scalar        | ~1.85 s   |
| SIMD          | ~0.45 s   |

This is to a speedup of approximately 4.1×, which is kind of reasonable i guess. Caculating 4 times as many pixels at the time, should correspond to a speedup of about 4 times. What makes me a bit confused is that it's more than 4. I dont know what happens in the background and what optimizations could cause this, or if it's just a coincidence of other factors. But I'll take it.



## Interactive Benchmark Reports

Criterion made some cool looking, interactive statistic sheets that you can view on here:

https://hallalay.github.io/




## Conclusion

By using SIMD to compute 4 pixels at the same time we manage to speed up the mandelbrot rendering to approcimatly 4 times the original speed of a scalar application.