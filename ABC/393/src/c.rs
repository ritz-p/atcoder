use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize,usize);m]
    };
    let mut res = 0;
    let mut graph = vec![HashSet::new(); n + 1];
    for (u, v) in uv {
        if graph[u].contains(&v) || graph[v].contains(&u) || u == v {
            res += 1;
            continue;
        }
        graph[u].insert(v);
        graph[v].insert(u);
    }

    println!("{}", res);
}
