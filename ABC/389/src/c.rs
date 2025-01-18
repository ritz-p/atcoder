use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize
    };
    let mut queue = VecDeque::new();
    let mut current = 0;
    let mut total = 0;
    for _i in 0..q {
        input! {
            query: usize,
        };
        match query {
            1 => {
                input! {
                    l: usize,
                };
                total += l;
                queue.push_back((l, total));
            }
            2 => {
                if let Some((l, _total)) = queue.pop_front() {
                    current += l;
                }
            }
            3 => {
                input! {
                    k: usize
                };
                println!("{}", queue[k - 1].1 - queue[k - 1].0 - current);
            }
            _ => {}
        }
    }
}
