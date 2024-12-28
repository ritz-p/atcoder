use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        xyc: [(usize,usize,char);m]
    };
    let mut vblack_bmap = BTreeMap::new();
    let mut hblack_bmap = BTreeMap::new();
    let mut white_bset = BTreeSet::new();

    for (x, y, c) in xyc {
        match c {
            'B' => {
                vblack_bmap.entry(x).or_insert_with(BTreeSet::new).insert(y);
                hblack_bmap.entry(y).or_insert_with(BTreeSet::new).insert(x);
            }
            'W' => {
                white_bset.insert((x, y));
            }
            _ => {}
        }
    }
    for (x, y) in white_bset {
        if let Some(vblack_set) = vblack_bmap.range(x..).next() {
            if let Some(&min) = vblack_set.1.range(y..).next() {
                if min > y {
                    println!("No");
                    return;
                }
            }
        }

        if let Some(hblack_set) = hblack_bmap.range(y..).next() {
            if let Some(&min) = hblack_set.1.range(x..).next() {
                if min > x {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
