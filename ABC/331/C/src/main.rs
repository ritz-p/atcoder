use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut arr = a.clone();
    arr.sort();
    let mut v = vec![0;1000001];
    let mut hash = HashMap::new();
    for i in &arr{
        v[*i] += 1;
    }
    let mut sum:usize = arr.iter().sum();
    arr.dedup();
    for i in &arr{
        hash.insert(i,sum - i * v[*i]);
        sum = sum - i * v[*i];
    }
    for i in 0..n{
        print!("{} ",hash.get(&a[i]).unwrap());
    }
}
