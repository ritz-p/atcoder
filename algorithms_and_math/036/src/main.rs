use proconio::input;
use std::f64::consts::PI;
 
fn main() {
  input! {
    a: f64,
    b: f64,
    h: f64,
    m: f64,
  }
 
  let a_rad = 2.0 * PI * m / 60.0;
  let ax = a * a_rad.cos();
  let ay = a * a_rad.sin(); 
  let b_rad = 2.0 * PI * (h / 12.0) + (2.0 * PI / 12.0) * (m / 60.0);
  let bx = b * b_rad.cos();
  let by = b * b_rad.sin();
  let res = ((bx-ax).powf(2.0)+(by-ay).powf(2.0)).sqrt();
 
  println!("{}", res);
}