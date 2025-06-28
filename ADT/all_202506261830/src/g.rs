use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut v = vec![(usize::MAX, usize::MAX); n];

    let mut res = vec![];

    for _i in 0..q {
        input! {
            qt: usize,
        };

        match qt {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                };
                v[x - 1].1 = y - 1;
                v[y - 1].0 = x - 1;
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                };
                v[x - 1].1 = usize::MAX;
                v[y - 1].0 = usize::MAX;
            }
            3 => {
                input! {
                    x: usize,
                };
                let mut current = x - 1;

                let mut rc = VecDeque::new();

                while current != usize::MAX {
                    rc.push_back(current + 1);
                    current = v[current].1;
                }
                current = v[x - 1].0;
                while current != usize::MAX {
                    rc.push_front(current + 1);
                    current = v[current].0;
                }
                res.push(rc);
            }
            _ => {}
        }
    }

    for r in res {
        println!("{} {}", r.len(), r.iter().join(" "));
    }
}
