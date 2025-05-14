use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut query = vec![];
    for _i in 0..n {
        input! {
            t: usize,
            k: usize,
            a: [usize;k]
        };
        query.push((t, a));
    }
    let mut res = query[n - 1].0;
    let mut queue = VecDeque::from_iter(query[n - 1].1.iter());
    let mut set = HashSet::new();

    while let Some(v) = queue.pop_front() {
        if set.contains(&(v - 1)) {
            continue;
        }
        set.insert(v - 1);
        res += query[v - 1].0;
        for q in &query[*v - 1].1 {
            queue.push_back(q);
        }
    }

    println!("{}", res);
}
