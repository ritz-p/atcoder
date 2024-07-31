use proconio::input;
use proconio::marker::Chars;
use std::collections::{BTreeMap, HashMap};
fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars;n]
    };
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut map = HashMap::new();
    let mut bmap = BTreeMap::new();
    for (index, e) in x.iter().enumerate() {
        map.insert(e, alphabet[index]);
    }
    for e in s {
        let mut ss = vec![];
        for c in &e {
            if let Some(cc) = map.get(&c) {
                ss.push(*cc);
            }
        }
        bmap.insert(ss.iter().collect::<String>(), e);
    }
    for v in bmap.values() {
        println!("{}", v.iter().collect::<String>());
    }
}
