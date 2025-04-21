use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut queue = VecDeque::new();

    for i in 0..q {
        input! {
            n: usize,
        };
        match n {
            1 => {
                input! {
                    x: usize
                };
                queue.push_back(x);
            }
            2 => {
                if let Some(v) = queue.pop_front() {
                    println!("{}", v);
                }
            }
            _ => {}
        }
    }
}
