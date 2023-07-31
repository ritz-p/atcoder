use std::collections::VecDeque;
use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
    }
    let mut g = vec![vec![]; n1 + n2];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        g[a-1].push(b);
        g[b-1].push(a);
    }

}
