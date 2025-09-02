use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut res = vec![0; 2 * n + 1];
    for (index, e) in a.iter().enumerate() {
        res[2 * index + 1] = res[e - 1] + 1;
        res[2 * index + 2] = res[e - 1] + 1;
    }

    println!("{}", res.iter().join("\n"));
}
