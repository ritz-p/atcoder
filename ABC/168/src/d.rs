use std::collections::VecDeque;

use itertools::Itertools;
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
        graph[b - 1].push(a - 1);
    }
    let mut stack = VecDeque::new();
    let mut visited = vec![false; n];
    let mut route = vec![usize::MAX; n];
    stack.push_back(0);
    while let Some(v) = stack.pop_front() {
        for g in &graph[v] {
            if !visited[*g] {
                route[*g] = v;
                stack.push_back(*g);
                visited[*g] = true;
            }
        }
    }
    if !route.iter().skip(1).all(|v| *v != usize::MAX) {
        println!("No");
    } else {
        println!("Yes");
        println!("{}", route.iter().skip(1).map(|v| v + 1).join("\n"));
    }
}
