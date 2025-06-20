use proconio::input;
fn main() {
    input! {
        k: usize,
    };
    let mut res = 0;
    for a in 1..=k {
        for b in 1..=k / a {
            let c = k / (a * b);
            res += c;
        }
    }

    println!("{}", res);
}
