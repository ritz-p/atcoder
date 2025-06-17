use std::collections::VecDeque;

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

    // w の最大値が 1024 より
    let mut visited = vec![vec![false; 1024]; n];

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some((current, xor)) = queue.pop_front() {
        for (g, w) in &graph[current] {
            let next = xor ^ w;
            if !visited[*g][next] {
                visited[*g][next] = true;
                queue.push_back((*g, next));
            }
        }
    }

    // n に到達していてかつ一番小さいものを求める
    for (i, res) in visited[n - 1].iter().enumerate() {
        if *res {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
