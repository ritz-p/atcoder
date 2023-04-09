use proconio::input;
use std::collections::BinaryHeap;

const INF: isize = 1_000_000_000_000;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [isize; n]
    }
    let mut que = BinaryHeap::new();
    que.push(0);
    let mut ans: Vec<isize> = vec![INF; k + 1];
    for i in 0..k + 1 {
        let mut v: isize = que.pop().unwrap() * -1;
        if i > 0 {
            if ans[i - 1] == v {
                while ans[i - 1] == v {
                    v = que.pop().unwrap() * -1;
                }
            }
        }
        ans[i] = v;
        for j in 0..n {
            que.push(-(v + a[j]));
        }
    }
    println!("{}", ans[k]);
}
