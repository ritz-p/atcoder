use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut root3: usize = 1;
    while root3.pow(3) <= n {
        root3 += 1;
    }
    let mut res = 0;
    for a in 1..root3 {
        for b in a..=(n as f64 / a as f64).sqrt() as usize {
            res += n / (a * b) - b + 1;
        }
    }
    println!("{}", res);
}
