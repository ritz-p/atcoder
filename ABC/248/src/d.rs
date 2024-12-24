use proconio::input;
use superslice::*;
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        q: usize,
    };
    let mut v = vec![vec![]; n + 1];

    for (index, e) in a.iter().enumerate() {
        v[*e].push(index + 1);
    }
    for _i in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        };
        let lc = v[x].lower_bound(&l);
        let rc = v[x].lower_bound(&(r + 1));
        println!("{}", rc - lc);
    }
}
