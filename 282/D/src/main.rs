use proconio::input;
use std::collections::HashSet;

fn main(){
    input!{
        n: usize,
        m: usize,
        tup: [(usize,usize);m],
    };
    let mut arr_hashset = vec![HashSet::new();n];
    for i in 0..m{
        arr_hashset[tup[i].0-1].insert(tup[i].1);
    }

    println!("{:?}",arr_hashset);
}