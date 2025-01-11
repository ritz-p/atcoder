use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    };
    let mut bmap = BTreeMap::new();
    for (index, e) in a.iter().enumerate() {
        bmap.insert(e, index);
    }
    let mut res = 0;
    let max = bmap.get(&a[a.len() - 1]).unwrap();

    for e in &a {
        let mut iter = bmap.range(..e * 2);
        if let Some((_k, v)) = iter.next_back() {
            res += max - v;
        }
    }
    println!("{}", res);
}
