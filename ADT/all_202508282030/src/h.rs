use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize,usize,usize);m]
    };
    let mut graph = vec![vec![]; n];
    for (u, v, w) in uvw {
        graph[u - 1].push((v - 1, w));
        graph[v - 1].push((u - 1, w));
    }

    let mut res = usize::MAX;
    let mut set = HashSet::new();
    set.insert(0);
    dfs(&graph, &mut set, &mut res, 0, 0, n - 1);
    println!("{}", res);
}

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    set: &mut HashSet<usize>,
    res: &mut usize,
    index: usize,
    mut current: usize,
    goal: usize,
) {
    if index == goal {
        *res = *res.min(&mut current);
    }
    for (g, w) in &graph[index] {
        if !set.contains(g) {
            set.insert(*g);
            dfs(graph, set, res, *g, current | w, goal);
            set.remove(g);
        }
    }
}
