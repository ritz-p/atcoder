use std::{
    collections::{BTreeSet, VecDeque},
    iter::FromIterator,
};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let l = s.len();
    let mut bset = BTreeSet::new();
    let mut queue = VecDeque::from_iter(s);
    for _i in 0..=l {
        bset.insert(queue.iter().collect::<String>());
        let c = queue.pop_front();
        if let Some(v) = c {
            queue.push_back(v);
        }
    }

    if let Some(v) = bset.first() {
        println!("{}", v);
    }
    if let Some(v) = bset.last() {
        println!("{}", v);
    }
}
