use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize,usize);n-1],
        v: [Usize1;k]
    };
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let start = v[0];
    let v = v.into_iter().collect::<HashSet<usize>>();
    let mut visited = vec![false; n];
    let res = dfs(&graph, &v, &mut visited, start);
    println!("{}", res);
}

fn dfs(g: &[Vec<usize>], v: &HashSet<usize>, visited: &mut [bool], node: usize) -> i64 {
    let mut cnt = 0;

    visited[node] = true;

    for &nnode in &g[node] {
        if visited[nnode] {
            continue;
        }
        cnt += dfs(g, v, visited, nnode);
    }

    if v.contains(&node) || cnt != 0 {
        cnt += 1;
    }

    cnt
}
