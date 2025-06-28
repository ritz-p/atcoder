use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        q: usize,
    };

    let mut queue = VecDeque::new();

    let mut current = 0;

    let mut res = vec![];

    for _ in 0..q {
        input! {
            qt: usize,
        };

        match qt {
            1 => {
                input! {
                    l: usize,
                };
                if let Some((_, cc)) = queue.iter().last() {
                    queue.push_back((l, cc + l));
                } else {
                    queue.push_back((l, l));
                }
            }
            2 => {
                if let Some((l, _)) = queue.pop_front() {
                    current += l;
                }
                if queue.len() == 0 {
                    current = 0;
                }
            }
            3 => {
                input! {
                    k: usize,
                };
                if k == 1 {
                    res.push(0);
                } else {
                    res.push(queue[k - 2].1 - current);
                }
            }
            _ => {}
        }
    }
    println!("{}", res.iter().join("\n"));
}
