use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
       n:usize,
       m:usize,
    }
    let mut a = VecDeque::new();
    let mut b = VecDeque::new();
    for _ in 0..n {
        input! {input:usize};
        a.push_back(input);
    }
    for _ in 0..m {
        input! {input:usize};
        b.push_back(input);
    }
    let mut ans1 = String::new();
    let mut ans2 = String::new();
    for i in 1..=n + m {
        if a.len() == 0 {
            ans2 += &i.to_string();
            ans2 += " ";
        } else if b.len() == 0 {
            ans1 += &i.to_string();
            ans1 += " ";
        } else if a.front() < b.front() {
            ans1 += &i.to_string();
            ans1 += " ";
            a.pop_front();
        } else {
            ans2 += &i.to_string();
            ans2 += " ";
            b.pop_front();
        }
    }
    println!("{}",ans1);
    println!("{}",ans2);
}
