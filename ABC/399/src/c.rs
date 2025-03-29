use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    };
    if m == 0 {
        println!("0");
        return;
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in edges {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut s = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            s += 1;
            let mut que = VecDeque::new();
            que.push_back(i);
            while !que.is_empty() {
                let q = que.pop_front().unwrap();
                for g in &graph[q] {
                    if !visited[*g] {
                        visited[*g] = true;
                        que.push_back(*g);
                    }
                }
            }
        }
    }
    println!("{}", m - n + s);
}
