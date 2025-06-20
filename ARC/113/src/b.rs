use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        mut b: [usize;n]
    };
    let mut res = vec![];
    b.sort();
    for i in 0..n {
        let x = a[0] ^ b[i];
        let mut c = vec![];
        for j in 0..n {
            c.push(x ^ a[j]);
        }
        c.sort();
        if b == c {
            res.push(x);
        }
    }

    res.sort();
    res.dedup();
    println!("{}", res.len());
    println!("{}", res.iter().join("\n"));
}
