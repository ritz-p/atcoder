use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize);n-1]
    };
    let mut graph = vec![vec![]; n];

    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    for g in graph.iter_mut() {
        g.sort();
    }
    let mut route = vec![];
    let mut visited = vec![false; n];
    dfs(&graph, &mut visited, 0, &mut route);

    println!("{}", route.iter().map(|v| v + 1).join(" "));
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, index: usize, route: &mut Vec<usize>) {
    visited[index] = true;
    route.push(index);

    for g in &graph[index] {
        if !visited[*g] {
            dfs(graph, visited, *g, route);
            route.push(index);
        }
    }
}
