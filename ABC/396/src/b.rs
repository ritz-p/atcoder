use std::collections::VecDeque;

use proconio::input;
fn main() {
    input! {
        q: usize
    };
    let v = vec![0; 100];
    let mut queue = VecDeque::from(v);
    for _i in 0..q {
        input! {
            n: usize,
        };
        match n {
            1 => {
                input! {
                    query: usize
                };
                queue.push_back(query);
            }
            2 => {
                if let Some(value) = queue.pop_back() {
                    println!("{}", value);
                }
            }
            _ => {}
        }
    }
}
