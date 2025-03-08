use std::f64::consts::PI;

use proconio::input;
fn main() {
    input! {
        n: f64,
        x0: f64,
        y0: f64,
        xh: f64,
        yh: f64
    };
    let x = (x0 + xh) / 2.0;
    let y = (y0 + yh) / 2.0;
    let theta = PI * 2.0 / n;
    let cos = theta.cos();
    let sin = theta.sin();
    let res_x = cos * (x0 - x) - sin * (y0 - y) + x;
    let res_y = sin * (x0 - x) + cos * (y0 - y) + y;
    println!("{:.11} {:.11}", res_x, res_y);
}
