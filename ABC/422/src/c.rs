use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        mut abc: [(usize,usize,usize);n]
    };
    let mut res = vec![];
    for (a, b, c) in abc {
        let current = a.min(c.min((a + b + c) / 3));
        res.push(current);
    }

    println!("{}", res.iter().join("\n"));
}
