use std::cmp::min;
use proconio::input;
fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
 
    let mut step2 = 0;
    let mut step1 = (h[1] - h[0]).abs();
    for i in 2..n {
        let min_cost = min(
            step1 + (h[i] - h[i - 1]).abs(),
            step2 + (h[i] - h[i - 2]).abs(),
        );
        step2 = step1;
        step1 = min_cost;
    }
 
    println!("{}", step1);
}