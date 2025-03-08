use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize,usize,usize);m]
    };
    let mut graph = vec![vec![]; n];
    let mut bset = BTreeSet::new();
    for (u, v, w) in uvw {
        graph[u - 1].push((v - 1, w));
        graph[v - 1].push((u - 1, w));
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    dfs(0, n - 1, &mut visited, &graph, 0, &mut bset);

    if let Some(&xor) = bset.iter().next() {
        println!("{}", xor);
    }
}

fn dfs(
    index: usize,
    goal: usize,
    visited: &mut Vec<bool>,
    graph: &Vec<Vec<(usize, usize)>>,
    xor: usize,
    bset: &mut BTreeSet<usize>,
) {
    if index == goal {
        bset.insert(xor);
        return;
    }
    for &(next, w) in &graph[index] {
        if !visited[next] {
            visited[next] = true;
            dfs(next, goal, visited, graph, xor ^ w, bset);
            visited[next] = false;
        }
    }
}
