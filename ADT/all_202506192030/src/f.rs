use std::collections::{HashSet, VecDeque};

use proconio::input;
fn main() {
    input! {
        n: usize,
    };

    let mut graph = vec![];
    let mut tv = vec![];

    for _i in 0..n {
        input! {
            t: usize,
            k: usize,
            a: [usize;k]
        };
        tv.push(t);
        graph.push(a);
    }
    let mut res = 0;
    let mut queue = VecDeque::new();

    queue.push_back(n);
    let mut set = HashSet::new();

    while let Some(v) = queue.pop_front() {
        res += tv[v - 1];

        for g in &graph[v - 1] {
            if !set.contains(&g) {
                queue.push_back(*g);
                set.insert(g);
            }
        }
    }

    println!("{}", res);
}
