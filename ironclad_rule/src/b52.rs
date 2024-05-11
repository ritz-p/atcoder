use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
fn main(){
    input!{
        n: usize,
        x: usize,
        mut s: Chars,
    };

    let mut que = VecDeque::new();
    que.push_back(x-1);
    s[x-1] = '@';
    loop{
        if que.is_empty(){
            break;
        }
        let q = que.pop_front().unwrap();
        if q > 0{
            if s[q-1] == '.'{
                s[q-1] = '@';
                que.push_back(q-1);
            }
        }
        if q < n-1{
            if s[q+1] == '.'{
                s[q+1] = '@';
                que.push_back(q+1);
            }
        }
    }
    println!("{}",s.iter().join(""));
}