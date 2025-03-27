use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sp: [(String,usize);n]
    };
    let mut v = vec![];
    for (index, (s, p)) in sp.iter().enumerate() {
        v.push((index, s, p));
    }
    v.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| b.2.cmp(&a.2)));

    println!("{}", v.iter().map(|value| value.0 + 1).join("\n"));
}
