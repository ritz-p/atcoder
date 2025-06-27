use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;
fn main() {
    input! {
        n: usize,
        mut td: [(usize,usize);n]
    };

    td.sort();
    td.reverse();

    let mut heap = BinaryHeap::new();
    let mut time = 0;
    let mut res = 0;
    while !heap.is_empty() || !td.is_empty() {
        if heap.is_empty() {
            if let Some(&(t, _d)) = td.last() {
                time = t;
            }
        }

        while let Some(&(t, d)) = td.last() {
            if t <= time {
                heap.push(Reverse(t + d));
                td.pop();
            } else {
                break;
            }
        }

        while let Some(Reverse(x)) = heap.pop() {
            if time <= x {
                res += 1;
                break;
            }
        }
        time += 1;
    }

    println!("{}", res);
}
