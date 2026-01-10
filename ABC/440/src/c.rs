use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        t: usize,
    };
    let mut res = vec![];
    for _ in 0..t {
        input! {
            n: usize,
            w: usize,
            c: [usize;n]
        };
        let mut v = vec![0; 2 * w];
        for (i, e) in c.iter().enumerate() {
            v[i % (2 * w)] += e;
        }
        let mut sv = vec![0; 3 * w + 1];
        for i in 0..3 * w {
            sv[i + 1] = sv[i] + v[i % (2 * w)];
        }
        let mut r = usize::MAX;
        for i in 0..2 * w {
            r = r.min(sv[i + w] - sv[i]);
        }
        res.push(r);
    }
    println!("{}", res.iter().join("\n"));
}
