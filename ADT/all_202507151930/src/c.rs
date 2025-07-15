use proconio::input;
fn main() {
    input! {
        n: usize,
        mut x: [usize;n*5]
    };
    x.sort();
    let sum = x.iter().skip(n).take(n * 3).sum::<usize>();
    let res = sum as f64 / (3.0 * n as f64);
    println!("{}", res);
}
