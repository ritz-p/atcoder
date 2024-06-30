use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
    };
    let mut heap = BinaryHeap::new();
    for _i in 0..n {
        input! {
            q: usize,
        };
        match q {
            1 => {
                input! {
                    c: usize
                };
                heap.push(Reverse(c));
            }
            2 => {
                let Reverse(peek) = heap.peek().unwrap();
                println!("{}", peek);
            }
            3 => {
                heap.pop();
            }
            _ => {}
        }
    }
}
