use proconio::input;
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };
    let mut r = vec![0; n + 1];
    let mut b = vec![0; n + 1];
    r[1] = 0;
    b[1] = 1;
    for i in 2..=n {
        b[i] = r[i - 1] + b[i - 1] * y;
        r[i] = r[i - 1] + b[i] * x;
    }

    println!("{}", r[n]);
}
