use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        n: usize
    };
    let mut que = VecDeque::new();
    for _i in 0..n {
        input! {
            q: usize,
        };
        match q {
            1 => {
                input! {
                    s: String
                };
                que.push_back(s);
            }
            2 => {
                println!("{}", que[0]);
            }
            3 => {
                que.pop_front();
            }
            _ => {}
        }
    }
}
