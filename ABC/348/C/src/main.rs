use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
    };
    let mut map:HashMap<usize,usize> = HashMap::new();
    for _i in 0..n{
        input!{
            a: usize,
            c: usize,
        };
        if let Some(k) = map.get(&c){
            if a < *k{
                map.insert(c,a);
            }
        }else{
            map.insert(c,a);
        }
    }
    let mut v:Vec<usize> = map.values().cloned().collect::<Vec<usize>>();
    v.sort();
    v.reverse();
    println!("{}",v[0]);
}
