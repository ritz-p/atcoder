use proconio::input;
use std::collections::{BTreeMap, BTreeSet};
fn main(){
    input!{
        n: usize,
        mut lr: [(usize,usize);n]
    };
    let mut bmap = BTreeMap::new();
    for (l,r) in lr{
        bmap.entry(l).or_insert_with(BTreeSet::new).insert(r);
    }
    let mut res = 0;
    for (map_index,b) in &bmap{
        for (set_index,s) in b.iter().enumerate(){
            res += bmap.range(..=s).count();
            res += b.len()-1-set_index;
            println!("{}",s);
        }
    }
    println!("{:?}",bmap);
    println!("{}",res);
}
