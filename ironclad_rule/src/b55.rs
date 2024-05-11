use std::collections::BTreeSet;

use proconio::input;

fn main(){
    input!{
        n: usize,
        q: [(usize,isize);n]
    };

    let mut bset = BTreeSet::new();

    for query in q{
        if query.0 == 1{
            bset.insert(query.1);
        }else{
            let p1 = bset.range(..=query.1);
            let mut p2 = bset.range(query.1..);
            let mut res = isize::MAX;
            if let Some(v1) = p1.rev().next(){
                res = res.min((v1 - query.1).abs());
            }
            if let Some(v2) = p2.next(){
                res = res.min((v2 - query.1).abs());
            }
            if res == isize::MAX{
                println!("-1");
            }else{
                println!("{}",res);
            }
        }
    }
}