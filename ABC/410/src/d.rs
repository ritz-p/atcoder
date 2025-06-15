use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(usize, usize, usize); m]
    }

    let mut graph = vec![vec![]; n];
    for (a, b, w) in &abw {
        graph[a - 1].push((b - 1, w));
    }

    let mut visited = vec![vec![false; 1024]; n];

    let mut queue = VecDeque::new();

    visited[0][0] = true;
    queue.push_back((0, 0));

    while let Some((current, xor)) = queue.pop_front() {
        for &(goal, weight) in &graph[current] {
            let next = xor ^ weight;

            if !visited[goal][next] {
                visited[goal][next] = true;
                queue.push_back((goal, next));
            }
        }
    }

    for (index, res) in visited[n - 1].iter().enumerate() {
        if *res == true {
            println!("{}", index);
            return;
        }
    }
    println!("{}", -1);
}
