use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        k: usize,
        arr: [usize;n]
    };
    let mut res = k * (k + 1) / 2;
    let set:HashSet<usize> = arr.iter().cloned().collect();
    for a in set{
        if a <= k{
            res -= a;
        }
    }
    println!("{}",res);
}
