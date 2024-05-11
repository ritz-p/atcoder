use std::collections::HashMap;

use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut map:HashMap<usize,usize> = HashMap::new();
    for i in a{
        *map.entry(i).or_insert(0) += 1;
    }
    let mut res = 0;
    for e in map{
        if e.1 > 1{
            res += ((e.1) * (e.1-1)) / 2;
        }
    }
    println!("{}",res);
}