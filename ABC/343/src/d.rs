use proconio::input;
use std::collections::{HashMap,HashSet};
fn main(){
    input!{
        n: usize,
        t: usize
    };
    let mut hash:HashMap<usize,HashSet<usize>> = HashMap::new();
    hash.insert(0,(1..n).into_iter().collect());
    let mut ns = vec![0;n+1];
    for i in 0..t{
        input!{
            a: usize,
            b: usize
        };
        if hash.contains_key(&ns[a]){
            if hash[&ns[a]].contains(&a){
                hash.get_mut(&ns[a]).unwrap().remove(&a);
                if hash.get(&ns[a]).unwrap().len() == 0{
                    hash.remove(&ns[a]);
                }
            }
        }
        ns[a] += b;
        if hash.contains_key(&ns[a]){
            hash.get_mut(&ns[a]).unwrap().insert(a);
        }else{
            hash.insert(ns[a],vec![a].into_iter().collect());
        }
        println!("{}",n.min(hash.len()));
    }
}
