use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64,f64);n]
    };
    let mut res = distance(0.0, 0.0, xy[0].0, xy[0].1);
    for i in 0..n - 1 {
        res += distance(xy[i].0, xy[i].1, xy[i + 1].0, xy[i + 1].1);
    }
    res += distance(xy[n - 1].0, xy[n - 1].1, 0.0, 0.0);

    println!("{}", res);
}

fn distance(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let x = a - c;
    let y = b - d;
    (x * x + y * y).sqrt()
}
