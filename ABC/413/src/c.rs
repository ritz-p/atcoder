use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        q: usize,
    };

    let mut queue = VecDeque::new();
    let mut res = vec![];

    for _ in 0..q {
        input! {
            qt: usize,
        };
        match qt {
            1 => {
                input! {
                    c: usize,
                    x: usize,
                };
                queue.push_back((c, x));
            }
            2 => {
                let mut count = 0;
                input! {
                    mut k: usize,
                };
                while k > 0 {
                    if let Some((c, x)) = queue.pop_front() {
                        if c >= k {
                            count += x * k;
                            queue.push_front((c - k, x));
                            k = 0;
                        } else {
                            count += x * c;
                            k -= c;
                        }
                    }
                }
                res.push(count);
            }
            _ => {}
        }
    }
    println!("{}", res.iter().join("\n"));
}
