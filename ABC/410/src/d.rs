use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(usize,usize,usize);m]
    };

    let mut graph = vec![vec![]; n];

    for (a, b, w) in abw {
        graph[a - 1].push((b - 1, w));
    }

    let mut dist = vec![usize::MAX; n];
    let mut visited = vec![false; n];
    let mut basis = vec![];

    dist[0] = 0;
    dfs(0, usize::MAX, &graph, &mut visited, &mut dist, &mut basis);

    if dist[n - 1] != usize::MAX {
        let mut res = dist[n - 1];
        for &b in &basis {
            if res ^ b < res {
                res ^= b;
            }
        }
        println!("{}", res);
    } else {
        println!("-1");
    }
}

fn dfs(
    v: usize,
    start: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    visited: &mut Vec<bool>,
    dist: &mut Vec<usize>,
    basis: &mut Vec<usize>,
) {
    visited[v] = true;
    for &(goal, w) in &graph[v] {
        let next = dist[v] ^ w;
        if !visited[goal] {
            dist[goal] = next;
            dfs(goal, v, graph, visited, dist, basis);
        } else if start != v {
            let cycle = dist[goal] ^ next;
            xor(basis, cycle);
        }
    }
}

fn xor(basis: &mut Vec<usize>, mut x: usize) {
    for &b in basis.iter() {
        if (x ^ b) < x {
            x ^= b;
        }
    }
    if x != 0 {
        basis.push(x);
    }
}
