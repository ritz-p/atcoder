use std::{
    collections::{BTreeSet, VecDeque},
    iter::FromIterator,
};

use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars
    };
    let mut s = VecDeque::from_iter(s.into_iter());
    let l = s.len();
    let mut bset = BTreeSet::new();
    for _i in 0..l {
        let c = s.pop_front().unwrap();
        s.push_back(c);
        bset.insert(s.iter().collect::<String>());
    }

    println!("{}", bset.first().unwrap());
    println!("{}", bset.last().unwrap());
}
