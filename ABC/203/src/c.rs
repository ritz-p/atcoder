use proconio::input;
use std::collections::BTreeMap;
fn main(){
    input!{
        n: usize,
        k: usize,
        ab: [(usize,usize);n]
    };
    let mut res = k;
    let mut bmap = BTreeMap::new();
    for (a,b) in ab{
        *bmap.entry(a).or_insert(0) += b;
    }

    for (key,v) in &bmap{
        if res < *key{
            println!("{}",res);
            return;
        }
        res += v;
    }
    println!("{}",res);
}
