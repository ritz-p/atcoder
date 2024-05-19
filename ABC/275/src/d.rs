use std::collections::HashMap;
 
use proconio::input;
 
fn main() {
    input! {
        n: usize
    };
    let mut memo = HashMap::new();
    memo.insert(0,1);
    println!("{}",  rec(n, &mut memo))
}
 
fn rec(n:usize, memo: &mut HashMap<usize, usize>) -> usize {
    return match memo.get(&n){
        None => {
            let v = rec(n/2, memo) + rec(n/3, memo);
            memo.insert(n,v);
            return v;
        },
        _ => *memo.get(&n).unwrap()
    }
    // if n == 0 {
    //     return 1
    // } else if memo.contains_key(&n) {
    //     return *memo.get(&n).unwrap()
    // } else {
    //     // println!("{}/2 {}/3",n/2,n/3);
    //     let v = rec(n/2, memo) + rec(n/3, memo);
    //     memo.insert(n, v);
    //     return v
    // }
}