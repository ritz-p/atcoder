use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize,usize);m]
    };
    let mut graph = vec![vec![]; n];

    for (u, v) in &uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut target = HashSet::new();

    for i in 0..n {
        target.insert(i);
        let mut delete = HashSet::new();
        let mut visited = HashSet::new();
        dfs(&graph, 0, &mut delete, &target, &mut visited);

        if target.len() == visited.len() {
            println!("{}", delete.len());
        } else {
            println!("-1");
        }
    }
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    index: usize,
    delete: &mut HashSet<usize>,
    target: &HashSet<usize>,
    visited: &mut HashSet<usize>,
) {
    if visited.contains(&index) {
        return;
    }
    visited.insert(index);
    for g in &graph[index] {
        if target.contains(&g) {
            dfs(graph, index, delete, target, visited);
        } else {
            delete.insert(*g);
        }
    }
}
