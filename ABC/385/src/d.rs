use std::collections::{BTreeMap, BTreeSet, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut sx: isize,
        mut sy: isize,
        xy: [(isize,isize);n],
        dc: [(char,isize);m]
    };
    let mut vert = BTreeMap::new();
    let mut hori = BTreeMap::new();

    for (x, y) in xy {
        vert.entry(x).or_insert(BTreeSet::new()).insert(y);
        hori.entry(y).or_insert(BTreeSet::new()).insert(x);
    }
    let mut set = HashSet::new();
    for (d, c) in dc {
        match d {
            'U' => {
                if let Some(bset) = vert.get(&sx) {
                    let mut iter = bset.range(sy + 1..=sy + c);
                    while let Some(v) = iter.next() {
                        set.insert((sx, *v));
                    }
                }
                sy += c;
            }
            'D' => {
                if let Some(bset) = vert.get(&sx) {
                    let mut iter = bset.range(sy - c..=sy - 1);
                    while let Some(v) = iter.next() {
                        set.insert((sx, *v));
                    }
                }
                sy -= c;
            }
            'L' => {
                if let Some(bset) = hori.get(&sy) {
                    let mut iter = bset.range(sx - c..=sx - 1);
                    while let Some(v) = iter.next() {
                        set.insert((*v, sy));
                    }
                }
                sx -= c;
            }
            'R' => {
                if let Some(bset) = hori.get(&sy) {
                    let mut iter = bset.range(sx + 1..=sx + c);
                    while let Some(v) = iter.next() {
                        set.insert((*v, sy));
                    }
                }
                sx += c;
            }
            _ => {}
        }
    }

    println!("{} {} {}", sx, sy, set.len());
}
