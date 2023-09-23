use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    proconio::input! {
        n: usize,
        q: [(usize, usize, usize)]
    }

    let res = {
        let mut eating_heap = BinaryHeap::new();
        let mut waiting_heap = BinaryHeap::from_iter((0..n).map(Reverse));
        let mut weights = vec![0; n];

        for (t, w, s) in q {
            while let Some((Reverse(ti), pi)) = eating_heap.peek().copied() {
                if ti > t {
                    break;
                } else {
                    waiting_heap.push(Reverse(pi));
                    eating_heap.pop();
                }
            }
            if let Some(Reverse(head)) = waiting_heap.pop() {
                eating_heap.push((Reverse(t + s), head));
                weights[head] += w;
            }
        }
        weights
    };

    let ans = res.into_iter().join("\n");
    println!("{}", ans);
}
