use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        arr: [usize;n],
    };
    let mut hashmap = HashMap::new();
    for x in arr {
        let counter = hashmap.entry(x).or_insert(0);
        *counter += 1;
    }
    let mut res = 0;
    for (_,value) in hashmap{
        res += value / 2;
    }
    println!("{}", res);
}
