use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
    }

    let mut heap = BinaryHeap::new();

    for a in a {
        heap.push(Reverse(a));
        if heap.len() > k {
            heap.pop();
        }
        if heap.len() == k {
            let Reverse(ans) = heap.peek().unwrap();
            println!("{}", ans);
        }
    }
}
