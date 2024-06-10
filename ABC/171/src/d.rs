use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        a: [usize;n],
        q: usize,
        bc: [(usize,usize);q]
    };
    let mut res:usize = a.iter().sum();
    let mut map = HashMap::new();

    for e in a{
        *map.entry(e).or_insert(0) += 1;
    }

    for (b,c) in bc{
        if let Some(be) = map.get(&b){
            if b > c{
                res -= (b-c) * be;
            }else if c > b{
                res += (c-b) * be;
            }
            *map.entry(c).or_insert(0) += be.clone();
            map.remove(&b);
        }
        println!("{}",res);
    }
}
