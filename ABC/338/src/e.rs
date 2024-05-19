use proconio::input;
use std::mem::swap;
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut v = vec![(0, false); 2 * n];
    for (index, (mut a, mut b)) in ab.into_iter().enumerate() {
        if a > b {
            swap(&mut a, &mut b);
        }
        v[a-1] = (index, true); // 開始点
        v[b-1] = (index, false); // 終了点
    }

    let mut stack = vec![];
    for i in 0..2 * n {
        if v[i].1 {
            stack.push(v[i].0);
        } else if stack.pop() != Some(v[i].0) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
