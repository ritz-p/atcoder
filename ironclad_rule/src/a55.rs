use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: [(usize,usize);n]
    };
    let mut bset = BTreeSet::new();
    for (query, number) in q {
        match query {
            1 => {
                bset.insert(number);
            }
            2 => {
                bset.remove(&number);
            }
            3 => {
                let mut p = bset.range(number..);
                if let Some(v) = p.next() {
                    println!("{}", v);
                } else {
                    println!("-1");
                }
            }
            _ => {}
        }
    }
}
