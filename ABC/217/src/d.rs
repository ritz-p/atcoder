use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize,usize);q]
    };
    let mut bset = BTreeSet::new();

    for (c, x) in &cx {
        match c {
            1 => {
                bset.insert(x);
            }
            2 => {
                let high = bset.range(*x..).next();
                let low = bset.range(..=*x).next_back();
                if let (Some(high), Some(low)) = (high, low) {
                    println!("{}", *high - *low);
                } else if let Some(high) = high {
                    println!("{}", high);
                } else if let Some(low) = low {
                    println!("{}", l - *low);
                } else {
                    println!("{}", l);
                }
            }
            _ => {}
        }
    }
}
