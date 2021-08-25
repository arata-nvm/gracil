use std::time::Duration;

use image::{ImageBuffer, RgbImage};
use mexprp::{Answer, Context, Expression};
use pbr::ProgressBar;
use rug::Complex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    expr: String,

    #[structopt(short, default_value = "512")]
    size: u32,

    #[structopt(short, default_value = "1.0")]
    range: f64,

    #[structopt(short, default_value = "1")]
    mode: usize,
}

fn main() {
    let opt = Opt::from_args();
    let expr = Expression::parse(&opt.expr).unwrap();
    plot(expr, opt.size, opt.range, opt.mode);
}

fn plot(expr: Expression<Complex>, size: u32, range: f64, mode: usize) {
    let step = (range * 2.0) / size as f64;
    let mut ctx: Context<Complex> = Context::new();
    let mut img: RgbImage = ImageBuffer::new(size, size);
    let mut pb = ProgressBar::new((size * size) as u64);
    pb.set_max_refresh_rate(Some(Duration::from_millis(500)));

    for y in 0..size {
        for x in 0..size {
            pb.inc();
            let zx = -range + step * x as f64;
            let zy = -range + step * y as f64;
            let z = Complex::with_val(53, (zx, zy));
            ctx.set_var("z", z);

            let ans = match expr.eval_ctx(&ctx).unwrap() {
                Answer::Single(ans) => ans,
                Answer::Multiple(ans) => ans[0].clone(),
            };
            let color = match mode {
                1 => complex2color(ans),
                _ => complex2color2(ans),
            };
            let (r, g, b) = color.to_rgb();
            img.put_pixel(x, y, image::Rgb([r, g, b]));
        }
    }

    img.save("output.png").unwrap();
}

fn complex2color(z: Complex) -> hsl::HSL {
    let h_rad = z.clone().arg().real().to_f64();
    let h = (h_rad.to_degrees() + 360.0) % 360.0;
    let l = (1.0 - 2.0f64.powf(-z.abs().real().to_f64())) * 0.5;
    let s = 1.0;

    hsl::HSL { h, s, l }
}

fn complex2color2(z: Complex) -> hsl::HSL {
    let x = z.clone().real().to_f64();
    let y = z.clone().imag().to_f64();

    let hx = -((x - x.round()).abs() + 0.001).log(std::f64::consts::E);
    let hy = -((y - y.round()).abs() + 0.001).log(std::f64::consts::E);
    let h = 360.0 - (hx + hy) * 20.0;

    hsl::HSL { h, s: 1.0, l: 0.5 }
}
