use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut v = vec![0; n + 1];
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        v[a] += 1;
        v[b] += 1;
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    if v.iter().any(|x| *x > 2) {
        println!("No");
        return;
    }
    let mut flag = true;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] && !dfs(&graph, &mut visited, n, i) {
            flag = false;
        }
        if !flag {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, prev: usize, index: usize) -> bool {
    visited[index] = true;
    for &g in &graph[index] {
        if g == prev {
            continue;
        }
        if visited[g] {
            return false;
        }
        if !dfs(graph, visited, index, g) {
            return false;
        }
    }
    return true;
}
