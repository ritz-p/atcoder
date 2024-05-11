use proconio::input;
use std::collections::BinaryHeap;
fn main(){
    input!{
        n: usize,
        d: usize,
        mut xy: [(usize,usize);n]
    };

    let mut heap = BinaryHeap::new();
    xy.sort_by_key(|x|x.0);
    let mut count = 0;
    let mut res = 0;
    for i in 1..=d{
        while count < n && xy[count].0 <= i{
            heap.push(xy[count].1);
            count += 1;
        }
        if !heap.is_empty(){
            res += heap.pop().unwrap();
        }
    }
    println!("{}",res);
}