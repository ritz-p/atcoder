use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut ans = n * (n - 1) / 2 - m;
    let mut color = vec![None; n];
    for i in 0..n {
        if color[i].is_some() {
            continue;
        }
        color[i] = Some(true);
        let mut b = 0;
        let mut w = 0;
        let mut que = VecDeque::new();
        que.push_back(i);
        while let Some(u) = que.pop_front() {
            if color[u] == Some(true) {
                b += 1;
            } else {
                w += 1;
            }
            for &v in &graph[u] {
                if color[v] == color[u] {
                    println!("0");
                    return;
                }
                if color[v].is_some() {
                    continue;
                }
                color[v] = color[u].and_then(|flg| Some(!flg));
                que.push_back(v);
            }
        }
        ans -= b * (b - 1) / 2;
        ans -= w * (w - 1) / 2;
    }
    println!("{}", ans);
}