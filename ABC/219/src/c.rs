use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars;n]
    };

    let alphabet = ('a'..='z').collect::<Vec<char>>();
    let mut cmap = BTreeMap::new();
    for (index, c) in x.iter().enumerate() {
        cmap.insert(c, alphabet[index]);
    }
    let mut smap = BTreeMap::new();

    for cs in s {
        let mut res = vec![];
        for c in &cs {
            if let Some(v) = cmap.get(&c) {
                res.push(*v);
            }
        }
        smap.insert(res, cs);
    }

    println!(
        "{}",
        smap.values()
            .map(|s| s.iter().collect::<String>())
            .join("\n")
    );
}
