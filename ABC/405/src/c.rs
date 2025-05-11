use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let sum: usize = a.iter().sum();
    let mut res = 0;
    for e in a {
        res += e * (sum - e);
    }
    println!("{}", res / 2);
}
