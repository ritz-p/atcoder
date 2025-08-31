use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    for i in 0..n {
        let mut set = HashSet::new();
        let mut res = HashSet::new();
        set.insert(i);
        for g in &graph[i] {
            set.insert(*g);
        }

        for g1 in &graph[i] {
            for g2 in &graph[*g1] {
                if !set.contains(g2) {
                    res.insert(g2);
                }
            }
        }

        println!("{}", res.len());
    }
}
