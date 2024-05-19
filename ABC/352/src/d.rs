use proconio::input;
use std::collections::BTreeSet;
fn main(){
    input!{
        n: usize,
        k: usize,
        p: [usize;n]
    };
    let mut v = vec![0;n];
    for (index,e) in p.iter().enumerate(){
        v[*e-1] = index;
    }
    let mut set = BTreeSet::new();
    for i in 0..k{
        set.insert(v[i]);
    }
    let mut res = set.last().unwrap() - set.first().unwrap();
    for i in k..n{
        set.remove(&v[i-k]);
        set.insert(v[i]);
        res = res.min(set.last().unwrap() - set.first().unwrap());
    }
    println!("{}",res);
}
