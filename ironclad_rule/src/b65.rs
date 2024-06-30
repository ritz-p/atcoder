use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize,usize);n-1],
    };
    let mut graph = vec![vec![]; n];
    let mut res = vec![0; n];
    let mut visited = vec![false; n];

    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    dfs(&graph, &mut res, &mut visited, t - 1);

    println!("{}", res.iter().join(" "));
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    res: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    index: usize,
) -> usize {
    visited[index] = true;
    for g in &graph[index] {
        if !visited[*g] {
            res[index] = res[index].max(dfs(graph, res, visited, *g) + 1);
        }
    }

    return res[index];
}
