use proconio::input;
fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let mut res = 0;
    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || i.abs_diff(j) >= y {
                res += 1;
            }
        }
    }
    println!("{}", res as f64 / 36.0);
}
