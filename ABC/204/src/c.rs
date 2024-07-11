use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut graph = vec![vec![]; n];
    for (a, b) in &ab {
        graph[a - 1].push(b - 1);
    }
    let mut res = 0;
    for i in 0..n {
        let mut visited = vec![false; n];
        dfs(&graph, i, &mut visited);
        let c = visited.iter().filter(|e| **e == true).count();
        res += c;
    }
    println!("{}", res);
}

fn dfs(graph: &Vec<Vec<usize>>, index: usize, visited: &mut Vec<bool>) {
    if visited[index] {
        return;
    }
    visited[index] = true;
    for e in &graph[index] {
        dfs(graph, *e, visited);
    }
}
