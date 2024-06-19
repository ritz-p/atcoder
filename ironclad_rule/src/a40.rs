use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut map = HashMap::new();

    for e in a{
        *map.entry(e).or_insert(0) += 1;
    }
    let mut res = 0;
    for value in map.values(){
        if value >= &3{
            res += value * (value - 1) * (value - 2) / 6;
        }
    }

    println!("{}",res);
}