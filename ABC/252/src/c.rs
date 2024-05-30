use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut res = std::usize::MAX;
    for y in "0123456789".chars() {
        let mut bmap = BTreeMap::new();
        for x in s.iter() {
            let f = x.find(y).unwrap();
            *bmap.entry(f).or_insert(0) += 1;
        }
        let mut mx = 0;
        for (&v, &x) in bmap.iter() {
            mx = mx.max(v + 10 * (x - 1));
        }
        res = res.min(mx);
    }

    println!("{}",res);
}
