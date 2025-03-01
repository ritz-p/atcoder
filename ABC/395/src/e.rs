use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        uv: [(usize,usize);m]
    };
    let mut graph = vec![vec![]; n];
    for (u, v) in &uv {
        graph[u - 1].push((v, true));
        graph[v - 1].push((u, false));
    }
}

fn dfs(index: usize, arrow: bool, goal: usize, graph: &Vec<Vec<(usize, bool)>>, x: usize) {
    if index == goal {
        return;
    }
    for (g, ar) in &graph[index] {
        if *ar == arrow {
        } else {
        }
    }
}
