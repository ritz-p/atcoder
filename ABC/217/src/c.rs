use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize;n]
    };
    let mut res = vec![0; n];
    for (index, e) in p.iter().enumerate() {
        res[e - 1] = index + 1;
    }
    println!("{}", res.iter().join(" "));
}
