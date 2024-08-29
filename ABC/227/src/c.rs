use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut root3: usize = 1;
    while root3.pow(3) <= n {
        root3 += 1;
    }

    let mut ans = 0;

    for i in 1..=root3 {
        for j in i..=(n as f64 / i as f64).sqrt() as usize {
            ans += n / (i * j) - j + 1;
        }
    }

    println!("{}", ans);
}
