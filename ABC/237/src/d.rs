use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::VecDeque;
fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut res = VecDeque::new();
    res.push_back(n);

    for (index, c) in s.iter().rev().enumerate() {
        match *c {
            'R' => {
                res.push_front(n - index - 1);
            }
            'L' => {
                res.push_back(n - index - 1);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", res.iter().join(" "));
}
