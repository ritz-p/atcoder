use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    if m != n {
        println!("No");
        return;
    }

    let mut graph = vec![vec![]; n];

    for (a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    for g in &graph {
        if g.len() != 2 {
            println!("No");
            return;
        }
    }
    let mut visited = HashSet::new();
    dfs(&graph, &mut visited, 0);

    if visited.len() == n {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>, index: usize) {
    if visited.contains(&index) {
        return;
    }
    visited.insert(index);
    for &g in &graph[index] {
        dfs(graph, visited, g);
    }
}
