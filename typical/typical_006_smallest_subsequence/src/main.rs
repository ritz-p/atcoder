use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut s: Chars
    }
    let mut heap = BinaryHeap::new();
    let mut next = 0;
    let mut ans:Vec<char> = vec![];

    for (i,c) in s.iter().enumerate(){
        heap.push(Reverse((c,i)));
        if n <= i + k{
            loop{
                let Reverse((d,j)) = heap.pop().unwrap();
                if next <= j{
                    next = j + 1;
                    ans.push(*d);
                    break;
                }
            }
        }
    }
    let ans_s: String = ans.iter().collect();
    println!("{}",ans_s);
}
